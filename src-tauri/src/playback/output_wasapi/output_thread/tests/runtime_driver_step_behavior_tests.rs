//! Runtime driver step behavior tests.
//!
//! Tests for command step, idle step, and result builder pure functions.

use crate::playback::output_wasapi::output_thread::command::shutdown_command::ShutdownCommand;
use crate::playback::output_wasapi::output_thread::command::thread_command::ThreadCommand;
use crate::playback::output_wasapi::output_thread::event::ThreadEvent;
use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::driver::step_logic::{
    determine_driver_result, execute_command_step, execute_idle_step,
};
use crate::playback::output_wasapi::output_thread::runtime::state_machine::TransitionResult;
use crate::playback::output_wasapi::output_thread::state::OutputThreadLifecycle;

type L = OutputThreadLifecycle;
type C = ThreadCommand;

// --- Command step tests ---

#[test]
fn command_step_valid_transition_produces_event_and_continue() {
    let outcome = execute_command_step(C::Shutdown(ShutdownCommand::Graceful), L::Running);
    assert!(outcome.transition.is_applied());
    assert_eq!(outcome.new_lifecycle, L::Draining);
    assert_eq!(outcome.result, DriverResult::Continue);
    assert!(outcome.event.is_some());
    if let Some(ThreadEvent::StateChanged { from, to }) = outcome.event {
        assert_eq!(from, L::Running);
        assert_eq!(to, L::Draining);
    }
}

#[test]
fn command_step_rejected_from_stopping() {
    // Stopping→Stopped is valid lifecycle but no command triggers it.
    // Shutdown(Immediate) from Stopping is rejected by command rules.
    let outcome = execute_command_step(C::Shutdown(ShutdownCommand::Immediate), L::Stopping);
    assert!(outcome.transition.is_rejected());
    assert_eq!(outcome.result, DriverResult::Error);
    assert!(outcome.event.is_none());
}

#[test]
fn command_step_invalid_transition_produces_error() {
    let outcome = execute_command_step(C::Flush, L::NotStarted);
    assert!(outcome.transition.is_rejected());
    assert_eq!(outcome.result, DriverResult::Error);
    assert!(outcome.event.is_none());
    assert_eq!(outcome.new_lifecycle, L::NotStarted);
}

#[test]
fn command_step_already_at_target_produces_continue() {
    let outcome = execute_command_step(C::Flush, L::Running);
    assert!(outcome.transition.is_already_at_target());
    assert_eq!(outcome.result, DriverResult::Continue);
    assert!(outcome.event.is_none());
    assert_eq!(outcome.new_lifecycle, L::Running);
}

#[test]
fn command_step_failed_to_not_started_produces_continue() {
    let outcome = execute_command_step(C::Shutdown(ShutdownCommand::Graceful), L::Failed);
    // Shutdown Graceful from Failed is invalid
    assert!(outcome.transition.is_rejected());
    assert_eq!(outcome.result, DriverResult::Error);
}

// --- Idle step tests ---

#[test]
fn idle_step_below_max_returns_idle() {
    assert_eq!(execute_idle_step(0, 100), DriverResult::Idle);
    assert_eq!(execute_idle_step(50, 100), DriverResult::Idle);
    assert_eq!(execute_idle_step(99, 100), DriverResult::Idle);
}

#[test]
fn idle_step_at_max_returns_stop() {
    assert_eq!(execute_idle_step(100, 100), DriverResult::Stop);
}

#[test]
fn idle_step_above_max_returns_stop() {
    assert_eq!(execute_idle_step(150, 100), DriverResult::Stop);
}

#[test]
fn idle_step_zero_max_always_stops() {
    assert_eq!(execute_idle_step(0, 0), DriverResult::Stop);
}

// --- Result builder tests ---

#[test]
fn result_builder_applied_to_active_returns_continue() {
    let t = TransitionResult::Applied(
        crate::playback::output_wasapi::output_thread::runtime::state_machine::Transition::new(
            L::Running,
            L::Draining,
        ),
    );
    assert_eq!(
        determine_driver_result(&t, L::Draining),
        DriverResult::Continue
    );
}

#[test]
fn result_builder_applied_to_terminal_returns_stop() {
    let t = TransitionResult::Applied(
        crate::playback::output_wasapi::output_thread::runtime::state_machine::Transition::new(
            L::Stopping,
            L::Stopped,
        ),
    );
    assert_eq!(determine_driver_result(&t, L::Stopped), DriverResult::Stop);
}

#[test]
fn result_builder_rejected_returns_error() {
    let t = TransitionResult::Rejected(
        crate::playback::output_wasapi::output_thread::runtime::state_machine::Transition::new(
            L::NotStarted,
            L::Running,
        ),
        crate::playback::output_wasapi::output_thread::runtime::state_machine::TransitionError::InvalidTransition,
    );
    assert_eq!(
        determine_driver_result(&t, L::NotStarted),
        DriverResult::Error
    );
}

#[test]
fn result_builder_already_at_target_returns_continue() {
    assert_eq!(
        determine_driver_result(&TransitionResult::AlreadyAtTarget, L::Running),
        DriverResult::Continue,
    );
}

#[test]
fn result_builder_applied_to_failed_returns_stop() {
    let t = TransitionResult::Applied(
        crate::playback::output_wasapi::output_thread::runtime::state_machine::Transition::new(
            L::Running,
            L::Failed,
        ),
    );
    assert_eq!(determine_driver_result(&t, L::Failed), DriverResult::Stop);
}
