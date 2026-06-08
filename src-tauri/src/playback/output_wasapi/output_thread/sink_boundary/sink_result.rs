//! Sink result type.
//!
//! Represents the outcome of a sink consumer processing a request.
//! Pure data — no behavior, no IO.

/// Outcome of a sink consumer processing a request.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum SinkResult {
    /// Request processed successfully.
    Success {
        /// Number of frames actually processed.
        frames_processed: u64,
        /// Number of bytes written.
        bytes_written: u64,
    },
    /// Request completed with silence fill.
    SilenceFilled {
        /// Number of silent frames written.
        frames_written: u64,
    },
    /// Request skipped (e.g., buffer full, not ready).
    Skipped,
    /// No-op result for idle cycle.
    #[default]
    Noop,
}
