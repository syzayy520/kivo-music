//! Spawn result type.
//!
//! Pure data enum representing the outcome of a spawn attempt.
//! No behavior — only classification.

use crate::playback::output_wasapi::output_thread::runtime::thread_handle::ThreadHandle;

/// Outcome of attempting to spawn an output thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpawnResult {
    /// The thread was successfully spawned.
    Spawned {
        /// Handle identifying the spawned thread.
        handle: ThreadHandle,
    },
    /// The spawn was rejected.
    Rejected {
        /// Reason for rejection.
        reason: SpawnRejectReason,
    },
}

/// Reasons a spawn request may be rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpawnRejectReason {
    /// A thread with this handle already exists.
    AlreadyExists,
    /// The system cannot allocate a new thread.
    ResourceExhausted,
    /// The configuration is invalid.
    InvalidConfig,
}
