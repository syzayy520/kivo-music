/// Lifecycle state of the WASAPI output thread.
///
/// State machine:
/// ```text
/// Created → Running → Stopping → Stopped → Closed → Joined
///               ↑         │
///               └─────────┘ (pause/resume via silence fill)
/// ```
///
/// `Failed` is a terminal state reachable from any active state
/// when a panic or fatal error occurs.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum OutputThreadState {
    /// Thread has been spawned but has not yet entered the consumer loop.
    #[default]
    Created,
    /// Consumer loop is actively running.
    Running,
    /// Shutdown signal has been sent; loop is draining.
    Stopping,
    /// Consumer loop has exited normally.
    Stopped,
    /// `close()` has been called; resources are being released.
    Closed,
    /// `join()` has completed; thread handle is consumed.
    Joined,
    /// A panic or fatal error occurred; thread is no longer usable.
    Failed,
}

/// Statistics maintained by the output thread consumer loop.
///
/// All counters are monotonically increasing `u64` values.
/// They are zero at creation and accumulate over the thread lifetime.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub(crate) struct OutputThreadStats {
    /// Frames read from `RingBuffer` by the consumer.
    pub consumed_frames: u64,
    /// Frames confirmed written to the WASAPI render client
    /// via `GetBuffer` / `ReleaseBuffer`.
    pub rendered_frames: u64,
    /// Silence frames injected by the consumer when the `RingBuffer`
    /// was empty (underrun).
    pub silence_filled_frames: u64,
    /// Frames dropped due to backpressure or error.
    pub dropped_frames: u64,
    /// Number of `GetBuffer` / `ReleaseBuffer` failures.
    pub render_error_count: u64,
    /// Number of `AUDCLNT_E_DEVICE_INVALIDATED` events.
    pub device_lost_count: u64,
    /// Last error message from the consumer loop.
    pub last_output_thread_error: Option<String>,
    /// Last WASAPI render error message.
    pub last_render_error: Option<String>,
    /// Current lifecycle state of the output thread.
    pub output_thread_state: OutputThreadState,
}

/// Report sent by the output thread when it exits.
///
/// Carries the final stats snapshot and any terminal error
/// so the owning sink can surface it to the pipeline layer.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub(crate) struct OutputThreadReport {
    /// Final snapshot of thread statistics at exit.
    pub stats: OutputThreadStats,
    /// Error message if the thread exited due to a fatal error.
    pub error: Option<String>,
    /// Whether the thread exited because of a caught panic.
    pub panicked: bool,
}
