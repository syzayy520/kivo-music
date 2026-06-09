//! Thread loop skeleton tests.
//!
//! Verifies that the thread loop correctly processes commands,
//! manages idle steps, and respects terminal states.

use crate::playback::output_wasapi::output_thread::command::{ShutdownCommand, ThreadCommand};
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::{
    execute_loop_step, LoopResult, LoopState,
};
use crate::playback::output_wasapi::output_thread::state::thread_lifecycle::OutputThreadLifecycle;

// ── LoopState Tests ────────────────────────────────────────────────────

#[test]
fn loop_state_default_starts_not_started() {
    let state = LoopState::default();
    assert_eq!(state.lifecycle, OutputThreadLifecycle::NotStarted);
    assert_eq!(state.idle_count, 0);
    assert_eq!(state.step_count, 0);
}

#[test]
fn loop_state_transition_to_resets_idle_count() {
    let mut state = LoopState::new();
    state.record_idle();
    state.record_idle();
    assert_eq!(state.idle_count, 2);

    state.transition_to(OutputThreadLifecycle::Starting);
    assert_eq!(state.idle_count, 0);
    assert_eq!(state.lifecycle, OutputThreadLifecycle::Starting);
}

#[test]
fn loop_state_transition_to_same_does_not_reset_idle() {
    let mut state = LoopState::with_lifecycle(OutputThreadLifecycle::Running);
    state.record_idle();
    assert_eq!(state.idle_count, 1);

    state.transition_to(OutputThreadLifecycle::Running);
    assert_eq!(state.idle_count, 1);
}

#[test]
fn loop_state_is_terminal_for_stopped() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Stopped);
    assert!(state.is_terminal());
}

#[test]
fn loop_state_is_terminal_for_failed() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Failed);
    assert!(state.is_terminal());
}

#[test]
fn loop_state_is_active_for_running() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Running);
    assert!(state.is_active());
}

#[test]
fn loop_state_is_not_active_for_stopped() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Stopped);
    assert!(!state.is_active());
}

// ── Loop Step Tests ────────────────────────────────────────────────────

#[test]
fn loop_step_with_command_applies_transition() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Running);
    let outcome = execute_loop_step(&state, Some(ThreadCommand::Pause), 100);

    assert_eq!(outcome.result, LoopResult::Continue);
    assert_eq!(outcome.state.lifecycle, OutputThreadLifecycle::Running);
}

#[test]
fn loop_step_idle_increments_count() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Running);
    let outcome = execute_loop_step(&state, None, 100);

    assert_eq!(outcome.result, LoopResult::Continue);
    assert_eq!(outcome.state.idle_count, 1);
}

#[test]
fn loop_step_idle_stops_at_max() {
    let state = LoopState {
        lifecycle: OutputThreadLifecycle::Running,
        idle_count: 99,
        step_count: 100,
    };
    let outcome = execute_loop_step(&state, None, 100);

    assert_eq!(outcome.result, LoopResult::Stop);
}

#[test]
fn loop_step_terminal_state_stops_immediately() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Stopped);
    let outcome = execute_loop_step(&state, Some(ThreadCommand::Pause), 100);

    assert_eq!(outcome.result, LoopResult::Stop);
}

#[test]
fn loop_step_shutdown_graceful_transitions_to_draining() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Running);
    let outcome = execute_loop_step(
        &state,
        Some(ThreadCommand::Shutdown(ShutdownCommand::Graceful)),
        100,
    );

    assert_eq!(outcome.result, LoopResult::Continue);
    assert_eq!(outcome.state.lifecycle, OutputThreadLifecycle::Draining);
}

#[test]
fn loop_step_shutdown_immediate_transitions_to_stopping() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Running);
    let outcome = execute_loop_step(
        &state,
        Some(ThreadCommand::Shutdown(ShutdownCommand::Immediate)),
        100,
    );

    assert_eq!(outcome.result, LoopResult::Continue);
    assert_eq!(outcome.state.lifecycle, OutputThreadLifecycle::Stopping);
}

// ── LoopResult Tests ───────────────────────────────────────────────────

#[test]
fn loop_result_continue_equals_itself() {
    assert_eq!(LoopResult::Continue, LoopResult::Continue);
}

#[test]
fn loop_result_stop_equals_itself() {
    assert_eq!(LoopResult::Stop, LoopResult::Stop);
}

#[test]
fn loop_result_error_with_same_message_are_equal() {
    assert_eq!(
        LoopResult::Error("test".to_string()),
        LoopResult::Error("test".to_string())
    );
}

#[test]
fn loop_result_error_with_different_messages_are_not_equal() {
    assert_ne!(
        LoopResult::Error("a".to_string()),
        LoopResult::Error("b".to_string())
    );
}
