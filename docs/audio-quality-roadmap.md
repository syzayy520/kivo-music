# Kivo Audio Quality Roadmap

## Status

This document is a future roadmap only. It is not implemented yet.

- Must not be exposed as current capability.
- Does not change `PlaybackCapabilities`.
- Does not add Settings UI.
- Does not change runtime behavior.
- Does not add any code or dependency.

For current implementation order, see `docs/audio-core-implementation-checklist.md`.

---

## Architecture Boundary

### Strict tree-layered delegation model

Kivo backend must follow these rules permanently:

1. Upper layers manage only the next layer down. No skipping layers.
2. Page layer composes UI only. No playback logic.
3. Command layer is the external entry point only. No decode/output details.
4. Manager layer orchestrates state only. No DSP/decode/output details.
5. Engine layer exposes unified playback capability only. Must not bypass pipeline.
6. Pipeline layer chains decode/process/output only. No UI/settings/device policy.
7. Decoder layer decodes only. No user volume, ReplayGain, or device output.
8. Audio processing layer owns ReplayGain, clipping protection, and volume pipeline.
9. Output layer outputs only. No loudness policy, queue, or business recovery.
10. One file, one responsibility. No stacking.

### Audio processing chain

All audio processing follows this strict linear order. The order must not be rearranged.

```text
decoded samples
  -> replay gain
  -> clipping protection / limiter
  -> user volume
  -> output sink
```

Rules:

- ReplayGain must not be inside the decoder.
- Clipping protection must not be inside the output sink.
- User volume must not be merged with ReplayGain into one field.
- Output sink only outputs. It does not own loudness policy.
- Decoder only decodes. It does not own user volume policy.
- Manager does not carry DSP details.
- Loudness metadata / analysis / cache must be in a separate layer.

Current state: this chain does not exist yet. The pipeline passes decoded samples directly to the output sink.

---

## P1: Basic playback closure

P1 enables the player to function as a real music player.

| Feature | Goal | Key Architecture Principle |
|---------|------|---------------------------|
| Gapless Playback | Zero audible gap between consecutive album tracks. | Decoder pre-open before track end. Output sink stays open across tracks. Pipeline swaps decoder reference at EndOfStream. |
| Multi-format Decode | Support WAV, FLAC, MP3, AAC/M4A, ALAC, OGG Vorbis, Opus. | Each format gets its own decoder behind `decoders/`. All normalize to `f32`. Format selection driven by probe results, not extension. |
| WASAPI Shared Output | Real audio output through WASAPI shared mode. | `WasapiOutputSink` implements `OutputSink` with real WASAPI calls. Handle buffer underrun gracefully. Report real latency. |
| Seek Accuracy | Precise seek within a track for all formats. | Seek via `AudioDecoder::seek()`. Pipeline flush and clock update. Seekable during pause. Emit `SeekComplete` event. |
| Device Hotplug Recovery | Handle device changes during playback. | Register for device change notifications. Pause on removal, switch to default, resume. Preserve playback position. |
| Playback Error Recovery | Graceful error handling without unrecoverable state. | Recoverable `Failed` state. Skip-on-error policy. Typed error events. Corrupt file must not stop the queue. |

---

## P2: Audio quality foundation

P2 transforms Kivo from "it plays music" to "it plays music well".

| Feature | Goal | Key Architecture Principle |
|---------|------|---------------------------|
| ReplayGain / Loudness Normalization | Normalize perceived loudness across tracks. | Gain metadata read during probe. Applied in processing chain as sample-level multiplication. Track gain and album gain modes. |
| Clipping Protection | Prevent clipping when gain adjustments would cause overflow. | Applied after ReplayGain in the chain. Peak detection across channels. Reduce gain if peak exceeds 1.0. |
| Volume Pipeline | User volume as a separate processing stage. | Separate from ReplayGain. Applied after clipping protection. Mute as boolean flag. |
| Sample Rate Switching | Match output rate to source material. | Reinitialize `IAudioClient` when rates differ. Coordinate with gapless to avoid gaps. Fall back to device default if unsupported. |
| Bit Depth Negotiation | Match output bit depth to source when possible. | Fall back to device default if unsupported. Coordinate with dithering for depth reduction. |
| Output Buffer / Underrun Diagnostics | Report buffer health and underrun events. | Diagnostic data for output path tuning. Not user-facing. |
| Loudness Metadata / Cache | Store and retrieve ReplayGain values efficiently. | Read during probe. Cache for fast access. Separate from playback control layer. |
| Bit-perfect PCM Planning | Bypass all processing for pure signal path. | WASAPI exclusive mode. Output format matches source exactly. Disable all processing stages. Fail gracefully if unsupported. |

---

## P3: Professional and niche features

P3 serves power users and niche use cases. Only after P1 and P2 are stable.

| Feature | Direction |
|---------|-----------|
| Crossfade | Blend end of one track with start of next. Configurable overlap. Incompatible with bit-perfect. |
| High Quality Resampler | Software resampler for rate mismatch. Activated only when needed. Quality vs CPU tradeoff. |
| Dithering | Shaped noise for bit-depth reduction. TPDF and noise-shaped options. Power-user feature. |
| EQ / DSP Chain | Parametric equalizer. Preset profiles. Sample-level processing stage in the chain. |
| CUE Sheet | Parse CUE to split single-file albums into virtual tracks. Metadata-level concern. |
| APE / DSD | Niche format support. DSD converted to PCM before output. Does not degrade non-DSD playback. |
| Advanced Diagnostics | Output path diagnostics. Processing chain observability. Per-stage state reporting. |

---

## Future Settings Exposure Candidates

These are future categories only. They are not implemented. They must not appear in UI before backend support and `PlaybackCapabilities` are truthful.

| Category | Notes |
|----------|-------|
| Output Device | Select audio output device |
| Output Mode | Shared or exclusive |
| Buffer Size | Output buffer tuning |
| Volume Normalization | Enable/disable loudness normalization |
| ReplayGain Preamp | Gain adjustment before output |
| Prevent Clipping | Enable/disable clipping protection |
| Gapless Playback | Enable/disable gapless transitions |
| Crossfade | Enable/disable and overlap duration |
| Preferred Sample Rate / Bit Depth | Manual rate/depth selection |

No specific paths, no default values, no UI layout. Each future setting requires its own ticket and design.

---

## Do Not Prioritize

These are explicitly excluded from Kivo's roadmap.

| Item | Reason |
|------|--------|
| Dolby branding / Atmos rendering | Licensed SDK, hardware dependency, streaming-oriented content. Not local player scope. |
| Fake spatial audio | Kivo does not ship audio features that do not produce real improvement. |
| VST plugin system | Plugin hosting adds complexity and instability. Not justified for a local player. |
| FFmpeg as primary playback core | FFmpeg is for media probing and auxiliary analysis only. Native engine is the primary target. |
| mpv as primary playback core | mpv is a compatibility backend only. Native engine is the primary target. |
| Resource search / download / cloud-drive transfer / torrent / magnet | Hard product-policy ban. Kivo is local-first. No external content discovery. |

---

## Experience Principles

1. **Silence is the enemy**: No audible gaps between tracks. No silence at playback start. No pops, clicks, or glitches at transitions.
2. **The user should not think about audio**: Defaults must produce excellent quality. Advanced settings exist but are not required.
3. **Honest capability**: If a feature is not implemented, it must not appear to work. No false claims.
4. **The processing chain is the source of truth**: All audio transformations happen in the chain. The chain is observable and testable.
5. **Platform-specific code stays behind boundaries**: WASAPI code lives in `output_wasapi/`. Processing chain is platform-agnostic.

---

## Governance Rules

1. Do not change `PlaybackCapabilities` before the feature is runtime-closed and tested.
2. Do not add Settings before backend support exists and capabilities are truthful.
3. Do not claim a feature is supported when it is not.
4. Roadmap is not implementation status. Every capability requires a separate ticket, design, and verification.
5. Each future feature must be scoped to its own ticket with allowed/forbidden files, gates, and delivery report.
6. Documentation must not imply urgency or immediate execution.
7. Code is split by responsibility. Documentation is organized by topic. Roadmap remains a single coherent document.

---

## Implementation dependency order

```text
P1-3 (WASAPI Shared) -> unlocks all audio output
  |
  +-> P1-1 (Gapless)
  +-> P1-4 (Seek)
  +-> P1-5 (Error Recovery)
  +-> P1-6 (Hotplug)
  |
  +-> P2-1 (ReplayGain) -> requires processing chain
        |
        +-> P2-2 (Clipping Protection)
        +-> P2-3 (Sample Rate Switching)
        +-> P2-4 (Bit-perfect)
              |
              +-> P3 features (after P2 stable)
```

P1-2 (Multi-format) is independent and can proceed in parallel.

---

## Change log

| Date | Author | Change |
|------|--------|--------|
| 2026-06-03 | Kivo | Initial roadmap (DOC-P1-001) |
| 2026-06-03 | Kivo | Compress and restructure (DOC-P1-001A) |
