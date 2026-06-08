//! Thread handle type.
//!
//! Opaque handle identifying a spawned output thread. Does not hold
//! an actual JoinHandle — that is owned by the runtime. This is a
//! lightweight identifier only.

/// Opaque handle to a running output thread.
///
/// Used to correlate commands and events with a specific thread instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadHandle {
    /// Unique identifier for this thread instance.
    pub id: u64,
    /// Monotonic generation counter (incremented on restart).
    pub generation: u64,
}

impl Default for ThreadHandle {
    fn default() -> Self {
        Self {
            id: 0,
            generation: 0,
        }
    }
}

impl ThreadHandle {
    /// Create a new handle with the given id and generation.
    pub const fn new(id: u64, generation: u64) -> Self {
        Self { id, generation }
    }

    /// Check if this handle represents a valid (non-default) thread.
    pub const fn is_valid(&self) -> bool {
        self.id != 0
    }
}