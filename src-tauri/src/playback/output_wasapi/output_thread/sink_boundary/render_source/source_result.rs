//! Render source result type.
//!
//! Represents the outcome of a render source processing a request.
//! Pure data — no behavior, no IO.

/// Outcome of a render source processing a request.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum RenderSourceResult {
    /// Audio packet provided successfully.
    Packet {
        /// Number of frames actually provided.
        frames_provided: u64,
        /// Number of bytes read from source.
        bytes_read: u64,
    },
    /// Source is exhausted (end of stream).
    Exhausted,
    /// Request skipped (e.g., source not ready, buffer empty).
    Skipped,
    /// No-op result for idle cycle.
    #[default]
    Noop,
}
