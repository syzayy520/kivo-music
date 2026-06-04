# Kivo Audio WASAPI Scaffold Governance

**Scope**: This document governs all audio backend work on the `kivo-audio-native-decode-pipeline-p0-009` branch. It defines mandatory rules, risk boundaries, and acceptance criteria for every audio ticket. No ticket may violate these rules without a dedicated audit ticket explicitly approving the exception.

---

## 1. Current Stage

- **Branch**: `kivo-audio-native-decode-pipeline-p0-009`
- **Safety point**: `cb361be`
- **Stage**: WASAPI scaffold only
- **Real playback**: NO — no audible output, no Windows audio device opened
- **NativePipeline integration**: NO — `NativePipeline` uses `KivoNativeOutputSink` → `KivoNullOutputSink`, not `WasapiOutputSink`
- **PlaybackCapabilities**: CLOSED — all capability flags remain `false`
- **Output thread**: NOT started — `ring_buffer_output_thread` exists only as scaffold/test boundary
- **submit_frame**: Returns `UnsupportedOperation` — this is intentional and must not change without a dedicated audit ticket

---

## 2. Product Boundary

Kivo Music is a local desktop music player. It plays audio files from the user's local filesystem through the native audio pipeline.

**Forbidden product semantics** (must not appear as positive features):
- Resource search, download, or aggregation
- Cloud-drive resource search or automatic cloud transfer
- Magnet links, torrents, or peer-to-peer distribution
- External platform aggregation or scraping
- Online resource discovery or streaming from unauthorized third-party services

**Allowed local ownership semantics**:
- User-owned local files
- User-selected local folders
- User-owned local NAS / SMB / network shares
- Local network storage indexing, when explicitly assigned
- Local-first library management for user-controlled storage

These terms may only appear in negative guardrails (e.g., "this feature is not supported").

---

## 3. Strict Tree-Layer Governance Model

```
Page Layer
  → Command Layer
    → Manager Layer
      → Engine Layer
        → Pipeline Layer
          → Decoder Layer
          → Output Layer
```

### Rules

1. Each layer may only manage the next lower layer.
2. No layer may skip downward across multiple layers.
3. No lower layer may control an upper layer.
4. Cross-layer shortcuts are forbidden.
5. A file must belong to one layer and one responsibility.
6. A file over 220 lines must be split or explicitly justified.
7. A file over 260 lines fails the ticket unless the ticket is a read-only audit.
8. One file, one responsibility. One folder, one responsibility. One module, one layer.

### Layer Responsibilities

| Layer | Responsibility |
|-------|---------------|
| Page | UI composition and user interaction only |
| Command | Receive requests, validate parameters, call Manager only |
| Manager | Orchestrate engines only; no direct decoder or output access |
| Engine | Lifecycle state machine, task coordination only |
| Pipeline | Frame flow orchestration: decoder → buffer → output |
| Decoder | Read, probe, decode, produce frames only |
| Output | Output sink, RingBuffer, WASAPI scaffold, device boundary only |

### WASAPI Scaffold Constraints

- `WasapiOutputSink` belongs exclusively to the Output Layer.
- `RingBuffer` belongs exclusively to the Output Layer internal buffer boundary.
- `SilentRingBufferWriter` belongs exclusively to the Output Layer `frame_bridge` scaffold helper.
- `submit_frame` semantics belong to the `OutputSink` trait boundary — do not modify without audit.
- `NativePipeline` belongs to the Pipeline Layer — do not connect to `WasapiOutputSink`.
- `KivoNativeOutputSink` belongs to the output facade — do not switch from `NullOutput` to WASAPI.
- `PlaybackCapabilities` is not the Output Layer's decision — do not open capabilities because a scaffold exists.
- Any cross-layer integration (e.g., `NativePipeline` → `WasapiOutputSink`) requires a separate audit ticket.

---

## 4. Current Code Truth

| Component | State |
|-----------|-------|
| `WasapiOutputSink` | Lifecycle-only scaffold; no real device, no audible output |
| `submit_frame` | Returns `UnsupportedOperation`; does NOT write to RingBuffer |
| `ring_buffer` field | `Option<RingBuffer>` — ownership scaffold exists |
| `prepare_ring_buffer_for_stream` | `pub(crate)` — creates/replaces RingBuffer only, no data written |
| `SilentRingBufferWriter` | Exists in `frame_bridge/silent_writer.rs`; NOT connected to `submit_frame` |
| `RingBuffer::write_frames` | Exists; NOT called from production path |
| `NativePipeline` | Uses `KivoNativeOutputSink` → `KivoNullOutputSink`; NOT connected to `WasapiOutputSink` |
| `KivoNativeOutputSink` | Delegates to `KivoNullOutputSink`; NOT switched to WASAPI |
| `PlaybackCapabilities` | All flags `false`; no WASAPI capability enabled |
| Output thread | Scaffold/test boundary only; NOT started |
| Non-silent data | FORBIDDEN — `ensure_silent_frame` rejects non-zero samples |

---

## 5. Non-Negotiable Rules

1. Do not modify `submit_frame` unless a dedicated ticket explicitly allows it.
2. Do not return `Ok` from `submit_frame` unless trait semantics were separately audited.
3. Do not call `SilentRingBufferWriter` from production path unless explicitly allowed.
4. Do not call `RingBuffer::write_frames` unless explicitly allowed.
5. Do not write non-silent data.
6. Do not connect `NativePipeline` to `WasapiOutputSink`.
7. Do not modify `native_output.rs` or `native_null_output.rs` unless explicitly allowed.
8. Do not start output threads.
9. Do not use Windows audio APIs from scaffold tickets.
10. Do not enable `PlaybackCapabilities`.
11. Do not change `Cargo.toml` / `Cargo.lock` unless explicitly allowed.
12. Do not modify UI or frontend bridge.
13. Do not use "playback complete" wording without real Windows audible validation.
14. Do not create public APIs for scaffold-only helpers.
15. Do not use production-ready names for silent/test-only helpers.
16. Do not hide ignored-test failures or skipped tests.
17. Do not trust completion reports without remote diff inspection.
18. Do not run broad `fmt` and stage unrelated files.
19. Do not `git add .`
20. Do not force push.

---

## 6. Naming Rules

### Allowed Helper Names

- `write_silent_frame_for_scaffold`
- `submit_silent_frame_to_prepared_ring_buffer_for_scaffold`
- `test_only_silent_ring_buffer_write`
- `scaffold_silent_write`

### Forbidden Helper Names

- `play_frame`
- `write_audio`
- `submit_to_wasapi`
- `render_audio`
- `output_audio`
- `real_playback`
- `wasapi_play`
- `native_playback_ready`

**Reason**: Names must not imply that real playback is complete or that WASAPI output is functional.

---

## 7. submit_frame Risk

`OutputSink::submit_frame` returning `Ok` is understood by upper layers as "output succeeded." Without an output thread, without a WASAPI render client, and without a real device consumption chain, returning `Ok` from `submit_frame` is a major semantic risk.

**Current recommended path**: Internal/test-visible helper first. `submit_frame` remains `UnsupportedOperation` until a dedicated audit ticket approves the change.

**Why this matters**: If `submit_frame` returns `Ok` and `pending_frames` increments, the pipeline and engine layers will believe frames are being played. This creates a false completion signal that propagates upward to the UI.

---

## 8. RingBuffer Risk

`RingBuffer` is an in-memory queue. `available_frames` and `pending_frames` must not be described as "audible playback." Without a consumption thread and WASAPI render client, RingBuffer accumulation does not equal output success.

**Key constraints**:
- `RingBuffer` frames are not audible until consumed by an output thread connected to a real device.
- `pending_frames` semantics must be carefully managed — do not fabricate real output state.
- Buffer overflow (`WouldBlock`) is expected when no consumer exists; this is not a playback failure.

---

## 9. Silent-Only Risk

Silent-only is a safe placeholder path. Silent frame writing proves only that:
- Format bridging works (Float32 → bytes)
- RingBuffer write path is testable
- Silence guard rejects non-silent data

**Silent-only does NOT equal**:
- Playback
- Audio output
- WASAPI completion
- Real device readiness

Non-silent frames must be rejected until the real output chain is ready.

---

## 10. PlaybackCapabilities Rules

Any capability related to `can_use_wasapi`, `can_select_output_device`, `exclusive_mode`, or `bit_perfect_mode` may only be opened after:
- Real output chain is validated
- Windows device enumeration works
- Audible output is confirmed
- A dedicated audit ticket approves the change

Scaffold existence, helper existence, RingBuffer existence, and silent writer existence are NOT justification for opening capabilities.

---

## 11. NativePipeline Rules

`NativePipeline` must NOT connect to `WasapiOutputSink`. `KivoNativeOutputSink` must NOT switch from `KivoNullOutputSink` to WASAPI.

Before any integration, a separate audit ticket must confirm:
- Output lifecycle management
- Thread lifecycle and shutdown
- Decode loop backpressure handling
- Error propagation across layers
- stop/flush/close resource release
- Windows device validation
- UI responsiveness under load
- No deadlock, no panic, no resource leak

---

## 12. File Responsibility Rules

- One file, one responsibility.
- One folder, one responsibility.
- One module, one layer.
- Do not pile unrelated helpers into `sink.rs`.
- Do not pile state machine logic into output modules.
- Do not pile output logic into pipeline modules.
- Do not pile decoder logic into manager modules.
- If a file exceeds 220 lines, the task must split or stop.
- If a file exceeds 260 lines, the task fails unless it is a read-only audit.
- Future implementation tickets must report line counts for all modified files.

---

## 13. Required Gates For Audio Tickets

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml -- wasapi_output_sink
cargo test --manifest-path src-tauri/Cargo.toml -- frame_bridge
cargo test --manifest-path src-tauri/Cargo.toml -- ring_buffer
cargo test --manifest-path src-tauri/Cargo.toml -- ring_buffer_output_thread
cargo test --manifest-path src-tauri/Cargo.toml -- output_wasapi
cargo test --manifest-path src-tauri/Cargo.toml -- native_pipeline
cargo test --manifest-path src-tauri/Cargo.toml -- playback
cargo test --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml -- --ignored
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
git diff --name-only
git diff --check
git status --short
```

**Note**: If a test filter does not match, the ticket must report the failure and run the correct test name. If ignored tests cannot run, the reason must be stated — do not pretend they passed.

---

## 14. Completion Report Rules

Every ticket must report:
- State Header (repo, branch, base, new HEAD, ticket ID)
- Modified files with line counts
- Allowlist compliance
- Forbidden files untouched
- Gates key output
- Whether `cargo fmt` touched non-allowlist files
- Whether `submit_frame` changed
- Whether `SilentRingBufferWriter` was called
- Whether `RingBuffer::write_frames` was called
- Whether `NativePipeline` changed
- Whether `PlaybackCapabilities` changed
- Whether non-silent data path was introduced
- Commit hash and push result
- Clean status

---

## 15. Acceptance Rules

| Condition | Verdict |
|-----------|---------|
| Read-only ticket has diff | REJECT |
| Implementation ticket exceeds allowlist | REJECT |
| Gates not run | NOT ACCEPTED |
| `submit_frame` silently changed | REJECT |
| Capabilities opened | REJECT |
| `NativePipeline` connected to WASAPI | REJECT |
| Non-silent path introduced | REJECT |
| File over 220 lines without split/justification | NOT ACCEPTED |
| File over 260 lines | REJECT |
| Cross-layer violation | REJECT |
| "Real playback" claimed without Windows audible test | REJECT |

---

## 16. Markdown-only Gate Policy

This ticket changes Markdown only. Cargo gates are useful as repository health checks. If cargo gates fail due to pre-existing Rust/code issues unrelated to this Markdown-only ticket, the failure must be reported honestly, but it must not block committing this documentation. Do not modify Rust/source files to fix cargo gates in this ticket. If any failure is caused by this ticket, stop and report.

---

## 17. Next Safe Route

```
P0-058A  submit_frame silent write audit (done)
  → P0-058G  governance doc (this ticket)
  → P0-058B  internal/test-visible silent helper
  → P0-058C  scope audit
  → P0-059A  submit_frame semantics audit
  → P0-059B  maybe submit_frame silent-only behavior
  → later    output thread consumption audit
  → later    NativePipeline integration audit
  → later    PlaybackCapabilities enablement
```

**P0-058B must NOT directly change `submit_frame`.** P0-058B is recommended to add only an internal/test-visible helper that calls `SilentRingBufferWriter`. `submit_frame` remains `UnsupportedOperation`.

---

*This document is the authoritative governance reference for all audio backend work on this branch. Violations require a dedicated justification ticket before merge.*
