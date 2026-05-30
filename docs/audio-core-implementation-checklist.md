# Kivo Audio Core Implementation Checklist

This document is the source of truth for Kivo's playback-core implementation order and module boundaries.

## Non-negotiable principles

1. Kivo owns the playback system.
2. mpv/libmpv is only a playback backend.
3. FFmpeg/ffprobe is only for media probing and auxiliary analysis.
4. WASAPI is the Windows output capability layer, not the playback brain.
5. Tauri commands must call Kivo manager/service layers, never mpv or ffmpeg directly.
6. PlaybackManager must not do media probing.
7. MediaProbeService must not control playback.
8. Do not create two competing playback cores.
9. Do not add fake playback state to make the UI look active.
10. Keep modules small, single-purpose, and final-architecture aligned.

## Responsibility map

### Kivo PlaybackManager

Owns:

- PlaybackState cache
- Current track
- Queue ownership
- Play/pause/stop/seek orchestration
- Next/previous behavior
- Repeat/shuffle policy
- StateChanged / TrackChanged / Progress / Error event boundaries
- Error recovery policy
- Output settings coordination

Does not own:

- Low-level decoding
- Media probing
- Cover extraction
- Direct mpv process/control details
- Direct FFmpeg/ffprobe calls

### PlaybackEngine trait

Represents a playback backend boundary.

Required baseline commands:

- descriptor
- load
- play
- pause
- resume
- stop
- seek
- set_volume
- set_muted
- current_state
- shutdown

Later extensions:

- output device selection
- exclusive output mode
- gapless
- replaygain
- backend health checks

### MpvBackend

Owns only real playback backend work:

- Load local media path
- Decode/play through mpv/libmpv
- Pause/resume/stop
- Seek
- Current position/duration reporting
- Backend-level playback errors
- Backend shutdown

Must not own:

- Queue logic
- Recently played
- Library metadata
- Home recommendations
- Cover cache
- Business state

### MediaProbeService

Owns media analysis and probing:

- File duration
- Codec/container
- Sample rate
- Bit depth
- Channels/channel layout
- Bitrate
- Lossless detection
- Embedded cover metadata
- Audio stream list
- Subtitle/video stream fields reserved for future Kivo video capability
- Dolby/Atmos/HDR/Dolby Vision fields reserved for future video/audio expansion
- Probe error model
- Probe cache boundary

Must not own:

- Playback control
- Queue behavior
- Output device selection

### FFprobeBackend / FFmpegProbeBackend

Owns only probe/analysis implementation details:

- ffprobe invocation or library adapter
- Parsing probe JSON/result
- Mapping to Kivo probe result types
- Handling missing binaries or probe failures

Must not become a second playback backend unless a future ticket explicitly creates a separate PlaybackEngine implementation.

### Output / WASAPI layer

Owns output capability model:

- OutputDevice
- OutputSettings
- OutputMode
- Shared mode
- Exclusive mode
- Bit-perfect intent
- Device fallback policy
- Device disconnect handling
- Future WASAPI backend coordination

Must not own:

- Queue
- Metadata probing
- UI state

## P0 implementation order

### KIVO-AUDIO-CORE-ARCH-P0-001

Goal: establish the final playback-core architecture without fake playback.

Scope:

- PlaybackManager skeleton
- PlaybackEngine trait expansion
- PlaybackCommand boundary if needed
- PlaybackEvent boundary refinement
- PlaybackState/error/timeline/volume/output type hardening
- commands.rs routed through manager/service layer
- MpvBackend boundary only, not real mpv playback yet

Done when:

- No Tauri command calls mpv directly
- No fake playing state is introduced
- The manager is the only playback orchestration entry point
- cargo check passes
- frontend remains unchanged

### KIVO-MEDIA-PROBE-ARCH-P0-001B

Goal: establish media probe architecture.

Scope:

- MediaProbeService
- ProbeBackend trait
- FFprobeBackend boundary
- AudioProbeResult
- VideoProbeResult reserved fields
- Dolby/HDR/Atmos metadata fields reserved
- Probe error model
- Probe cache boundary

Done when:

- PlaybackManager does not probe media directly
- FFmpeg/ffprobe code is isolated behind probe service/backend
- cargo check passes
- frontend remains unchanged

### KIVO-AUDIO-OUTPUT-ARCH-P0-001C

Goal: establish output/WASAPI-ready model.

Scope:

- OutputDevice improvements
- OutputSettings improvements
- OutputMode enum
- Shared/exclusive/bit-perfect intent
- fallback policy types

Done when:

- Output model is ready for WASAPI tickets
- No actual risky device switching is added yet
- cargo check passes

### KIVO-AUDIO-MPV-BACKEND-P0-002

Goal: implement real mpv-backed playback minimum loop.

Scope:

- load local file
- play
- pause/resume
- stop
- current state
- position/duration readback
- backend errors

Done when:

- playback state is backed by real backend behavior
- no fake state
- cargo check passes

### KIVO-MEDIA-FFPROBE-P0-003

Goal: implement real media probing minimum loop.

Scope:

- ffprobe-backed duration
- codec/container
- sample rate
- bit depth
- channels
- bitrate
- lossless detection
- embedded cover metadata boundary

Done when:

- probe results are not inferred from extension alone
- probe errors are typed
- cargo check passes

### KIVO-AUDIO-EVENTS-P0-004

Goal: connect state/progress/event flow.

Scope:

- progress event loop
- state changed events
- track changed events
- playback error events
- frontend subscription boundary if explicitly allowed

Done when:

- events are manager-owned
- backend only reports backend facts
- cargo check passes

### KIVO-AUDIO-QUEUE-P0-005

Goal: implement queue behavior.

Scope:

- append
- play next
- remove
- reorder
- next
- previous
- repeat
- shuffle mapping
- end-of-track advance policy

Done when:

- queue policy is owned by Kivo
- backend does not own queue
- cargo check passes

### KIVO-AUDIO-WASAPI-P0-006

Goal: implement Windows output capability incrementally.

Scope:

- device list
- shared mode
- exclusive mode
- bit-perfect strategy
- fallback behavior
- device disconnect recovery

Done when:

- output failures are recoverable
- settings are explicit
- cargo check passes

## Release and licensing gate

Before bundling mpv, FFmpeg, ffprobe, or related binaries:

- audit license compatibility
- audit distribution method
- audit binary source and integrity
- document user-facing dependency notices if required
- avoid undocumented binary drops

## Current baseline finding

Current playback code is a useful module skeleton but not yet a real playback core:

- playback_get_state returns default state
- mpv module is descriptor-only
- PlaybackEngine trait is incomplete
- Cargo.toml has no real audio/mpv/ffmpeg dependency yet
- FFmpeg/ffprobe layer is not present yet

Implementation must proceed by the P0 order above.
