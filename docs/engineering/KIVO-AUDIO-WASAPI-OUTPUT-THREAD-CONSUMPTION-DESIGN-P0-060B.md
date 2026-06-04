# KIVO-AUDIO-WASAPI-OUTPUT-THREAD-CONSUMPTION-DESIGN-P0-060B

**Type**: Design document only
**Ticket**: P0-060B
**Base**: 5ae2012
**Branch**: `kivo-audio-native-decode-pipeline-p0-009`
**Status**: Design-only — no Rust code changes, no real playback

---

## 1. Current Accepted State

| Component | State |
|---|---|
| Safety point | `5ae2012` |
| Stage | WASAPI scaffold only |
| `submit_frame` | Returns `UnsupportedOperation` |
| `ScaffoldSilentWriter` | Exists, `#[allow(dead_code)]`, not in production path |
| `RingBuffer` | Memory-only buffer, no consumer thread |
| `output thread` modules | Smoke test scaffold behind opt-in env vars |
| `NativePipeline` | Uses `KivoNativeOutputSink` → `KivoNullOutputSink`, NOT `WasapiOutputSink` |
| `PlaybackCapabilities` | All flags `false`, no WASAPI capability enabled |
| Real playback | **None** — no audible output, no Windows audio device opened |
| Consumer thread | **None** — `read_frames_or_silence` has no production callers |
| `rendered_frames` | **Does not exist** — no field tracks hardware-rendered frames |
| WASAPI production path | **None** — all WASAPI ops behind opt-in env vars |

Accepted tickets: P0-058A, P0-058G, P0-058G-FIX, P0-058H, P0-058B-A, P0-058B-PRE, P0-058B, P0-058C-24H, P0-059A, P0-060A.

---

## 2. Non-Goals

This document does NOT:

- Implement any Rust code
- Change `submit_frame` behavior
- Connect `NativePipeline` to `WasapiOutputSink`
- Open `PlaybackCapabilities`
- Open any Windows audio device
- Start any output thread
- Claim audible playback is possible
- Modify UI or frontend
- Change `Cargo.toml` / `Cargo.lock`
- Modify existing governance or runbook documents
- Create any production code path
- Call `ScaffoldSilentWriter` from production
- Write non-silent data

This document IS:

- A design specification for future implementation tickets
- A prerequisite analysis for real WASAPI output
- A ticket queue definition for the next 6-10 tickets

---

## 3. Output Thread Consumption Problem

### 3.1 The Semantic Gap

Today, there is a chain of "write" operations, but no "consume" operations:

```
[Decoder] → [AudioBuffer] → [RingBuffer::write_frames] → ??? → [WASAPI GetBuffer] → [Speaker]
                                        ↑
                              We are here (write only)
                                        ↓
                              No consumer exists
```

### 3.2 Five Distinct States

| State | Meaning | Current Status |
|---|---|---|
| **accepted** | `submit_frame` received the frame | Not implemented (returns `UnsupportedOperation`) |
| **buffered** | Frame written to `RingBuffer` | Possible via `ScaffoldSilentWriter` only |
| **consumed** | Output thread read frame from `RingBuffer` | **Does not exist** |
| **rendered** | Frame written to WASAPI `GetBuffer`/`ReleaseBuffer` | **Does not exist** |
| **audible** | Frame verified as audible output | **Does not exist** |

### 3.3 Why submit_frame Cannot Return Ok Today

If `submit_frame` returns `Ok`:
1. `NativePipeline` updates `output_status` and advances clock (`native_pipeline_drain.rs:22-29`)
2. Engine layer believes frame was "played"
3. UI shows playback progress
4. User believes audio is playing

But no audio is playing. This is a **false completion signal** that propagates upward through every layer.

### 3.4 Current RingBuffer Stats Gap

`RingBufferStats` tracks: `total_frames_written`, `total_frames_read`, `total_silence_frames_filled`, `underrun_count`, `overrun_count`.

**Missing**: `rendered_frames` — frames confirmed written to WASAPI render client. Without this, we cannot prove any frame reached hardware.

---

## 4. Required State Model

### 4.1 RingBuffer Stats (ring_buffer layer) — already exist

| Field | Type | Description |
|---|---|---|
| `total_frames_written` | `u64` | Frames accepted by `write_frames` |
| `total_frames_read` | `u64` | Frames returned by `read_frames_or_silence` |
| `total_silence_frames_filled` | `u64` | Silence bytes injected on underrun |
| `underrun_count` | `u64` | Times consumer found buffer empty |
| `overrun_count` | `u64` | Times producer found buffer full |

Defined in `ring_buffer/types.rs`.

### 4.2 Output Thread Stats (output thread layer) — to be created

| Field | Type | Description |
|---|---|---|
| `consumed_frames` | `u64` | Frames read from RingBuffer by consumer |
| `rendered_frames` | `u64` | Frames confirmed written to WASAPI render client |
| `silence_filled_frames` | `u64` | Silence frames injected by consumer on underrun |
| `dropped_frames` | `u64` | Frames dropped due to backpressure or error |
| `render_error_count` | `u64` | Number of GetBuffer/ReleaseBuffer failures |
| `device_lost_count` | `u64` | Number of device-lost events |
| `last_output_thread_error` | `Option<String>` | Last error message from consumer loop |
| `last_render_error` | `Option<String>` | Last WASAPI render error |
| `output_thread_state` | `OutputThreadState` | Current lifecycle state (see 6.1) |

### 4.3 OutputRuntimeStatus Fields — already exist

`pending_frames`, `is_open`, `is_active`, `last_error` — defined in `output.rs:48`.

### 4.4 Fields NOT for Public API

`consumed_frames`, `rendered_frames`, `render_error_count`, `device_lost_count`, `output_thread_state` should NOT be exposed through `OutputRuntimeStatus` until a dedicated audit approves. Without a real consumer thread, they would always be zero, creating false confidence.

---

## 5. Consumer Loop Design

### 5.1 Overview

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐
│ RingBuffer   │ ──→ │ Consumer Loop │ ──→ │ WASAPI Render   │
│ (producer)   │     │ (output thd) │     │ Client (device) │
└─────────────┘     └──────────────┘     └─────────────────┘
```

### 5.2 Loop Inputs

- `RingBuffer` (shared, thread-safe) — source of audio frames
- `IAudioRenderClient` — WASAPI render client handle
- `IAudioClient` — for `GetCurrentPadding`, `Start`, `Stop`
- Shutdown signal (`AtomicBool`) — to request clean exit
- `u32 buffer_size_frames` — WASAPI buffer size from `GetBufferSize`

### 5.3 Loop Outputs

- Updated `RingBufferStats` (read counters)
- Updated output thread stats (consumed, rendered, errors)
- Thread exit via channel report

### 5.4 Loop Pseudocode

```
loop:
    if shutdown_requested(): break

    padding = GetCurrentPadding()
    free_frames = buffer_size_frames - padding

    if free_frames == 0:
        sleep(buffer_duration / 4)  // avoid busy loop
        continue

    to_read = min(free_frames, ring_buffer.available_frames())

    if to_read == 0:
        // underrun — fill silence
        fill GetBuffer with silence
        stats.underrun_count += 1
    else:
        ring_buffer.read_frames_or_silence(buf)
        copy buf to GetBuffer

    ReleaseBuffer(to_read, flags)
    stats.consumed_frames += to_read
    stats.rendered_frames += to_read
```

### 5.5 RingBuffer Empty Handling

When `available_frames == 0`: underrun. Fill WASAPI buffer with silence. Increment `underrun_count`. Do NOT block.

### 5.6 RingBuffer Closed Handling

When `is_closed == true`: drain remaining frames with silence fill, then signal end-of-stream, break loop.

### 5.7 Pause Handling

Recommended: Consumer loop keeps running. RingBuffer empties naturally, silence fills. No explicit pause state needed in consumer.

### 5.8 Flush Handling

Signal consumer to pause, reset RingBuffer, clear `pending_frames`, resume consumer.

### 5.9 Backpressure Feedback

Consumer loop never pushes backpressure directly. `RingBuffer::write_frames` returns `WouldBlock` when full. Producer (`submit_frame`) maps `WouldBlock` to error.

### 5.10 Avoiding Busy Loop

When WASAPI buffer full OR RingBuffer empty: `sleep(buffer_duration / 4)`. Use `thread::sleep`, NOT spin-wait.

### 5.11 Avoiding Deadlock

- Consumer never holds lock while calling WASAPI APIs
- RingBuffer uses minimal-lock design
- Shutdown signal uses `AtomicBool` — no mutex wait
- `join()` called from Output Layer owner only

---

## 6. Output Thread Lifecycle Design

### 6.1 Thread State Machine

```
Created → Running → Stopping → Stopped → Closed → Joined
              ↑         │
              └─────────┘ (pause/resume via silence fill)
```

States: `Created`, `Running`, `Stopping`, `Stopped`, `Closed`, `Joined`.

### 6.2 Thread Ownership

- **Owner**: `WasapiOutputSink` (Output Layer)
- **Handle**: `Option<JoinHandle<()>>` stored in sink
- **Shutdown signal**: `Arc<AtomicBool>` shared between sink and thread
- **Report channel**: `mpsc::Receiver<ThreadReport>` for thread exit status

### 6.3 Thread Creation Timing

Created when `open()` succeeds AND device available AND `Initialize` succeeds AND `IAudioRenderClient` obtained. NOT created by `submit_frame`.

### 6.4 Join Responsibility

`close()` MUST join thread. If `close()` called without `stop()`, sends shutdown then joins. Join timeout: 5 seconds.

### 6.5 Panic Handling

Consumer runs inside `catch_unwind`. Panic captured and sent via channel. `close()` surfaces it as `PlaybackError::Output`.

### 6.6 Reopen Handling

`open()` on already-open sink: `close()` first → drop RingBuffer → create new → spawn new thread.

### 6.7 Device Reset Handling

`AUDCLNT_E_DEVICE_INVALIDATED` → set `device_lost` flag → break loop → send error report → recovery requires `close()` + `open()`.

### 6.8 Layer Governance

Output thread belongs to Output Layer ONLY. Page/Command/Manager/Engine/Pipeline MUST NOT directly manage thread. Pipeline may call `submit_frame`, `flush`, `stop`, `close` via OutputSink trait.

---

## 7. submit_frame Future Semantics

### 7.1 Current State

Returns `Err(PlaybackError::UnsupportedOperation(...))`. Intentional and correct.

### 7.2 Minimum Future Semantics

**`Ok` means ONLY**: "frame was accepted into RingBuffer"

**`Ok` does NOT mean**: consumed, rendered, audible, device working.

### 7.3 Precondition Checklist

Before `submit_frame` may return `Ok`:

- [ ] RingBuffer exists and is open
- [ ] Consumer thread is running
- [ ] WASAPI render client is active
- [ ] Error mapping defined (see 7.4)
- [ ] Backpressure strategy implemented
- [ ] Dedicated audit ticket approved

### 7.4 Error Mapping Draft

| Error | When | Maps to |
|---|---|---|
| `NoRingBuffer` | RingBuffer not prepared | `PlaybackError::Output("ring buffer not ready")` |
| `WouldBlock` | Buffer full | `PlaybackError::Output("output buffer full")` |
| `OutputThreadNotRunning` | Thread exited/panicked | `PlaybackError::Output("output thread not running")` |
| `DeviceNotReady` | Device invalidated | `PlaybackError::Output("audio device lost")` |
| `RenderFailed` | GetBuffer/ReleaseBuffer error | `PlaybackError::Output("render error")` |
| `UnsupportedFormat` | Format not supported | `PlaybackError::UnsupportedFormat(...)` |
| `Closed` | Sink is closed | `PlaybackError::Output("sink closed")` |

### 7.5 pending_frames Semantics

Increments on `Ok`, decrements when consumer reads. Represents "accepted but not yet consumed". Does NOT represent "in WASAPI buffer" or "audible".

---

## 8. Backpressure Strategy

### 8.1 Recommended: Return WouldBlock

When `RingBuffer::write_frames` returns `WouldBlock`:

1. `submit_frame` returns `PlaybackError::Output("output buffer full")`
2. Pipeline does NOT advance clock
3. Pipeline retries on next `pump_once()` cycle
4. Frame stays in `NativePipelineBuffer` (not lost)

### 8.2 Why Not Block

- Blocks worker thread → UI freezes
- WASAPI has strict timing → device timeout risk
- Deadlock risk if consumer panics

### 8.3 Why Not Drop

- Audio glitch (silent gap)
- User hears skip
- No recovery signal to pipeline

### 8.4 UI Responsiveness

`submit_frame` never blocks UI thread. If backpressure persists, UI shows buffering indicator.

---

## 9. Flush / Stop / Close Semantics

### 9.1 Flush

**Clears**: RingBuffer frames, `pending_frames` to 0.
**Preserves**: Thread state, device, RingBuffer capacity.
**After flush**: Consumer fills silence until new data arrives.

### 9.2 Stop

**Does**: Sets `is_active = false`.
**Does NOT**: Join thread, drop RingBuffer, close device.
**After stop**: `submit_frame` returns error. Consumer drains then enters silence.

### 9.3 Close

**Does**: Shutdown signal → join thread → drop RingBuffer → reset state → release device.
**Guarantees**: Thread joined, memory freed, device released.

### 9.4 Comparison

| Op | RingBuffer | Thread | Device | State |
|---|---|---|---|---|
| `flush()` | Cleared | Running | Open | Active |
| `stop()` | Preserved | Running | Open | Inactive |
| `close()` | Dropped | Joined | Released | Default |

---

## 10. WASAPI Render Boundary Requirements

### 10.1 GetBuffer / ReleaseBuffer

Called by consumer thread only. GetBuffer returns writable pointer. ReleaseBuffer called after writing. Use `AUDCLNT_BUFFERFLAGS_SILENT` for silence.

### 10.2 Start / Stop / Reset

- `Start`: called after Initialize + GetService + initial prefill
- `Stop`: called during `close()` or device reset
- `Reset`: called after Stop, before re-Initialize

### 10.3 Error Handling

| Error | Action |
|---|---|
| `AUDCLNT_E_DEVICE_INVALIDATED` | Set `device_lost`, break loop |
| `AUDCLNT_E_BUFFER_TOO_LARGE` | Log, reduce request size |
| Other HRESULT | Log, break loop |

### 10.4 Shared Mode Only (Phase 1)

`AUDCLNT_SHAREMODE_SHARED`, no exclusive mode. Exclusive requires separate audit.

### 10.5 Audible Verification

NOT a current phase goal. Requires human listening test or loopback capture.

---

## 11. File / Module Split Plan

### 11.1 Design Principles

- Single file = single function = single responsibility
- Every file ≤ 220 lines (hard limit)
- Test files are separate files

### 11.2 Proposed New Files

| File | Responsibility | Est. Lines | Not Allowed |
|---|---|---|---|
| `output_thread_state.rs` | State enum, stats struct, defaults | ~80 | No WASAPI, no thread spawn |
| `output_thread_control.rs` | Thread spawn/join/panic, shutdown signal | ~180 | No RingBuffer ops, no render |
| `output_thread_consumer.rs` | Consumer loop, GetBuffer/ReleaseBuffer, stats | ~200 | No thread spawn, no sink lifecycle |
| `output_thread_errors.rs` | Error enum, HRESULT mapping | ~100 | No thread logic |
| `sink_submit.rs` | `submit_frame` impl, error mapping, RingBuffer write | ~150 | No thread mgmt, no WASAPI |
| `sink_submit_tests.rs` | submit_frame error mapping tests | ~180 | No production code |
| `output_thread_consumer_tests.rs` | Consumer loop unit tests | ~200 | No production code |

### 11.3 Why Each File Fits Under 220 Lines

- `output_thread_state.rs`: Enums and structs with derives only
- `output_thread_control.rs`: Thread lifecycle only
- `output_thread_consumer.rs`: Single loop function + helpers
- `output_thread_errors.rs`: Error types and mapping tables
- `sink_submit.rs`: Single function with error branches
- Test files: ~15-20 lines per test, 10 tests ≈ 200 lines

### 11.4 Existing Files to Modify

| File | Change |
|---|---|
| `sink.rs` | Add delegation to `sink_submit.rs` helper |
| `mod.rs` | Add new module declarations |

### 11.5 Files NOT Proposed

No changes to `native_pipeline.rs`, `native_output.rs`, `capabilities.rs`, `Cargo.toml` — each requires separate audit.

---

## 12. Test Plan

### 12.1 Unit Tests (cargo test, no device)

| Category | File | Description |
|---|---|---|
| State transitions | `output_thread_state_tests.rs` | Valid/invalid transitions |
| Thread lifecycle | `output_thread_control_tests.rs` | Spawn, join, panic |
| Consumer mock | `output_thread_consumer_tests.rs` | Mock RingBuffer + mock WASAPI |
| Error mapping | `output_thread_errors_tests.rs` | HRESULT → PlaybackError |
| submit_frame | `sink_submit_tests.rs` | RingBuffer errors → PlaybackError |
| Backpressure | `sink_submit_tests.rs` | WouldBlock handling |
| Flush/Stop/Close | `sink_lifecycle_tests.rs` | Lifecycle after operations |

### 12.2 Opt-in Smoke Tests (Windows, behind env vars)

| Test | Env Var |
|---|---|
| Full consumer lifecycle | `KIVO_WASAPI_CONSUMER_SMOKE=1` |
| Device lost recovery | `KIVO_WASAPI_DEVICE_LOST_SMOKE=1` |

### 12.3 Non-Windows

All WASAPI tests return `skipped_non_windows`. Thread/RingBuffer tests run on all platforms.

### 12.4 Regression Tests

Every boundary must confirm: `WasapiOutputSink` still scaffold, `PlaybackCapabilities` still false, `submit_frame` unchanged, `KivoNativeOutputSink` still NullSink.

---

## 13. Implementation Ticket Queue

### P0-060C: Output Thread State Types
- **Type**: Implementation
- **Purpose**: Define `OutputThreadState` enum, `OutputThreadStats` struct
- **Allowed**: `output_thread_state.rs`, tests, `mod.rs`
- **Forbidden**: `sink.rs`, `native_pipeline.rs`, `capabilities.rs`, `Cargo.toml`
- **Gates**: All cargo gates, line count ≤ 220
- **Acceptance**: Types compile, tests pass, no behavior change

### P0-060D: Output Thread Control Boundary
- **Type**: Implementation
- **Purpose**: Thread spawn/join/panic scaffold, shutdown signal
- **Allowed**: `output_thread_control.rs`, tests, `mod.rs`
- **Forbidden**: `sink.rs`, consumer code, WASAPI code
- **Gates**: All cargo gates, line count ≤ 220
- **Acceptance**: Thread lifecycle works, no real device

### P0-060E: RingBuffer Consumer Scaffold Tests
- **Type**: Implementation
- **Purpose**: Tests for RingBuffer read semantics, stats, underrun
- **Allowed**: `ring_buffer_tests/buffer_tests.rs`, `ring_buffer/buffer.rs` (if stats added)
- **Forbidden**: `sink.rs`, thread code, WASAPI code
- **Gates**: All cargo gates, line count ≤ 220
- **Acceptance**: Tests pass, stats correct

### P0-060F: Output Thread Consumer Scaffold
- **Type**: Implementation
- **Purpose**: Consumer loop body with mock WASAPI, sleep on empty
- **Allowed**: `output_thread_consumer.rs`, `output_thread_consumer_tests.rs`, `output_thread_errors.rs`
- **Forbidden**: `sink.rs`, real WASAPI calls
- **Gates**: All cargo gates, line count ≤ 220
- **Acceptance**: Loop logic correct, no real device

### P0-061A: submit_frame Error Mapping Audit
- **Type**: Read-only audit
- **Purpose**: Audit all error paths from RingBuffer write to submit_frame
- **Allowed**: None (chat-only report)
- **Forbidden**: All files
- **Acceptance**: Report delivered in chat

### P0-061B: sink_submit Helper Implementation
- **Type**: Implementation
- **Purpose**: Implement `submit_frame` helper that writes to RingBuffer
- **Allowed**: `sink_submit.rs`, tests, `sink.rs` (delegation), `mod.rs`
- **Forbidden**: `native_pipeline.rs`, thread code, real WASAPI
- **Gates**: All cargo gates, line count ≤ 220
- **Acceptance**: submit_frame writes to RingBuffer, errors mapped correctly

### P0-061C: sink_submit Scope Audit
- **Type**: Read-only audit
- **Purpose**: Audit submit_frame change safety, confirm no false Ok
- **Allowed**: None (chat-only report)
- **Forbidden**: All files
- **Acceptance**: Report delivered in chat

### P0-062A: NativePipeline Integration Audit
- **Type**: Read-only audit
- **Purpose**: Audit safety of connecting NativePipeline to WasapiOutputSink
- **Allowed**: None (chat-only report)
- **Forbidden**: All files
- **Acceptance**: Report delivered in chat, all prerequisites confirmed

### P0-062B: PlaybackCapabilities Enablement Audit
- **Type**: Read-only audit
- **Purpose**: Audit safety of enabling WASAPI capabilities
- **Allowed**: None (chat-only report)
- **Forbidden**: All files
- **Acceptance**: Report delivered in chat

### P0-062C: Final WASAPI Scope Audit
- **Type**: Read-only audit
- **Purpose**: Final confirmation that all boundaries are safe
- **Allowed**: None (chat-only report)
- **Forbidden**: All files
- **Acceptance**: Report delivered in chat

**Important**: No ticket in this queue directly connects NativePipeline, opens capabilities, or enables real playback. Each of those requires a separate audit ticket.

---

## 14. Risk Matrix

| Risk | Severity | Current Status | Mitigation | Ticket |
|---|---|---|---|---|
| False Ok from submit_frame | Critical | Blocked (returns UnsupportedOperation) | Require audit before change | P0-061A, P0-061C |
| No consumer thread | Critical | No consumer exists | Design consumer loop | P0-060D, P0-060F |
| Output thread deadlock | High | N/A (no thread) | AtomicBool shutdown, no mutex in loop | P0-060D |
| Busy loop | High | N/A (no thread) | sleep(buffer/4) on empty/full | P0-060F |
| Backpressure | High | N/A (no producer) | Return WouldBlock, pipeline retries | P0-061B |
| Device lost | High | N/A (no device) | Detect HRESULT, break loop, surface error | P0-060F |
| Flush/stop race | Medium | N/A (no thread) | Shutdown signal, join guarantee | P0-060D |
| Close/join leak | Medium | N/A (no thread) | close() MUST join, timeout 5s | P0-060D |
| RingBuffer frame counting | Medium | Partial (stats exist) | Add rendered_frames to output thread stats | P0-060C |
| PlaybackCapabilities false-positive | Medium | Blocked (all false) | Require audit + rendered_frames proof | P0-062B |
| Cross-layer shortcut | Medium | Blocked (governance) | Strict tree-layer enforcement | All audits |
| File size > 220 | Low | All files OK | Split before implementation | All tickets |

---

## 15. Final Decision

### Next ticket does NOT allow:

- Changing `submit_frame` behavior
- Connecting `NativePipeline` to `WasapiOutputSink`
- Opening `PlaybackCapabilities`
- Starting a real output thread
- Opening a Windows audio device
- Claiming audible playback

### Next recommended ticket:

**P0-060C** — Output Thread State Types. Define `OutputThreadState` enum and `OutputThreadStats` struct as pure data types with no behavior. This is the foundation for all subsequent work.

### When submit_frame behavior change is allowed:

Only after ALL of:
1. Consumer thread exists and is tested (P0-060D, P0-060F)
2. Error mapping is audited (P0-061A)
3. A dedicated audit ticket (P0-061C) approves the change
4. `rendered_frames` stat exists and is populated
5. No false Ok risk remains

---

## 16. Final Safety Statement

This document is design-only.
No Rust production code changed.
No submit_frame behavior changed.
No NativePipeline integration was introduced.
PlaybackCapabilities remain closed.
No real audio output path was introduced.
No output thread was started.
No Windows audio device was opened.
Kivo Music remains in WASAPI scaffold stage.

---

*This document is the authoritative design reference for WASAPI output thread consumption. Implementation must follow the ticket queue defined in Section 13. No ticket may skip ahead or violate the prerequisite chain.*
