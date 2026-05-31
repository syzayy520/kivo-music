# Kivo Audio Core Implementation Checklist

This document is the source of truth for Kivo's final audio-core implementation order and module boundaries.

## Final target

Kivo's long-term main playback core is **Kivo Native Engine**.

The architecture must be designed for a self-owned native engine from the start. mpv/libmpv is a compatibility backend, FFmpeg/ffprobe is a media probe layer, and WASAPI is the Windows output layer.

## Execution status baseline (2026-05-31)

This section tracks progress against the original execution queue without changing the core roadmap.

### Completed tickets

1. KIVO-RUSTFMT-P0-000B
2. KIVO-AUDIO-BUFFER-P0-016C-CLOSE
3. KIVO-WAV-DECODER-SPLIT-P0-016E
4. KIVO-DECODER-SESSION-P0-017A
5. KIVO-DECODER-RUNTIME-STATE-P0-017B
6. KIVO-OUTPUT-FRAME-P0-018A
7. KIVO-OUTPUT-SINK-BOUNDARY-P0-018B
8. KIVO-PLAYBACK-WORKER-COMMAND-P0-019A
9. KIVO-PLAYBACK-WORKER-STATE-P0-019B
10. KIVO-NATIVE-PIPELINE-BOUNDARY-P0-020A
11. KIVO-COMMANDS-THIN-WRAPPER-P0-021B
12. KIVO-MANAGER-QUEUE-FACADE-P0-021C
13. KIVO-WORKER-COMMAND-STATE-MAPPING-P0-019C
14. KIVO-DECODER-PIPELINE-GLUE-P0-005A
15. KIVO-OUTPUT-PIPELINE-GLUE-P0-005B
16. KIVO-NATIVE-ENGINE-PIPELINE-WIRING-P0-005C
17. KIVO-AUDIO-EVENT-DISPATCH-P0-008A
18. KIVO-AUDIO-EVENT-BUS-P0-008B
19. KIVO-AUDIO-EVENT-RESULT-MAP-P0-008C
20. KIVO-AUDIO-EVENT-PROGRESS-BRIDGE-P0-008D

### Remaining original audit/check tickets

1. KIVO-VERIFY-RECENT-REMOTE-P0-000 (read-only verification ticket)
2. KIVO-DECODER-REQUEST-P0-016B-CHECK
3. KIVO-WAV-DECODER-AUDIT-P0-016D (read-only audit ticket)
4. KIVO-MANAGER-COMMANDS-AUDIT-P0-021A (read-only audit ticket)

### Next batch execution order (10-ticket acceleration mode)

1. KIVO-PLAN-AUDIT-P0-000F (this documentation baseline ticket)
2. KIVO-DECODER-REQUEST-CHECK-P0-016B-R1
3. KIVO-WAV-DECODER-AUDIT-P0-016D-R1
4. KIVO-MANAGER-COMMANDS-AUDIT-P0-021A-R1
5. KIVO-AUDIO-EVENTS-P0-008E
6. KIVO-AUDIO-EVENTS-P0-008F
7. KIVO-AUDIO-NATIVE-MIN-PLAYBACK-P0-005D
8. KIVO-AUDIO-NATIVE-MIN-PLAYBACK-P0-005E
9. KIVO-AUDIO-NATIVE-MIN-PLAYBACK-P0-005F
10. KIVO-AUDIO-QUEUE-P0-009A

## Non-negotiable principles

1. Kivo Native Engine is the primary long-term playback target.
2. Kivo owns playback orchestration, state, queue, events, and recovery.
3. mpv/libmpv is only a compatibility PlaybackEngine backend.
4. FFmpeg/ffprobe is only for media probing and auxiliary analysis.
5. WASAPI is the Windows output layer, not the playback brain.
6. Tauri commands must call Kivo manager/service layers, not backend implementations directly.
7. PlaybackManager must not do media probing.
8. MediaProbeService must not control playback.
9. Output/WASAPI modules must not own queue, metadata, or UI state.
10. Do not fake playback state.
11. Multiple playback backends must share the same PlaybackEngine boundary.
12. Keep modules small, single-purpose, and final-architecture aligned.

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
- Backend selection between Kivo Native Engine and compatibility backends

Does not own:

- Decoder internals
- Media probing internals
- Cover extraction internals
- Direct mpv control details
- Direct ffprobe calls
- Direct WASAPI device API calls

### PlaybackEngine trait

All playback backends implement the same trait:

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

Backend facts must include:

- backend kind
- capabilities
- health state
- timing facts
- typed error facts

### KivoNativeEngine

The main final backend.

Owns:

- Kivo-owned playback state machine
- Kivo-owned timeline model
- Native decode scheduling boundary
- Native buffer model
- Native output pipeline boundary
- Gapless-ready architecture
- Bit-perfect-ready architecture
- WASAPI-ready output boundary
- Precise seek model
- Error recovery hooks

Early native implementation must be honest:

- It may start as final architecture scaffolding.
- It must not claim capabilities it has not implemented.
- It must return typed unsupported errors until real decode/output pieces exist.

Must not own:

- Library database
- Home recommendations
- Cover cache policy
- Recently played business logic
- UI state

### MpvBackend

Compatibility backend only.

Owns:

- Load local media path through mpv/libmpv
- Play/pause/resume/stop
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
- Kivo Native Engine decisions

### MediaProbeService

Owns media analysis and probing:

- Duration
- Codec/container
- Sample rate
- Bit depth
- Channels/channel layout
- Bitrate
- Lossless detection
- Embedded cover metadata boundary
- Stream list boundaries
- Probe error model
- Probe cache boundary

Must not own:

- Playback control
- Queue behavior
- Output device selection

### FFprobeBackend / FFmpegProbeBackend

Owns only probe implementation details:

- ffprobe adapter
- Probe result parsing
- Mapping to Kivo probe result types
- Missing tool / probe failure handling

Must not become a second playback system unless a future ticket explicitly creates another PlaybackEngine implementation.

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
- WASAPI coordination boundary

Must not own:

- Queue
- Metadata probing
- UI state
- Library state

## P0 implementation order

### KIVO-AUDIO-NATIVE-ARCH-P0-001

Goal: establish the final native-first playback architecture.

Scope:

- PlaybackManager skeleton
- PlaybackEngine trait expansion
- KivoNativeEngine backend boundary
- Backend descriptor/capability model for Native and mpv compatibility backend
- PlaybackEvent boundary refinement
- PlaybackState/error/timeline/volume/output type hardening
- commands.rs routed through manager/service layer

Done when:

- Native engine is represented as the primary target
- Tauri commands do not call backend implementations directly
- No fake playing state is introduced
- PlaybackManager is the only playback orchestration entry point
- mpv is modeled only as compatibility backend
- cargo check passes
- frontend remains unchanged

### KIVO-MEDIA-PROBE-ARCH-P0-001B

Goal: establish media probe architecture.

Scope:

- MediaProbeService
- ProbeBackend trait
- FFprobeBackend boundary
- AudioProbeResult
- VideoProbeResult reserved boundary
- Probe error model
- Probe cache boundary

Done when:

- PlaybackManager does not probe media directly
- FFmpeg/ffprobe code is isolated behind probe service/backend
- Probe results are separate from playback state
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
- native output pipeline boundary

Done when:

- Output model is ready for WASAPI tickets
- KivoNativeEngine can depend on output abstractions later
- No risky device switching is added yet
- cargo check passes

### KIVO-AUDIO-NATIVE-SCAFFOLD-P0-002

Goal: implement honest Kivo Native Engine scaffold.

Scope:

- KivoNativeEngine struct
- Native engine internal state machine
- typed unsupported errors for decode/output actions not yet implemented
- no fake successful playback

Done when:

- Native backend is primary in manager configuration
- State remains honest when playback is not implemented
- cargo check passes

### KIVO-AUDIO-NATIVE-DECODE-P0-003

Goal: establish native decode boundary.

Scope:

- Decoder trait
- AudioFrame model
- sample format model
- stream open boundary
- decode error model
- selected implementation strategy documented before dependency introduction

Done when:

- decode boundary is independent from queue and UI
- no output code mixed into decoder
- cargo check passes

### KIVO-AUDIO-NATIVE-OUTPUT-P0-004

Goal: establish native output boundary.

Scope:

- OutputSink trait
- WASAPI-ready sink model
- buffer write boundary
- underrun/error model
- latency fields

Done when:

- output boundary is independent from decoder and queue
- no media probing mixed into output
- cargo check passes

### KIVO-AUDIO-NATIVE-MINIMUM-PLAYBACK-P0-005

Goal: implement first real Kivo Native Engine playback loop.

Scope:

- load local file through selected decoder
- feed frames into output sink
- play/pause/stop
- honest state updates
- typed errors

Done when:

- playback state is backed by real native behavior
- no fake state
- cargo check passes

### KIVO-AUDIO-MPV-COMPAT-P0-006

Goal: implement mpv-backed compatibility playback.

Scope:

- load local file
- play
- pause/resume
- stop
- current state
- position/duration readback
- backend errors

Done when:

- mpv is selectable as compatibility backend only
- mpv does not own queue or business state
- cargo check passes

### KIVO-MEDIA-FFPROBE-P0-007

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

### KIVO-AUDIO-EVENTS-P0-008

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

### KIVO-AUDIO-QUEUE-P0-009

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

### KIVO-AUDIO-WASAPI-P0-010

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
- Kivo Native Engine can use WASAPI path
- cargo check passes

## Release and dependency gate

Before adding mpv, FFmpeg, ffprobe, decoder, or output dependencies:

- audit license compatibility
- audit distribution method
- audit source and integrity
- document user-facing dependency notices if required
- avoid undocumented dependency drops
- audit Windows build reliability
- audit binary size
- audit long-term maintenance risk

## Current baseline finding

Current playback code is a useful module skeleton but not yet a real playback core:

- playback_get_state returns default state
- mpv module is descriptor-only
- PlaybackEngine trait is incomplete
- Cargo.toml has no real audio/mpv/ffmpeg/native decode dependency yet
- FFmpeg/ffprobe layer is not present yet
- Kivo Native Engine layer is not present yet

Implementation must proceed by the P0 order above.
