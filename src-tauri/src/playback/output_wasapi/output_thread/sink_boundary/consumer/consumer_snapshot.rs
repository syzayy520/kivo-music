//! Consumer snapshot type.
//!
//! Pure data snapshot of a sink consumer's state.
//! No behavior, no IO.

/// Snapshot of a sink consumer's current state.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ConsumerSnapshot {
    /// Total requests processed.
    pub requests_processed: u64,
    /// Total frames rendered.
    pub frames_rendered: u64,
    /// Total bytes written.
    pub bytes_written: u64,
    /// Total errors encountered.
    pub errors: u64,
    /// Whether the consumer is currently ready.
    pub is_ready: bool,
}
