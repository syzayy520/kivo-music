//! Dispatch outcome type.
//!
//! Represents the result of a sink dispatch operation.
//! Pure data — no behavior, no IO, no WASAPI.

/// Outcome of a sink consumer dispatch operation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum DispatchOutcome {
    /// Request processed successfully.
    Success {
        /// Number of frames processed.
        frames_processed: u64,
        /// Number of bytes written.
        bytes_written: u64,
    },
    /// Request completed with silence fill.
    SilenceFilled {
        /// Number of silent frames written.
        frames_written: u64,
    },
    /// Request was skipped (not ready, buffer full).
    Skipped,
    /// No-op dispatch (idle cycle).
    #[default]
    Noop,
    /// Dispatch resulted in an error.
    Failed,
}
