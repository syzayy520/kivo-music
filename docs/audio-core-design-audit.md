# Kivo Audio Core Design Audit

> Ticket: KIVO-AUDIO-DOC-AUDIT-P0  
> Scope: documentation-only audit of the backend audio core design and current implementation reality  
> Rule: this document must be treated as a guardrail before assigning new backend audio implementation work.

## 1. Executive Summary

Kivo's backend audio direction is correct: the product should use a Native-first playback core, with compatibility backends such as mpv treated as secondary adapters only. The current codebase already has a useful skeleton: manager, engine, pipeline, decoder, output, event, lifecycle, queue, and activity-log boundaries exist as separate modules.

However, the current implementation is not a finished playback core. It is a structured foundation. Native playback, output submission, the pipeline worker runtime, and Windows/WASAPI output are still intentionally unsupported. The design documentation must therefore not describe Kivo Core Audio as already capable of real playback. It must describe the present state as: scaffolded, partially decoded for WAV, event-aware, but not yet wired to real output.

The next work must focus on a controlled internal pipeline path before touching WASAPI, FFmpeg, mpv, UI, or broad frontend integration.

## 2. Non-Negotiable Architecture Principle: Tree-Layer Governance

The audio core must follow a strict tree-shaped responsibility model.

Think of the codebase like a government hierarchy:

```text
President
└── Province Governor
    └── City Mayor
        └── County Head
            └── Township Head
                └── Local Officer
```

In Kivo audio code, this means:

```text
Tauri Command Layer
└── PlaybackManagerState
    └── PlaybackManager
        └── PlaybackEngine / KivoNativeEngine
            └── NativePipeline
                ├── Decoder Runtime
                │   └── Concrete Decoder, such as WAV decoder
                ├── Output Runtime
                │   └── Concrete Output Sink, eventually WASAPI
                └── Worker State / Event Bridge
```

Hard rule:

- A parent may coordinate only its direct children.
- A child must not reach upward and mutate parent state directly.
- A high-level layer must not skip layers and control low-level internals.
- A low-level module must not know about UI, Tauri command details, or page state.
- Each folder owns one responsibility class.
- Each file owns one responsibility.
- If one file starts becoming both policy + runtime + transport + tests + formatting, split it.

Bad pattern:

```text
commands.rs directly opens decoder and submits frames to output
manager.rs manually parses audio formats
native.rs directly performs device-specific WASAPI behavior
wav_decoder.rs writes playback state or emits UI events
```

Good pattern:

```text
commands.rs calls PlaybackManagerState
PlaybackManagerState locks and delegates to PlaybackManager
PlaybackManager delegates to KivoNativeEngine
KivoNativeEngine delegates runtime work to NativePipeline
NativePipeline delegates decode work to decoder runtime and output work to output runtime
Concrete decoder only decodes
Concrete output sink only outputs
Events are emitted through the event bridge/bus layer
```

This tree model is not optional. It is the maintainability foundation for Kivo.

## 3. Current Implementation Reality

### 3.1 What already exists and is good

The current backend audio core already has useful module boundaries:

- `playback/manager.rs`
- `playback/engine.rs`
- `playback/backends/native.rs`
- `playback/native_pipeline.rs`
- `playback/decoder.rs`
- `playback/decoders/*`
- `playback/output.rs`
- `playback/native_output.rs`
- `playback/windows_audio.rs`
- `playback/events.rs`
- `playback/playback_event_bus.rs`
- `playback/playback_event_bridge.rs`
- `playback/playback_worker_*`
- `playback/activity_*`
- `playback/lifecycle_*`

This is a good sign. The project is not a random pile of playback code.

### 3.2 What is not real yet

The following must be treated as not implemented yet:

- Real native playback.
- Real output submission.
- WASAPI shared mode.
- WASAPI exclusive mode.
- Bit-perfect output.
- Real playback worker thread.
- Real decode-output scheduling loop.
- mpv compatibility runtime.
- FFmpeg-based playback or decoding.
- Multi-format decoder support beyond the current WAV direction.

The current backend must not be described as production playback.

## 4. Design Document Corrections Required

If any existing design document says or implies the following, it must be corrected.

### 4.1 Incorrect: `KivoNativeEngine already plays audio`

Correct wording:

```text
KivoNativeEngine is the primary engine boundary. It currently owns playback state and pipeline scaffolding, but real output is not wired yet.
```

### 4.2 Incorrect: `WASAPI is implemented`

Correct wording:

```text
Windows audio is currently represented by planning/status data structures only. WASAPI shared/exclusive output must be implemented in later tickets after the internal decode pipeline is stable.
```

### 4.3 Incorrect: `mpv is the current playback backend`

Correct wording:

```text
mpv is a compatibility backend descriptor only. It is not the Kivo primary playback core and must not drive the architecture.
```

### 4.4 Incorrect: `FFmpeg is part of playback`

Correct wording:

```text
FFmpeg, if introduced later, may be used behind a decoder/probe boundary only. It must not become the playback manager, output system, or UI-facing playback core.
```

### 4.5 Incorrect: `load means real audio is ready to play`

Correct wording:

```text
load must eventually mean the track has been validated, decoder opened, pipeline prepared, and state committed atomically. The current partial state update + unsupported result must be corrected before real playback integration.
```

## 5. Current Risk Audit

### P0 Risk 1: Partial state mutation before unsupported error

Current behavior allows load-like operations to prepare some internal state and then return unsupported. This can confuse frontend state, events, tests, and future worker logic.

Required correction:

- Define atomic command semantics.
- Either commit state only after success, or introduce an explicit `PreparedButOutputUnsupported`-style transitional model.
- Add tests proving failed load does not corrupt previous state unless explicitly designed.

### P0 Risk 2: Pipeline worker exists but does not execute real runtime work

The pipeline has command mapping and state-transition scaffolding, but real command handling is unsupported.

Required correction:

- Implement a controlled internal pipeline step first.
- Do not jump directly to WASAPI.
- Do not jump directly to FFmpeg.
- Do not pretend output exists.

### P0 Risk 3: Output settings are ahead of output implementation

Output settings already contain concepts such as exclusive mode and bit-perfect mode, but there is no real output engine yet.

Required correction:

- Keep fields as planning data.
- Do not expose them as working user-facing features yet.
- Implement shared output before exclusive output.
- Add capability reporting so the frontend can tell available vs planned features.

### P0 Risk 4: Decoder/probe boundary may blur later

The decoder should decode audio frames. Metadata probing, cover extraction, library scanning, and duration/codec inspection should not be randomly mixed into playback manager or concrete decoder code.

Required correction:

- `MediaProbeService` owns probing.
- `AudioDecoder` owns frame decode.
- `NativePipeline` owns runtime orchestration.
- `PlaybackManager` owns high-level playback commands and queue coordination only.

### P0 Risk 5: Manager lock boilerplate can grow into a maintenance hazard

`PlaybackManagerState` currently repeats lock/delegate patterns many times.

Required correction:

- Keep `manager.rs` from becoming a large command dumping ground.
- If more methods are added, introduce small internal helpers or split state wrapper logic.
- Do not mix queue policy, engine runtime, activity event mapping, and output policy into `manager.rs`.

## 6. Required Layer Ownership Map

### 6.1 Tauri command layer

Owns:

- Request/response boundary.
- Calling manager state.
- Returning serializable results.

Must not own:

- Decode logic.
- Output logic.
- Worker loop.
- Device-specific behavior.
- Audio format policy.

### 6.2 PlaybackManagerState

Owns:

- Thread-safe access to `PlaybackManager`.
- Locking and delegation.

Must not own:

- Audio processing.
- Queue algorithms beyond delegation.
- Event policy beyond calling approved helper layers.

### 6.3 PlaybackManager

Owns:

- Primary engine reference.
- Compatibility backend descriptors.
- Queue coordination.
- High-level playback command delegation.

Must not own:

- Decode implementation.
- Output sink implementation.
- WASAPI details.
- FFmpeg details.
- Frontend/UI concepts.

### 6.4 PlaybackEngine / KivoNativeEngine

Owns:

- Engine-level playback state.
- Mapping high-level commands to native pipeline operations.
- Engine descriptor/capabilities.

Must not own:

- Concrete decoder internals.
- Concrete output internals.
- Device enumeration details.
- UI activity rendering.

### 6.5 NativePipeline

Owns:

- Decode/output orchestration.
- Worker command handling.
- Pipeline state.
- Frame flow from decoder to output boundary.

Must not own:

- Tauri commands.
- UI state.
- Library metadata policy.
- Queue policy.

### 6.6 Decoder runtime

Owns:

- Opening decoder from approved requests.
- Reading frames.
- Seeking.
- Closing.
- Reporting decode errors.

Must not own:

- Playback queue.
- UI events.
- Output device selection.
- WASAPI policy.

### 6.7 Output runtime

Owns:

- Device open/close.
- Frame submission.
- Pause/resume/flush/stop at output level.
- Runtime status such as active device, latency, gaps, and last error.

Must not own:

- Decoder format guessing.
- Queue selection.
- Track metadata scraping.
- Frontend state.

### 6.8 Event/activity layer

Owns:

- Mapping command/runtime results to events.
- Recording activity log.
- Progress throttling.

Must not own:

- Playback decisions.
- Decode operations.
- Output operations.

## 7. Next Development Sequence

The next tasks must be ordered. Do not skip steps.

### Stage 1: Documentation correction and guardrails

Status: this document.

Goal:

- Make the design reality explicit.
- Add tree-layer governance rules.
- Prevent future workers from claiming unsupported features are implemented.

### Stage 2: Native decode pipeline without real output

Recommended ticket:

```text
KIVO-AUDIO-NATIVE-DECODE-PIPELINE-P0-009
```

Goal:

- Connect WAV decoder factory to NativePipeline.
- Allow pipeline to open a decoder for a valid WAV path.
- Allow one controlled decode step.
- Store or expose decoded frame state internally.
- Keep real output unsupported.

Allowed files should be limited to:

```text
src-tauri/src/playback/native_pipeline.rs
src-tauri/src/playback/backends/native.rs
src-tauri/src/playback/decoder_session.rs
src-tauri/src/playback/decoder_runtime_state.rs
src-tauri/src/playback/decoders/factory.rs
src-tauri/src/playback/native_pipeline_tests.rs
new small playback test files if needed
```

Forbidden:

```text
No frontend changes.
No UI changes.
No mpv runtime.
No FFmpeg.
No WASAPI.
No real output claims.
No broad refactor.
No package dependency expansion unless explicitly approved.
```

Acceptance:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
```

Functional checks:

```text
- WAV decoder can be created from path.
- NativePipeline can open a decoder session.
- NativePipeline can execute one decode step.
- Decode errors are typed.
- Unsupported formats remain typed UnsupportedFormat.
- Real output remains clearly unsupported.
- Failed load/decode does not corrupt prior committed playback state.
```

### Stage 3: Null output sink for internal pipeline tests

Goal:

- Add a test-only or internal null output sink.
- Prove frame submission path works without real audio hardware.
- Track pending frames / submitted frames / output status.

Do not implement WASAPI yet.

### Stage 4: Worker runtime model

Goal:

- Establish command queue and worker state transitions.
- Prove play/pause/resume/stop/seek transitions without real hardware.
- Add runtime event emission from worker state, not only command result mapping.

### Stage 5: Windows output capability probe

Goal:

- Detect whether Windows audio backend is available.
- Enumerate devices only after the pipeline model is stable.
- Keep shared/exclusive/bit-perfect capability reporting honest.

### Stage 6: WASAPI shared mode output

Goal:

- Implement safe shared-mode output first.
- Avoid exclusive mode until shared mode is stable.
- Keep device lost, sample-rate mismatch, and latency status explicit.

### Stage 7: WASAPI exclusive / bit-perfect experiments

Goal:

- Implement as an opt-in capability.
- Add clear fallback rules.
- Never claim bit-perfect unless format negotiation proves it.

### Stage 8: Multi-format decoder expansion

Goal:

- Decide decoder strategy after WAV pipeline works.
- Candidate directions: Symphonia, FFmpeg behind a strict adapter, or staged codec-specific decoders.
- Keep decoder abstraction clean.

### Stage 9: mpv compatibility backend

Goal:

- Implement only as an adapter after native core remains primary.
- Must not pollute NativePipeline.
- Must pass licensing/distribution review before bundling.

## 8. Gates for Every Audio Core Ticket

Every backend audio ticket must report:

```text
State Header:
- Repo
- Branch
- Base commit
- HEAD before
- Ticket ID
- Scope
- Non-goals

Changed files:
- exact list only

Layer ownership check:
- Which layer was changed?
- Which direct child layer did it call?
- Did it skip any layer? If yes, stop.

Single-responsibility check:
- Any file taking on a second responsibility? If yes, split or stop.
- Any large file growth? If yes, justify or split.

Truthfulness check:
- Does any command claim real playback/output when it is still unsupported?
- Does backend status honestly report available vs planned?

Gates:
- cargo fmt
- cargo test
- cargo clippy -- -D warnings
- git diff --stat
- git status --short
```

## 9. Wording Rules for Future Documentation

Use:

```text
scaffolded
planned
unsupported
not wired yet
internal pipeline boundary
native-first architecture
compatibility backend descriptor
```

Do not use until true:

```text
real playback implemented
WASAPI output supported
bit-perfect supported
mpv backend active
FFmpeg playback core
production-ready audio engine
```

## 10. Final Decision

The backend audio core should continue, but the next step must be disciplined:

```text
Do documentation correction first.
Then connect WAV decode into NativePipeline.
Then add null output tests.
Then worker runtime.
Then Windows output capability.
Then WASAPI shared mode.
Only then consider exclusive mode, FFmpeg, or mpv runtime.
```

The architecture must stay tree-shaped and layered. Kivo cannot become a pile of direct cross-calls. Every file must keep one clear job, every folder must keep one responsibility class, and every parent layer must manage only its direct child layer.
