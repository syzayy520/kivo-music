//! Transition result type.
//!
//! Pure data enum representing the outcome of a transition attempt.
//! No behavior — only classification.

use crate::playback::output_wasapi::output_thread::runtime::state_machine::{
    transition::Transition, transition_error::TransitionError,
};

/// Outcome of attempting to apply a state transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionResult {
    /// The transition was successfully applied.
    Applied(Transition),
    /// The transition was rejected with a reason.
    Rejected(Transition, TransitionError),
    /// The current state already matches the target — no-op.
    AlreadyAtTarget,
}

impl TransitionResult {
    /// Check if the transition was applied.
    pub fn is_applied(self) -> bool {
        matches!(self, Self::Applied(_))
    }

    /// Check if the transition was rejected.
    pub fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected(_, _))
    }

    /// Check if the transition was a no-op.
    pub fn is_already_at_target(self) -> bool {
        matches!(self, Self::AlreadyAtTarget)
    }
}
