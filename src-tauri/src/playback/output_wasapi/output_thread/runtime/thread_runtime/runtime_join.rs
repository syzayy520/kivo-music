//! Runtime join logic.
//!
//! Pure functions for join validation and state transitions.

use crate::playback::output_wasapi::output_thread::runtime::thread_handle::ThreadHandle;
use crate::playback::output_wasapi::output_thread::runtime::thread_runtime::runtime_handle::{
    JoinState, RuntimeHandle,
};

/// Reasons a join request can be rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JoinRejectReason {
    /// The handle is not in a terminal state.
    NotTerminal,
    /// A join has already been requested or completed.
    AlreadyRequested,
}

/// Outcome of a join computation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoinOutcome {
    /// The handle can be joined.
    CanJoin(ThreadHandle),
    /// The join was rejected.
    Rejected(JoinRejectReason),
}

/// Validate whether a join can be requested on the given handle.
pub fn validate_join(handle: &RuntimeHandle) -> Result<(), JoinRejectReason> {
    if !handle.is_terminal() {
        return Err(JoinRejectReason::NotTerminal);
    }
    if handle.join_state != JoinState::NotRequested {
        return Err(JoinRejectReason::AlreadyRequested);
    }
    Ok(())
}

/// Compute the join outcome for a given handle.
pub fn compute_join_outcome(handle: &RuntimeHandle) -> JoinOutcome {
    match validate_join(handle) {
        Ok(()) => JoinOutcome::CanJoin(handle.thread_handle),
        Err(reason) => JoinOutcome::Rejected(reason),
    }
}

/// Mark a handle as having a join requested.
///
/// Returns an error if the join cannot be requested.
pub fn mark_join_requested(handle: &mut RuntimeHandle) -> Result<(), JoinRejectReason> {
    validate_join(handle)?;
    handle.join_state = JoinState::Requested;
    Ok(())
}

/// Mark a join as completed.
///
/// Updates the join state to Completed.
pub fn complete_join(handle: &mut RuntimeHandle) {
    handle.join_state = JoinState::Completed;
}
