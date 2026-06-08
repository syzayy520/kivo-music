//! Runtime state machine rules tests.
//!
//! Tests for lifecycle transition validation and command application rules.

use crate::playback::output_wasapi::output_thread::command::drain_command::DrainCommand;
use crate::playback::output_wasapi::output_thread::command::shutdown_command::ShutdownCommand;
use crate::playback::output_wasapi::output_thread::command::thread_command::ThreadCommand;
use crate::playback::output_wasapi::output_thread::runtime::state_machine::rules::{
    apply_command, validate_transition,
};
use crate::playback::output_wasapi::output_thread::runtime::state_machine::TransitionError;
use crate::playback::output_wasapi::output_thread::state::OutputThreadLifecycle;

type L = OutputThreadLifecycle;
type C = ThreadCommand;

// --- Lifecycle transition validation tests ---

#[test]
fn identity_transition_returns_already_at_target() {
    assert!(validate_transition(L::Running, L::Running).is_already_at_target());
}

#[test]
fn all_documented_transitions_are_applied() {
    let valid = [
        (L::NotStarted, L::Starting),
        (L::Starting, L::Running),
        (L::Starting, L::Failed),
        (L::Running, L::Draining),
        (L::Running, L::Stopping),
        (L::Running, L::Failed),
        (L::Draining, L::Stopping),
        (L::Draining, L::Failed),
        (L::Stopping, L::Stopped),
        (L::Stopping, L::Failed),
        (L::Failed, L::NotStarted),
        (L::Stopped, L::NotStarted),
    ];
    for (from, to) in valid {
        let result = validate_transition(from, to);
        assert!(result.is_applied(), "expected Applied for {from:?}→{to:?}");
    }
}

#[test]
fn undocumented_transition_is_rejected() {
    let result = validate_transition(L::NotStarted, L::Running);
    assert!(result.is_rejected());
    if let crate::playback::output_wasapi::output_thread::runtime::state_machine::TransitionResult::Rejected(_, err) =
        result
    {
        assert_eq!(err, TransitionError::InvalidTransition);
    }
}

// --- Command application tests ---

#[test]
fn shutdown_graceful_running_to_draining() {
    let r = apply_command(L::Running, C::Shutdown(ShutdownCommand::Graceful));
    assert!(r.is_applied());
}

#[test]
fn shutdown_graceful_draining_to_stopping() {
    let r = apply_command(L::Draining, C::Shutdown(ShutdownCommand::Graceful));
    assert!(r.is_applied());
}

#[test]
fn shutdown_graceful_invalid_state_rejected() {
    assert!(apply_command(L::NotStarted, C::Shutdown(ShutdownCommand::Graceful)).is_rejected());
}

#[test]
fn shutdown_immediate_from_running_and_draining() {
    assert!(apply_command(L::Running, C::Shutdown(ShutdownCommand::Immediate)).is_applied());
    assert!(apply_command(L::Draining, C::Shutdown(ShutdownCommand::Immediate)).is_applied());
}

#[test]
fn shutdown_with_timeout_running_to_draining() {
    let r = apply_command(L::Running, C::Shutdown(ShutdownCommand::WithTimeout(1000)));
    assert!(r.is_applied());
}

#[test]
fn drain_running_to_draining() {
    let r = apply_command(L::Running, C::Drain(DrainCommand::UntilEmpty));
    assert!(r.is_applied());
}

#[test]
fn drain_draining_is_already_at_target() {
    assert!(apply_command(L::Draining, C::Drain(DrainCommand::UntilEmpty)).is_already_at_target());
}

#[test]
fn flush_running_is_already_at_target() {
    assert!(apply_command(L::Running, C::Flush).is_already_at_target());
}

#[test]
fn flush_invalid_state_rejected() {
    assert!(apply_command(L::NotStarted, C::Flush).is_rejected());
}

#[test]
fn set_volume_running_is_already_at_target() {
    assert!(apply_command(L::Running, C::SetVolume(50)).is_already_at_target());
}

#[test]
fn pause_and_resume_running_is_already_at_target() {
    assert!(apply_command(L::Running, C::Pause).is_already_at_target());
    assert!(apply_command(L::Running, C::Resume).is_already_at_target());
}

#[test]
fn pause_invalid_state_rejected() {
    assert!(apply_command(L::NotStarted, C::Pause).is_rejected());
}

#[test]
fn volume_pause_resume_from_draining_rejected() {
    assert!(apply_command(L::Draining, C::Flush).is_rejected());
    assert!(apply_command(L::Draining, C::SetVolume(50)).is_rejected());
    assert!(apply_command(L::Draining, C::Pause).is_rejected());
    assert!(apply_command(L::Draining, C::Resume).is_rejected());
}

#[test]
fn commands_from_stopped_rejected() {
    assert!(apply_command(L::Stopped, C::Flush).is_rejected());
    assert!(apply_command(L::Stopped, C::SetVolume(50)).is_rejected());
    assert!(apply_command(L::Stopped, C::Pause).is_rejected());
}

#[test]
fn commands_from_failed_rejected() {
    assert!(apply_command(L::Failed, C::Flush).is_rejected());
    assert!(apply_command(L::Failed, C::SetVolume(50)).is_rejected());
    assert!(apply_command(L::Failed, C::Pause).is_rejected());
}
