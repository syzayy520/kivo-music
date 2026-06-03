# Kivo Audio Quality Roadmap

This document defines the future audio quality roadmap for Kivo Music as a mature local hi-fi music player.

Execution authority note:

- This document is read-only planning. It does not change any code, capability, or Settings behavior.
- For strict execution format and conflict precedence, see `docs/AUDIO_CORE_EXECUTION_SPEC.md`.
- For current implementation order, see `docs/audio-core-implementation-checklist.md`.

## Scope and prohibitions

This document is strictly informational. It must not:

- Add or modify any Rust source file.
- Add or modify any TypeScript source file.
- Add or modify any CSS file.
- Add or modify any Tauri command or frontend API.
- Add or modify any Settings UI or Settings schema.
- Change any runtime capability status.
- Introduce any new dependency.

## Tier overview

```text
Tier     | Goal                                   | Unlock condition
---------|----------------------------------------|------------------
P1       | Basic playback closure                 | P0 decode + output runtime loop closed
P2       | Audio quality foundation               | P1 stable, WASAPI shared mode working
P3       | Professional and niche features        | P2 stable, user demand confirmed
Never    | Explicitly excluded                    | Architectural or product-policy ban
```

---

## P1: Basic playback closure

P1 features enable the player to feel like a real music player. Without these, the product cannot be used for casual listening.

### P1-1: Gapless playback

Goal: Eliminate silence gaps between consecutive tracks on the same album.

Why it matters: Many albums (classical, live, DJ mixes, concept albums) are designed as continuous experiences. A gap between tracks breaks artistic intent.

Architecture principle:

- Gapless requires decoder pre-open: before the current track ends, the next track's decoder must be opened and the first frame decoded.
- The output sink must not be closed between tracks. Only the decoder source changes.
- The NativePipeline must support a "next decoder warm" state where the current decoder is draining while the next decoder is ready.
- The transition point is determined by the current decoder returning `EndOfStream`. At that moment, the pipeline swaps the decoder reference and continues draining from the new decoder's buffer.
- Any crossfade (P3) is layered on top of this mechanism, not a replacement for it.

Experience principle:

- Zero audible gap between album tracks when gapless is enabled.
- Gapless should be the default behavior for albums. A user setting may override this to force gaps.
- The UI should not show any loading indicator between gapless transitions.

Settings planning:

- `Settings > Playback > Gapless`: On / Off (default: On).

### P1-2: Multi-format decode

Goal: Support common audio formats beyond WAV: MP3, FLAC, AAC/M4A, OGG Vorbis, Opus, WMA.

Why it matters: A music player that only plays WAV files is not usable. The majority of user libraries contain MP3 and FLAC.

Architecture principle:

- The `AudioDecoder` trait remains unchanged. Each format gets its own decoder implementation behind `decoders/`.
- The decoder factory (`create_decoder_for_path()`) maps file extensions and probe results to the correct decoder.
- All decoders normalize output to `f32` samples in `[-1.0, 1.0]`, matching the existing `DecodedAudioFrame` contract.
- Format selection is driven by `MediaProbeService` results (codec, container) rather than file extension alone, because extension can be misleading.
- Each decoder implementation is isolated in its own file, respecting the single-responsibility and file-size rules.

Experience principle:

- Playback should start within 200ms of load for local files, regardless of format.
- Seek within a track must work for all supported formats.
- The UI must not show format-specific behavior to the user. All formats should feel identical.

Settings planning:

- No user-facing settings needed. Format support is automatic.

### P1-3: WASAPI shared mode output

Goal: Implement real audio output through WASAPI in shared mode on Windows.

Why it matters: This is the first step to producing audible sound from the native engine. Shared mode is the simplest and most compatible output path.

Architecture principle:

- `WasapiOutputSink` implements `OutputSink` with real WASAPI calls.
- The output sink opens an `IAudioClient` in shared mode, initializes a render client, and submits audio buffers via `IAudioRenderClient`.
- The output sink must handle buffer underrun gracefully by inserting silence (zero-filled frames) rather than crashing.
- Sample rate conversion for shared mode is handled by Windows Audio Engine; Kivo does not need to resample.
- The output sink reports real latency through `OutputRuntimeStatus.latency`.
- Device enumeration uses `IMMDeviceEnumerator` to list available output devices.

Experience principle:

- Audio must play without stuttering on typical Windows hardware.
- If the default output device changes (e.g., headphone plugged in), the player should handle this gracefully (see P1-6 Hotplug).

Settings planning:

- `Settings > Audio > Output Device`: System Default / specific device (default: System Default).
- `Settings > Audio > Output Mode`: Shared (default). Exclusive mode is P2.

### P1-4: Seek

Goal: Implement precise seeking within a track for all supported formats.

Why it matters: Users expect to jump to any point in a track instantly. Without seek, the player is a radio stream, not a music player.

Architecture principle:

- Seek is implemented at the decoder level via `AudioDecoder::seek(position_ms)`.
- After seeking, the pipeline resets its buffer (flush) and updates the clock to the new position.
- Seek accuracy depends on the codec container. For MP3/Vorbis, seek is sample-accurate after the decoder handles its own seeking logic. For FLAC, seek is frame-accurate.
- The pipeline must emit a `SeekComplete` event after the seek operation succeeds, so the UI can update the progress bar.
- Seek during pause must work: the decoder seeks, the buffer is flushed, but the output remains paused.

Experience principle:

- Seek response time must be under 100ms for local files.
- The progress bar must update immediately to the seek target, even before the first decoded frame at the new position arrives.
- No audible pop or click at the seek boundary.

Settings planning:

- No user-facing settings needed. Seek behavior is automatic.

### P1-5: Error recovery

Goal: Handle playback errors gracefully without crashing or leaving the player in an unrecoverable state.

Why it matters: Files may be corrupted, disks may disconnect, codecs may fail mid-stream. The player must degrade gracefully.

Architecture principle:

- The `NativePipeline` must have a `Failed` state that is recoverable: the user can skip to the next track or stop.
- Decoder errors (corrupt frame, unexpected EOF) must be caught and reported as typed `PlaybackError` variants, not panics.
- Output errors (device lost, buffer underrun) must trigger a recovery path: attempt to reopen the output sink with the same or default device.
- The `PlaybackManager` must implement a skip-on-error policy: if a track fails to load or decode, automatically advance to the next track in the queue with a user-visible notification.
- Error events must include: error category, affected track, whether recovery was attempted, and whether recovery succeeded.

Experience principle:

- A single corrupt file must not stop the entire queue. The player skips it and continues.
- Error notifications must be non-intrusive (toast or status bar), not modal dialogs.
- The user must be able to retry a failed track manually.

Settings planning:

- `Settings > Playback > On Error`: Skip to next / Stop playback (default: Skip to next).

### P1-6: Output device hotplug

Goal: Handle audio device changes during playback (e.g., headphone plug/unplug, USB audio device connect/disconnect).

Why it matters: Real-world usage involves frequent device changes. A player that freezes or crashes on device change is not acceptable.

Architecture principle:

- The output sink registers for device change notifications via `IMMNotificationClient`.
- On device removal: the output sink pauses, attempts to switch to the default device, and resumes if successful.
- On device addition: the output sink does not automatically switch. The user may switch manually via Settings.
- The `PlaybackManager` emits a `BackendSwitched` or `DeviceChanged` event so the UI can update the device selector.
- Device switch must preserve playback position. The decoder does not re-seek; only the output sink is re-initialized.

Experience principle:

- Playback should resume within 1 second of a device switch.
- No crash or hang on device removal.
- The UI must show the currently active output device.

Settings planning:

- `Settings > Audio > On Device Disconnect`: Switch to default / Pause (default: Switch to default).

---

## P2: Audio quality foundation

P2 features transform Kivo from "it plays music" to "it plays music well". These are the features that distinguish a hi-fi player from a casual player.

### P2-1: ReplayGain

Goal: Normalize perceived loudness across tracks so the user does not need to manually adjust volume between songs.

Why it matters: Tracks from different albums, eras, and mastering styles have wildly different loudness. Without normalization, quiet tracks sound too quiet and loud tracks are jarring.

Architecture principle:

- ReplayGain metadata is read by `MediaProbeService` during probing: track gain, album gain, track peak, album peak.
- The gain value is stored in `AudioMetadata` alongside codec and sample rate information.
- Gain is applied in the audio processing chain as a sample-level multiplication: `output_sample = input_sample * gain_linear`.
- The processing chain order is: decoded samples -> replay gain -> clipping protection -> user volume -> output sink.
- Clipping protection (P2-2) works in tandem: if the gain would cause clipping, the gain is reduced.
- ReplayGain uses the standard reference loudness of 89 dB SPL.

Experience principle:

- Loudness normalization must be perceptually seamless. The user should not notice volume jumps between tracks.
- The user must be able to choose between track gain and album gain modes.
- Album gain preserves the internal dynamics of an album. Track gain normalizes each track independently.

Settings planning:

- `Settings > Audio > Loudness Normalization`: On / Off (default: On).
- `Settings > Audio > Normalization Mode`: Track / Album (default: Album).
- `Settings > Audio > Pre-amp`: -12 dB to +12 dB slider (default: 0 dB).

### P2-2: Clipping protection

Goal: Prevent audio clipping when gain adjustments or loud source material would cause sample values to exceed the valid range.

Why it matters: Clipping produces harsh distortion that damages the listening experience and can damage speakers at high volumes.

Architecture principle:

- Clipping protection is applied after replay gain in the processing chain.
- Two strategies are available: hard limiting (clamp to [-1.0, 1.0]) and soft limiting (compress peaks with a smooth curve).
- The initial implementation uses hard limiting for simplicity. Soft limiting can be added later.
- Peak detection uses the absolute maximum sample value across all channels in a frame.
- If peak exceeds 1.0 after gain, the gain is reduced for that frame to prevent clipping.
- The clipping protection must report when it activates (for diagnostics), but this is not visible to the user.

Experience principle:

- No audible clipping distortion under any gain configuration.
- Clipping protection must not introduce audible pumping or compression artifacts in normal use.

Settings planning:

- No user-facing settings. Clipping protection is always active when ReplayGain is enabled.

### P2-3: Sample rate switching

Goal: Automatically switch the output device's sample rate to match the source material's sample rate.

Why it matters: If the source is 96kHz but the output is locked to 44.1kHz, the Windows audio engine resamples, which can introduce artifacts. Matching the sample rate avoids unnecessary resampling.

Architecture principle:

- Before opening the output sink for a new track, compare the source sample rate with the current device sample rate.
- If they differ and the device supports the source rate, reinitialize the `IAudioClient` with the matching sample rate.
- If the device does not support the source rate, fall back to the device's default rate (Windows will resample).
- Sample rate switching requires closing and reopening the output sink. This must be coordinated with gapless playback to avoid introducing a gap.
- The `OutputSettings` struct gains a `preferred_sample_rate: Option<u32>` field.

Experience principle:

- Sample rate switching must be inaudible to the user. No pop, click, or gap.
- The UI may display the current output sample rate in a non-intrusive location (e.g., status bar or Settings).

Settings planning:

- `Settings > Audio > Sample Rate Matching`: Automatic / Manual (default: Automatic).
- `Settings > Audio > Manual Sample Rate`: 44100 / 48000 / 88200 / 96000 / 176400 / 192000 (only shown when Manual is selected).

### P2-4: Bit-perfect output

Goal: Deliver audio samples to the output device without any modification by the Windows audio engine (no resampling, no mixing, no volume adjustment).

Why it matters: Audiophile users want the purest possible signal path. Bit-perfect output ensures the samples reach the DAC exactly as decoded.

Architecture principle:

- Bit-perfect requires WASAPI exclusive mode: `IAudioClient::Initialize` with `AUDCLNT_SHAREMODE_EXCLUSIVE`.
- In exclusive mode, the output sink has exclusive access to the device. No other applications can play audio.
- The output format must match the source format exactly: sample rate, bit depth, channel count.
- Bit-perfect disables all software processing: no replay gain, no volume control, no clipping protection. The processing chain is bypassed.
- If the device does not support the exact format, bit-perfect mode must report failure and fall back to shared mode.
- The `OutputSettings` struct already has `bit_perfect_mode: bool`.

Experience principle:

- Bit-perfect mode is an opt-in power user feature. It is not the default.
- When bit-perfect is active, the UI volume slider must be disabled or hidden, because volume is controlled by the hardware or not at all.
- Entering and exiting bit-perfect mode must not crash or leave the player in a broken state.

Settings planning:

- `Settings > Audio > Output Mode`: Shared / Exclusive (default: Shared).
- `Settings > Audio > Bit-perfect`: On / Off (only shown when Exclusive is selected, default: Off).

---

## P3: Professional and niche features

P3 features serve power users, audiophiles, and niche use cases. These are implemented only after P1 and P2 are stable.

### P3-1: Crossfade

Goal: Blend the end of one track with the beginning of the next for seamless transitions.

Why it matters: Some users prefer continuous music without silence, even between unrelated tracks. Crossfade provides a radio-like experience.

Architecture principle:

- Crossfade is layered on top of gapless. The pipeline maintains two decoders simultaneously during the overlap period.
- The overlap duration is configurable (1-12 seconds, default 5 seconds).
- During overlap, samples from both decoders are mixed with a linear or equal-power crossfade curve.
- Crossfade is incompatible with bit-perfect mode (requires sample mixing).

Experience principle:

- Crossfade must be smooth and not introduce clicks or volume spikes.
- The user should be able to enable/disable and configure the duration.

Settings planning:

- `Settings > Playback > Crossfade`: On / Off (default: Off).
- `Settings > Playback > Crossfade Duration`: 1-12 seconds slider (only shown when On, default: 5s).

### P3-2: High-quality resampler

Goal: Provide a high-quality software resampler for cases where the output device does not support the source sample rate.

Why it matters: Windows' built-in resampler quality varies. A high-quality resampler (e.g., libsamplerate, rubato) ensures consistent quality across devices.

Architecture principle:

- The resampler sits in the audio processing chain between decoded samples and the output sink.
- Resampling is only activated when the source sample rate differs from the output sample rate and the user has not opted for bit-perfect mode.
- The resampler must support arbitrary ratio conversion (e.g., 44100 -> 48000).
- Quality settings trade off latency and CPU usage against audio quality.

Experience principle:

- Resampling must be inaudible at the highest quality setting.
- CPU usage must remain reasonable (<5% on modern hardware).

Settings planning:

- `Settings > Audio > Resampler Quality`: Low / Medium / High (default: High).
- Only shown when Sample Rate Matching is Manual or when the device does not support the source rate.

### P3-3: Dithering

Goal: Add shaped noise when reducing bit depth (e.g., 24-bit source to 16-bit output) to reduce quantization distortion.

Why it matters: Without dithering, bit-depth reduction produces correlated quantization error that is perceptually harsh. Proper dithering makes the error inaudible.

Architecture principle:

- Dithering is applied in the audio processing chain when the output bit depth is lower than the source bit depth.
- Three dithering algorithms: none, rectangular (TPDF), and noise-shaped (e.g., Shibata).
- Dithering is only relevant in bit-perfect-adjacent scenarios or when the output device forces a lower bit depth.

Experience principle:

- Dithering must be imperceptible. The user should not hear noise.
- Dithering is a power-user feature. Default is off unless bit depth reduction is detected.

Settings planning:

- `Settings > Audio > Dithering`: Off / TPDF / Noise-shaped (default: Off).

### P3-4: Equalizer and DSP

Goal: Provide a parametric equalizer and optional DSP effects for users who want to customize their listening experience.

Why it matters: Different headphones and speakers have different frequency responses. An EQ allows users to compensate.

Architecture principle:

- EQ is a sample-level processing stage in the audio processing chain, after replay gain and before clipping protection.
- The EQ engine operates on f32 samples in the frequency domain using FFT or in the time domain using biquad filters.
- A parametric EQ with at least 10 bands is the target. Each band has: frequency, gain (dB), Q factor, and filter type (peak, low-shelf, high-shelf).
- DSP effects (reverb, surround, loudness) are optional extensions built on the same processing chain.

Experience principle:

- EQ changes must be applied in real-time without audible glitches.
- Preset EQ profiles (e.g., "Flat", "Bass Boost", "Vocal") should be available.

Settings planning:

- `Settings > Audio > Equalizer`: Off / On (default: Off).
- `Settings > Audio > EQ Presets`: Flat, Bass Boost, Vocal, Custom (only shown when On).
- `Settings > Audio > EQ Band N`: Frequency, Gain, Q (only shown when Custom).

### P3-5: CUE sheet support

Goal: Parse CUE sheets to split a single audio file into individual tracks with precise boundaries.

Why it matters: Many users have albums stored as a single FLAC/WAV file with a CUE sheet. Without CUE support, these albums cannot be navigated as individual tracks.

Architecture principle:

- CUE sheet parsing is a metadata-level concern, not a decoder-level concern.
- The `MediaProbeService` parses the CUE sheet and produces a list of virtual tracks with start/end timestamps.
- During playback, the decoder opens the underlying audio file but the pipeline constrains the decode window to the virtual track's boundaries.
- Gapless playback between CUE tracks uses the same mechanism as album gapless.

Experience principle:

- CUE tracks must appear as separate entries in the queue and library.
- Seek within a CUE track must be constrained to the track's boundaries.

Settings planning:

- No user-facing settings. CUE support is automatic when a .cue file is detected alongside an audio file.

### P3-6: DSD playback

Goal: Support DSD (Direct Stream Digital) audio files (.dsf, .dff).

Why it matters: DSD is a niche but important format for audiophiles. Supporting DSD differentiates Kivo from casual players.

Architecture principle:

- DSD decoding converts DSD bitstream to PCM (DoP or direct conversion) before passing to the output sink.
- DSD64 (2.8 MHz), DSD128 (5.6 MHz), and DSD256 (11.2 MHz) are the target rates.
- The output sink must support the high sample rates required for DSD-over-PCM.
- If the output device does not support the required sample rate, DSD is downsampled to PCM at the highest supported rate.

Experience principle:

- DSD playback must not degrade quality for non-DSD users.
- DSD files must be clearly labeled in the UI (codec badge).

Settings planning:

- `Settings > Audio > DSD Playback`: PCM conversion / DoP (default: PCM conversion).

---

## Do not prioritize: Explicitly excluded features

These features are intentionally excluded from Kivo's roadmap due to architectural, product-policy, or philosophical reasons.

### Dolby Atmos / Dolby Digital

Why excluded:

- Dolby Atmos requires licensed SDK integration and specific hardware support.
- Kivo is a local music player, not a home theater system. The complexity and licensing cost do not justify the benefit.
- Atmos content is primarily distributed through streaming services, not local music libraries.

Decision: Do not implement. If future demand is overwhelming, reassess as a dedicated ticket with explicit scope.

### FFmpeg or mpv as primary playback engine

Why excluded:

- Kivo's architecture is native-first. FFmpeg and mpv are compatibility layers, not the primary engine.
- Making FFmpeg or mpv the primary engine would undermine the native decode + output pipeline that is the core of Kivo's architecture.
- FFmpeg is used only for media probing and auxiliary analysis. mpv is only a compatibility backend.

Decision: The native engine remains the primary target. mpv and FFmpeg retain their current scoped roles.

### Online resource search, torrent, magnet, download

Why excluded:

- Kivo is a local-first music player. It does not provide content discovery, downloading, or streaming from external sources.
- Introducing torrent, magnet, or download semantics violates the product boundary defined in `docs/QUALITY_GATES.md` and `docs/ARCHITECTURE.md`.
- Cloud/NAS/WebDAV integrations (if any) must mean user-owned library sources, not public content discovery.

Decision: Never implement. This is a hard product-policy ban, not a prioritization decision.

### Cloud streaming service integration

Why excluded:

- Integrating with Spotify, Apple Music, Tidal, or similar services would transform Kivo from a local player into a streaming client.
- This conflicts with the local-first, self-owned library philosophy.

Decision: Do not implement. Kivo plays the user's own files.

---

## Architecture: Audio processing chain

All audio processing follows a strict, linear chain. Each stage is optional and can be bypassed. The chain order is defined once and must not be rearranged.

```text
[1] Decoder
    |
    v
[2] Replay Gain (P2-1)
    |
    v
[3] Clipping Protection (P2-2)
    |
    v
[4] Resampler (P3-2, only when needed)
    |
    v
[5] Dithering (P3-3, only when reducing bit depth)
    |
    v
[6] EQ / DSP (P3-4, only when enabled)
    |
    v
[7] User Volume (level + muted)
    |
    v
[8] Output Sink (WASAPI / NullOutput / future backends)
```

### Chain rules

1. Each stage receives `&[f32]` samples and produces `&[f32]` samples (in-place mutation allowed).
2. Stages are composed at the pipeline level, not inside the decoder or output sink.
3. In bit-perfect mode, stages 2-7 are bypassed. The decoder output goes directly to the output sink.
4. Adding a new processing stage requires a dedicated ticket and must not modify existing stages.
5. The chain is evaluated once per audio frame (typically 1024-4096 samples).

### Current state (code-verified)

The processing chain does not exist yet. The current pipeline passes decoded samples directly to the output sink with no intermediate processing:

```text
Decoder -> Buffer -> Output Sink
```

This is correct for P0. The chain will be built incrementally as P1 and P2 features are implemented.

---

## Experience principles

These principles guide every audio quality decision.

### 1. Silence is the enemy

- No audible gaps between tracks (unless the user explicitly wants gaps).
- No silence at the start of playback. First audio within 200ms.
- No pops, clicks, or glitches at any transition.

### 2. The user should not think about audio

- Default settings must produce excellent audio quality for the majority of users.
- Advanced settings exist for power users but must not be required.
- The player must handle device changes, format differences, and error conditions transparently.

### 3. Honest capability

- If a feature is not implemented, it must not appear to work.
- If a feature degrades quality (e.g., resampling), the UI may indicate this.
- Bit-perfect mode must actually be bit-perfect. No false claims.

### 4. The processing chain is the source of truth

- All audio transformations happen in the chain, not scattered across decoders, output sinks, or UI code.
- The chain is observable: each stage can report its state for diagnostics.
- The chain is testable: each stage can be unit-tested with synthetic audio data.

### 5. Platform-specific code stays behind boundaries

- WASAPI code lives in `output_wasapi/`.
- Future platform backends (CoreAudio, ALSA, PipeWire) get their own directories.
- The processing chain and pipeline are platform-agnostic.

---

## Settings planning

Settings are organized by category. Each setting has a default value, allowed values, and a description.

### Audio settings

| Setting | Category | Default | Values | Description |
|---------|----------|---------|--------|-------------|
| Output Device | Audio | System Default | System Default, [device list] | Select the audio output device |
| Output Mode | Audio | Shared | Shared, Exclusive | WASAPI output mode (P2) |
| Bit-perfect | Audio | Off | On, Off | Bypass all processing (P2, requires Exclusive) |
| Sample Rate Matching | Audio | Automatic | Automatic, Manual | Match output rate to source (P2) |
| Manual Sample Rate | Audio | 44100 | 44100, 48000, 88200, 96000, 176400, 192000 | Fixed output sample rate (P2) |
| DSD Playback | Audio | PCM conversion | PCM conversion, DoP | DSD decode strategy (P3) |
| Resampler Quality | Audio | High | Low, Medium, High | Software resampler quality (P3) |
| Dithering | Audio | Off | Off, TPDF, Noise-shaped | Dithering algorithm (P3) |

### Loudness settings

| Setting | Category | Default | Values | Description |
|---------|----------|---------|--------|-------------|
| Loudness Normalization | Loudness | On | On, Off | Enable ReplayGain normalization (P2) |
| Normalization Mode | Loudness | Album | Track, Album | Use track or album gain (P2) |
| Pre-amp | Loudness | 0 dB | -12 to +12 dB | Gain adjustment before output (P2) |

### Playback settings

| Setting | Category | Default | Values | Description |
|---------|----------|---------|--------|-------------|
| Gapless | Playback | On | On, Off | Enable gapless track transitions (P1) |
| On Error | Playback | Skip to next | Skip to next, Stop playback | Behavior when a track fails (P1) |
| On Device Disconnect | Playback | Switch to default | Switch to default, Pause | Behavior on device removal (P1) |
| Crossfade | Playback | Off | On, Off | Enable crossfade between tracks (P3) |
| Crossfade Duration | Playback | 5 seconds | 1-12 seconds | Crossfade overlap duration (P3) |

### Equalizer settings

| Setting | Category | Default | Values | Description |
|---------|----------|---------|--------|-------------|
| Equalizer | EQ | Off | On, Off | Enable parametric equalizer (P3) |
| EQ Preset | EQ | Flat | Flat, Bass Boost, Vocal, Custom | EQ preset profile (P3) |
| EQ Band N (1-10) | EQ | Flat | Freq, Gain, Q | Per-band EQ parameters (P3, Custom only) |

### Settings implementation notes

1. Settings are stored in a user-local configuration file (JSON or TOML).
2. Settings changes take effect immediately where possible (volume, EQ). Some changes require output sink restart (output mode, sample rate).
3. Settings are validated on load. Invalid values fall back to defaults.
4. Settings UI is a separate frontend concern and is not detailed in this document.

---

## Implementation order guidance

The features in this roadmap are ordered by dependency and user impact.

```text
P1-3 (WASAPI Shared) -> unlocks all audio output
  |
  +-> P1-1 (Gapless)
  +-> P1-4 (Seek)
  +-> P1-5 (Error Recovery)
  +-> P1-6 (Hotplug)
  |
  +-> P2-1 (ReplayGain) -> requires sample-level processing chain
        |
        +-> P2-2 (Clipping Protection)
        |
        +-> P2-3 (Sample Rate Switching)
        +-> P2-4 (Bit-perfect)
              |
              +-> P3-1 (Crossfade)
              +-> P3-2 (Resampler)
              +-> P3-3 (Dithering)
              +-> P3-4 (EQ/DSP)
              +-> P3-5 (CUE)
              +-> P3-6 (DSD)
```

P1-2 (Multi-format) is independent and can proceed in parallel with other P1 features.

---

## Change log

| Date | Author | Change |
|------|--------|--------|
| 2026-06-03 | Kivo | Initial roadmap document created (DOC-P1-001) |
