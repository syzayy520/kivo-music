/// Error classification for WASAPI output thread operations.
///
/// These are pure classification codes — no error trait impl,
/// no conversion to pipeline errors, no Windows API references.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadErrorKind {
    /// Consumer thread panicked inside `catch_unwind`.
    ThreadPanic,
    /// Shutdown timed out before thread exited.
    ShutdownTimeout,
    /// Thread is not running when an operation requires it.
    OutputThreadNotRunning,
    /// Ring buffer is not prepared or not available.
    RingBufferNotReady,
    /// Buffer is full; write would block.
    BufferFull,
    /// Audio device is not ready or not available.
    DeviceNotReady,
    /// `AUDCLNT_E_DEVICE_INVALIDATED` or equivalent.
    DeviceLost,
    /// Render buffer operation failure.
    RenderFailed,
    /// Audio format not supported by device.
    UnsupportedFormat,
    /// Sink is closed; no further operations allowed.
    Closed,
}

impl OutputThreadErrorKind {
    /// Stable string code for logging and diagnostics.
    #[allow(dead_code)]
    pub(crate) fn as_code(self) -> &'static str {
        match self {
            Self::ThreadPanic => "thread_panic",
            Self::ShutdownTimeout => "shutdown_timeout",
            Self::OutputThreadNotRunning => "output_thread_not_running",
            Self::RingBufferNotReady => "ring_buffer_not_ready",
            Self::BufferFull => "buffer_full",
            Self::DeviceNotReady => "device_not_ready",
            Self::DeviceLost => "device_lost",
            Self::RenderFailed => "render_failed",
            Self::UnsupportedFormat => "unsupported_format",
            Self::Closed => "closed",
        }
    }

    /// Whether this error relates to the audio device.
    #[allow(dead_code)]
    pub(crate) fn is_device_error(self) -> bool {
        matches!(
            self,
            Self::DeviceNotReady | Self::DeviceLost | Self::RenderFailed | Self::UnsupportedFormat
        )
    }

    /// Whether this error relates to thread lifecycle.
    #[allow(dead_code)]
    pub(crate) fn is_lifecycle_error(self) -> bool {
        matches!(
            self,
            Self::ThreadPanic
                | Self::ShutdownTimeout
                | Self::OutputThreadNotRunning
                | Self::RingBufferNotReady
                | Self::Closed
        )
    }

    /// Whether this error is a backpressure signal.
    #[allow(dead_code)]
    pub(crate) fn is_backpressure(self) -> bool {
        matches!(self, Self::BufferFull)
    }
}

/// Summary of an output thread error with a human-readable message.
///
/// Pure data — no conversion to pipeline errors, no Windows mapping.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OutputThreadErrorSummary {
    pub kind: OutputThreadErrorKind,
    pub message: String,
}

impl OutputThreadErrorSummary {
    /// Create a new error summary.
    #[allow(dead_code)]
    pub(crate) fn new(kind: OutputThreadErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}
