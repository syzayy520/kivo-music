# P0-041 Output Thread Boundary Design Report

**Date**: 2026-06-03  
**Status**: Read-only design report (Phase 3)  
**Scope**: Output Thread Boundary design for WASAPI real audio implementation

---

## Executive Summary

This report documents the design decisions for the Output Thread Boundary in Kivo Music's WASAPI audio pipeline. Based on analysis of the existing smoke test infrastructure (`start_stop`, `silent_loop`, `reset_boundary`), we propose a thread-safe, RAII-guarded output thread design that maintains the established patterns while enabling real audio playback.

---

## 20 Design Questions & Answers

### Q1: What is the Output Thread Boundary?

The Output Thread Boundary is the architectural layer that separates the main playback control thread from the real-time audio rendering thread. It owns the lifecycle of:
- `IAudioClient` (Windows audio session)
- `IAudioRenderClient` (buffer access)
- Audio thread (real-time rendering loop)
- Ring buffer (producer-consumer data flow)

**Key principle**: The boundary ensures thread-safe handoff of audio frames from the decoder pipeline to the WASAPI render loop.

---

### Q2: What threading model should be used?

**Design choice**: Single dedicated thread with message-passing control.

```
Main Thread ──────────────────────────────────────► Audio Thread
    │                                                    │
    │  Control commands (Open/Close/Pause/Resume/Stop)    │
    │  via crossbeam channel                             │
    ▼                                                    ▼
[OutputThreadController]                      [AudioRenderWorker]
```

**Rationale**:
- WASAPI callbacks are not used (prohibited in current scope)
- Single thread avoids COM apartment complexity
- Message-passing ensures deterministic state transitions

---

### Q3: How should the thread lifecycle be managed?

**Design**: RAII guard pattern (extending `StartedClientGuard`).

```rust
pub struct OutputThreadGuard {
    thread_handle: Option<JoinHandle<()>>,
    command_sender: Sender<ControlCommand>,
    state: Arc<Mutex<ThreadState>>,
}

impl Drop for OutputThreadGuard {
    fn drop(&mut self) {
        // Send Stop command, wait for acknowledgment
        // Join thread with timeout
        // Assert thread terminated
    }
}
```

**Pattern**: Same as `StartedClientGuard` — explicit Stop before cleanup, safety net on Drop.

---

### Q4: What control commands are needed?

```rust
enum ControlCommand {
    Open(AudioOpenParams),
    SubmitFrame(AudioOutputFrame),
    Pause,
    Resume,
    Flush,
    Stop,
    SetVolume(f32),
    SetMuted(bool),
    Shutdown,
}
```

Each command returns a `Result<OutputRuntimeStatus, PlaybackError>` via response channel.

---

### Q5: How should the ring buffer be designed?

**Design**: Lock-free SPSC (Single Producer, Single Consumer) ring buffer.

```rust
pub struct AudioRingBuffer {
    buffer: Vec<f32>,
    capacity_frames: usize,
    write_pos: AtomicUsize,
    read_pos: AtomicUsize,
    channels: usize,
}
```

**Capacity**: 2x `buffer_size_frames` (from `GetBufferSize`) to absorb timing jitter.

**Why SPSC**: Only one producer (decoder thread submitting frames) and one consumer (audio thread reading frames).

---

### Q6: How should the audio render loop work?

```rust
fn audio_render_loop(
    audio_client: IAudioClient,
    render_client: IAudioRenderClient,
    ring_buffer: &AudioRingBuffer,
    control_receiver: Receiver<ControlCommand>,
    state: Arc<Mutex<ThreadState>>,
) {
    // 1. Pre-fill buffer (GetBuffer + ReleaseBuffer SILENT)
    // 2. Start audio client
    // 3. Loop:
    //    a. Check control channel (non-blocking)
    //    b. GetCurrentPadding → available frames
    //    c. If available > 0:
    //       - GetBuffer(available)
    //       - Read from ring buffer
    //       - ReleaseBuffer(available)
    //    d. Sleep for ~1ms (or use event handle)
    // 4. Stop on Shutdown command
}
```

**Key**: No silent loop prohibition here — this is real playback, not smoke testing.

---

### Q7: How should state transitions be managed?

**Design**: State machine with atomic transitions.

```rust
enum ThreadState {
    Idle,
    Opening,
    Ready,
    Playing,
    Paused,
    Stopping,
    Stopped,
    Error(String),
}
```

**Invariant**: Only valid transitions allowed (e.g., `Ready → Playing`, not `Idle → Paused`).

---

### Q8: How should errors propagate from audio thread?

**Design**: Error channel + atomic error flag.

```rust
pub struct ThreadError {
    pub hresult: i32,
    pub message: String,
    pub operation: &'static str,
}
```

Audio thread sends errors via channel; main thread polls on next control command.

---

### Q9: How should the OutputSink trait be implemented?

```rust
impl OutputSink for WasapiOutputSink {
    fn open(&mut self, settings: &OutputSettings) -> PlaybackResult<OutputRuntimeStatus> {
        // 1. Create OutputThreadGuard
        // 2. Send Open command
        // 3. Wait for Ready state
        // 4. Return status
    }
    
    fn submit_frame(&mut self, frame: AudioOutputFrame) -> PlaybackResult<OutputRuntimeStatus> {
        // 1. Convert frame to f32 samples
        // 2. Write to ring buffer
        // 3. Return status
    }
    
    // ... other methods
}
```

---

### Q10: How should COM apartment be managed across threads?

**Design**: Each thread initializes its own COM apartment.

- Main thread: `COINIT_MULTITHREADED` (for device enumeration)
- Audio thread: `COINIT_APARTMENTTHREADED` (for WASAPI calls)

**Rationale**: WASAPI requires STA on some Windows versions; separate apartments avoid conflicts.

---

### Q11: How should device hotplug be handled?

**Design**: Endpoint change notification via `IMMNotificationClient`.

```rust
struct DeviceChangeHandler {
    current_endpoint: Arc<Mutex<Option<IMMDevice>>>,
    restart_sender: Sender<RestartCommand>,
}
```

**Scope**: P0-041 only detects hotplug; actual restart logic deferred to P0-042.

---

### Q12: How should format conversion work?

**Design**: Conversion in main thread before ring buffer.

```
Decoder output (PCM f32/i16) → Format converter → Ring buffer → Audio thread
```

**Converter capabilities**:
- Sample rate conversion (future: resampler)
- Channel mapping (mono → stereo, etc.)
- Sample format conversion (i16 → f32)

---

### Q13: How should volume control be implemented?

**Design**: Software volume in audio thread (before WASAPI write).

```rust
fn apply_volume(samples: &mut [f32], volume: f32, muted: bool) {
    let gain = if muted { 0.0 } else { volume };
    for sample in samples.iter_mut() {
        *sample *= gain;
    }
}
```

**Future**: Hardware volume via `ISimpleAudioVolume` if available.

---

### Q14: How should pause/resume work?

**Design**: Pause stops audio client; resume restarts.

```
Pause: Stop audio client, clear ring buffer, set state=Paused
Resume: Pre-fill buffer, Start audio client, set state=Playing
```

**Gapless**: Not in P0-041 scope; future design needed.

---

### Q15: How should flush work?

**Design**: Clear ring buffer, reset padding.

```rust
fn flush(&self) {
    // 1. Stop audio client
    // 2. Reset audio client (clears internal buffer)
    // 3. Clear ring buffer
    // 4. Set state=Ready
}
```

---

### Q16: How should the thread be stopped safely?

**Design**: Graceful shutdown with timeout.

```rust
fn stop_thread(&mut self) {
    // 1. Send Shutdown command
    // 2. Wait for thread acknowledgment (channel recv)
    // 3. Join with timeout (e.g., 100ms)
    // 4. If timeout: force-kill thread (unsafe, last resort)
}
```

**Pattern**: Same as `StartedClientGuard::stop()` — explicit stop before drop.

---

### Q17: What prohibited operations must be avoided?

Per existing smoke test boundaries:
- ❌ `IAudioClient::IsFormatSupported` (use GetMixFormat only)
- ❌ Callbacks (`IAudioClient::SetEventHandle`)
- ❌ Async runtime (tokio, async-std)
- ❌ Ring buffer in smoke tests (only in real implementation)
- ❌ Decoder integration in smoke tests

**Allowed in real implementation**:
- ✅ Ring buffer
- ✅ Audio thread
- ✅ Format conversion
- ✅ Volume control

---

### Q18: How should the report struct be extended?

```rust
pub struct WasapiOutputThreadReport {
    // Thread lifecycle
    pub thread_spawned: bool,
    pub thread_id: Option<u64>,
    pub thread_state: ThreadState,
    
    // Audio session
    pub audio_client_created: bool,
    pub render_client_created: bool,
    pub buffer_size_frames: Option<u32>,
    
    // Ring buffer
    pub ring_buffer_capacity: Option<usize>,
    pub ring_buffer_used: Option<usize>,
    
    // Timing
    pub open_duration_ms: Option<u64>,
    pub submit_frame_count: u64,
    pub underrun_count: u64,
    
    // Error tracking
    pub last_error: Option<ThreadError>,
    pub error_count: u64,
}
```

---

### Q19: How should testing be structured?

**Unit tests**:
- State machine transitions
- Ring buffer operations
- Format conversion

**Integration tests** (opt-in, Windows-only):
- `output_thread_smoke_tests`: Open → SubmitFrame → Close
- `output_thread_pause_tests`: Open → Play → Pause → Resume → Close
- `output_thread_error_tests`: Invalid device, buffer underrun

**Pattern**: Same as `start_stop_tests` — env opt-in, RAII guards, detailed reports.

---

### Q20: What are the implementation phases?

**Phase 1 (P0-042)**: Basic output thread
- Thread spawn/join
- Ring buffer (SPSC)
- Open → SubmitFrame → Close

**Phase 2 (P0-043)**: Control commands
- Pause/Resume
- Flush
- SetVolume/SetMuted

**Phase 3 (P0-044)**: Error handling
- Device hotplug detection
- Error recovery
- Timeout handling

**Phase 4 (P0-045)**: Performance optimization
- Event-driven rendering (instead of polling)
- Buffer size tuning
- Latency measurement

---

## Design Principles

1. **RAII everywhere**: All resources guarded by Drop impls
2. **Thread safety**: Arc<Mutex<>> for shared state, channels for commands
3. **Graceful degradation**: Errors don't panic, return Result
4. **Observable**: Detailed reports for debugging
5. **Testable**: Each component independently testable
6. **Incremental**: Build on existing smoke test infrastructure

---

## Appendix: Existing Infrastructure

### Smoke Test Boundaries (from codebase analysis)

| Boundary | Start/Stop | GetCurrentPadding | Reset | Loop | Ring Buffer |
|----------|------------|-------------------|-------|------|-------------|
| `start_stop` | ✅ | ❌ | ❌ | ❌ | ❌ |
| `silent_loop` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `reset_boundary` | ✅ | ✅ | ✅ | ❌ | ❌ |
| **Output Thread** | ✅ | ✅ | ✅ | ✅ | ✅ |

### RAII Guards (from `guards.rs`)

- `ComApartment`: COM init/cleanup
- `MixFormatGuard`: WAVEFORMATEX pointer
- `BufferGuard`: IAudioRenderClient buffer
- `StartedClientGuard`: IAudioClient Start/Stop

**New guard needed**: `OutputThreadGuard` (thread lifecycle)

---

## Conclusion

The Output Thread Boundary design extends the existing RAII-guarded, thread-safe patterns established in the smoke tests. By using a dedicated audio thread with message-passing control and a lock-free ring buffer, we can achieve low-latency audio playback while maintaining the safety guarantees of the current architecture.

**Next steps**: Implement Phase 1 (P0-042) with basic output thread and ring buffer.
