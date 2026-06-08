//! Runtime handle type.
//!
//! Tracks the lifecycle and join state of a spawned output thread.
//! Pure data — no OS thread references.

use crate::playback::output_wasapi::output_thread::runtime::thread_handle::ThreadHandle;
use crate::playback::output_wasapi::output_thread::state::thread_lifecycle::OutputThreadLifecycle;

/// Join state of a runtime handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoinState {
    /// No join has been requested.
    NotRequested,
    /// A join has been requested but not yet completed.
    Requested,
    /// The join has completed (success or failure).
    Completed,
}

/// Runtime-managed handle for a spawned output thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHandle {
    /// The underlying thread handle identifier.
    pub thread_handle: ThreadHandle,
    /// Current lifecycle state.
    pub lifecycle: OutputThreadLifecycle,
    /// Current join state.
    pub join_state: JoinState,
}

impl RuntimeHandle {
    /// Create a new runtime handle in the given lifecycle state.
    pub fn new(thread_handle: ThreadHandle, lifecycle: OutputThreadLifecycle) -> Self {
        Self {
            thread_handle,
            lifecycle,
            join_state: JoinState::NotRequested,
        }
    }

    /// Check if the handle is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        self.lifecycle.is_terminal()
    }

    /// Check if a join can be requested.
    pub fn can_request_join(&self) -> bool {
        self.join_state == JoinState::NotRequested && self.is_terminal()
    }
}
