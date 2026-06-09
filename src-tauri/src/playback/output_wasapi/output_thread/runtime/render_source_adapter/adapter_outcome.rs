//! Adapter outcome type.
//!
//! Represents the result of a render source adapter operation.
//! Pure data — no behavior, no IO, no WASAPI.

/// Outcome of a render source adapter operation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum AdapterOutcome {
    /// Source provided a packet successfully.
    Packet {
        /// Number of frames provided.
        frames_provided: u64,
        /// Number of bytes read.
        bytes_read: u64,
    },
    /// Source is exhausted (end of stream).
    Exhausted,
    /// Source skipped the request (not ready or buffer full).
    Skipped,
    /// No-op adapter call (idle cycle).
    #[default]
    Noop,
    /// Adapter operation failed.
    Failed,
}
