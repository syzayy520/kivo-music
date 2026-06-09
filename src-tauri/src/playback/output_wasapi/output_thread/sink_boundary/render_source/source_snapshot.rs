//! Render source snapshot type.
//!
//! Pure data snapshot of a render source's state.
//! No behavior, no IO.

/// Snapshot of a render source's current state.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SourceSnapshot {
    /// Total requests accepted by the source.
    pub requests_accepted: u64,
    /// Total packets provided by the source.
    pub packets_provided: u64,
    /// Total frames read from the source.
    pub frames_read: u64,
    /// Total bytes read from the source.
    pub bytes_read: u64,
    /// Total errors encountered.
    pub errors: u64,
    /// Whether the source is fully exhausted.
    pub is_exhausted: bool,
    /// Whether the source is ready to serve requests.
    pub is_ready: bool,
}
