//! Thread runtime lifecycle tests.
//!
//! Verifies full lifecycle flows through the thread runtime components:
//! spawn → starting → running → draining → stopping → stopped.

use crate::playback::output_wasapi::output_thread::command::{ShutdownCommand, ThreadCommand};
use crate::playback::output_wasapi::output_thread::config::ThreadConfig;
use crate::playback::output_wasapi::output_thread::runtime::spawn::spawn_request::SpawnRequest;
use crate::playback::output_wasapi::output_thread::runtime::thread_handle::ThreadHandle;
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::{
    execute_loop_step, LoopResult, LoopState,
};
use crate::playback::output_wasapi::output_thread::runtime::thread_runtime::{
    complete_join, compute_join_outcome, create_runtime_handle, mark_join_requested, JoinOutcome,
    JoinState, RuntimeHandle,
};
use crate::playback::output_wasapi::output_thread::state::thread_lifecycle::OutputThreadLifecycle;

// ── Full Lifecycle Flow Tests ────────────────────────────────────────

#[test]
fn graceful_shutdown_lifecycle_flow() {
    // Spawn → Starting → Running → Draining → Stopping → Stopped
    let request = SpawnRequest::new(ThreadHandle::new(1, 1), ThreadConfig::default());
    let mut handle = create_runtime_handle(&request);
    assert_eq!(handle.lifecycle, OutputThreadLifecycle::Starting);

    // Starting → Running
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Running);
    handle.lifecycle = OutputThreadLifecycle::Running;

    // Running → Draining (graceful shutdown)
    let outcome = execute_loop_step(
        &state,
        Some(ThreadCommand::Shutdown(ShutdownCommand::Graceful)),
        100,
    );
    assert_eq!(outcome.result, LoopResult::Continue);
    assert_eq!(outcome.state.lifecycle, OutputThreadLifecycle::Draining);

    // Simulate thread completing drain: Draining → Stopping → Stopped
    handle.lifecycle = OutputThreadLifecycle::Stopped;

    // Verify handle is terminal and can be joined
    assert!(handle.is_terminal());
    assert!(handle.can_request_join());

    // Request and complete join
    assert!(mark_join_requested(&mut handle).is_ok());
    assert_eq!(handle.join_state, JoinState::Requested);
    complete_join(&mut handle);
    assert_eq!(handle.join_state, JoinState::Completed);
}

#[test]
fn immediate_shutdown_lifecycle_flow() {
    // Spawn → Starting → Running → Stopping → Stopped
    let request = SpawnRequest::new(ThreadHandle::new(2, 1), ThreadConfig::default());
    let mut handle = create_runtime_handle(&request);
    assert_eq!(handle.lifecycle, OutputThreadLifecycle::Starting);

    // Starting → Running
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Running);
    handle.lifecycle = OutputThreadLifecycle::Running;

    // Running → Stopping (immediate shutdown)
    let outcome = execute_loop_step(
        &state,
        Some(ThreadCommand::Shutdown(ShutdownCommand::Immediate)),
        100,
    );
    assert_eq!(outcome.result, LoopResult::Continue);
    assert_eq!(outcome.state.lifecycle, OutputThreadLifecycle::Stopping);

    // Simulate thread completing: Stopping → Stopped
    handle.lifecycle = OutputThreadLifecycle::Stopped;

    assert!(handle.is_terminal());
    assert!(handle.can_request_join());
}

#[test]
fn failed_state_prevents_join() {
    let handle = RuntimeHandle::new(ThreadHandle::new(3, 1), OutputThreadLifecycle::Failed);
    let outcome = compute_join_outcome(&handle);
    // Failed is terminal, so join should be allowed
    assert_eq!(outcome, JoinOutcome::CanJoin(ThreadHandle::new(3, 1)));
}

#[test]
fn non_terminal_state_rejects_join() {
    let handle = RuntimeHandle::new(ThreadHandle::new(4, 1), OutputThreadLifecycle::Running);
    let outcome = compute_join_outcome(&handle);
    assert_eq!(outcome, JoinOutcome::Rejected(
        crate::playback::output_wasapi::output_thread::runtime::thread_runtime::JoinRejectReason::NotTerminal
    ));
}

// ── LoopState Lifecycle Integration ──────────────────────────────────

#[test]
fn loop_state_tracks_lifecycle_through_transitions() {
    let mut state = LoopState::new();
    assert_eq!(state.lifecycle, OutputThreadLifecycle::NotStarted);

    // NotStarted → Starting
    state.transition_to(OutputThreadLifecycle::Starting);
    assert_eq!(state.lifecycle, OutputThreadLifecycle::Starting);
    assert!(state.is_active());

    // Starting → Running
    state.transition_to(OutputThreadLifecycle::Running);
    assert_eq!(state.lifecycle, OutputThreadLifecycle::Running);
    assert!(state.is_active());

    // Running → Draining
    state.transition_to(OutputThreadLifecycle::Draining);
    assert_eq!(state.lifecycle, OutputThreadLifecycle::Draining);

    // Draining → Stopping
    state.transition_to(OutputThreadLifecycle::Stopping);
    assert_eq!(state.lifecycle, OutputThreadLifecycle::Stopping);

    // Stopping → Stopped
    state.transition_to(OutputThreadLifecycle::Stopped);
    assert_eq!(state.lifecycle, OutputThreadLifecycle::Stopped);
    assert!(state.is_terminal());
    assert!(!state.is_active());
}

#[test]
fn loop_state_step_count_accurate_through_lifecycle() {
    let mut state = LoopState::new();
    assert_eq!(state.step_count, 0);

    // Each transition increments step_count
    state.transition_to(OutputThreadLifecycle::Starting);
    assert_eq!(state.step_count, 1);

    state.transition_to(OutputThreadLifecycle::Running);
    assert_eq!(state.step_count, 2);

    // Idle steps also increment step_count
    state.record_idle();
    assert_eq!(state.step_count, 3);
    assert_eq!(state.idle_count, 1);

    state.record_idle();
    assert_eq!(state.step_count, 4);
    assert_eq!(state.idle_count, 2);
}

// ── Multiple Handle Independence ─────────────────────────────────────

#[test]
fn multiple_handles_are_independent() {
    let req1 = SpawnRequest::new(ThreadHandle::new(10, 1), ThreadConfig::default());
    let req2 = SpawnRequest::new(ThreadHandle::new(20, 1), ThreadConfig::default());

    let mut h1 = create_runtime_handle(&req1);
    let h2 = create_runtime_handle(&req2);

    h1.lifecycle = OutputThreadLifecycle::Stopped;
    assert_eq!(h2.lifecycle, OutputThreadLifecycle::Starting);

    assert!(mark_join_requested(&mut h1).is_ok());
    assert_eq!(h1.join_state, JoinState::Requested);
    assert_eq!(h2.join_state, JoinState::NotRequested);
}
