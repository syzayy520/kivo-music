//! Lifecycle transition validation rules.
//!
//! Pure function that validates lifecycle state transitions based on the documented state graph.

use crate::playback::output_wasapi::output_thread::runtime::state_machine::{
    Transition, TransitionError, TransitionResult,
};
use crate::playback::output_wasapi::output_thread::state::OutputThreadLifecycle;

/// Validate a lifecycle transition request.
///
/// Returns `TransitionResult::Applied` if the transition is valid,
/// `TransitionResult::AlreadyAtTarget` if from == to,
/// or `TransitionResult::Rejected` with `TransitionError::InvalidTransition` otherwise.
pub fn validate_transition(
    from: OutputThreadLifecycle,
    to: OutputThreadLifecycle,
) -> TransitionResult {
    let transition = Transition::new(from, to);

    if transition.is_identity() {
        return TransitionResult::AlreadyAtTarget;
    }

    let valid = match (from, to) {
        // NotStarted → Starting
        (OutputThreadLifecycle::NotStarted, OutputThreadLifecycle::Starting) => true,
        // Starting → Running
        (OutputThreadLifecycle::Starting, OutputThreadLifecycle::Running) => true,
        // Starting → Failed
        (OutputThreadLifecycle::Starting, OutputThreadLifecycle::Failed) => true,
        // Running → Draining
        (OutputThreadLifecycle::Running, OutputThreadLifecycle::Draining) => true,
        // Running → Stopping
        (OutputThreadLifecycle::Running, OutputThreadLifecycle::Stopping) => true,
        // Running → Failed
        (OutputThreadLifecycle::Running, OutputThreadLifecycle::Failed) => true,
        // Draining → Stopping
        (OutputThreadLifecycle::Draining, OutputThreadLifecycle::Stopping) => true,
        // Draining → Failed
        (OutputThreadLifecycle::Draining, OutputThreadLifecycle::Failed) => true,
        // Stopping → Stopped
        (OutputThreadLifecycle::Stopping, OutputThreadLifecycle::Stopped) => true,
        // Stopping → Failed
        (OutputThreadLifecycle::Stopping, OutputThreadLifecycle::Failed) => true,
        // Failed → NotStarted
        (OutputThreadLifecycle::Failed, OutputThreadLifecycle::NotStarted) => true,
        // Stopped → NotStarted
        (OutputThreadLifecycle::Stopped, OutputThreadLifecycle::NotStarted) => true,
        // All other transitions are invalid
        _ => false,
    };

    if valid {
        TransitionResult::Applied(transition)
    } else {
        TransitionResult::Rejected(transition, TransitionError::InvalidTransition)
    }
}
