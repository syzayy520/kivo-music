//! Thread spawn runtime skeleton tests.
//!
//! Verifies RuntimeHandle creation, join validation, and state transitions.

use crate::playback::output_wasapi::output_thread::config::ThreadConfig;
use crate::playback::output_wasapi::output_thread::runtime::spawn::spawn_request::SpawnRequest;
use crate::playback::output_wasapi::output_thread::runtime::thread_handle::ThreadHandle;
use crate::playback::output_wasapi::output_thread::runtime::thread_runtime::{
    complete_join, compute_join_outcome, create_runtime_handle, mark_join_requested, JoinOutcome,
    JoinRejectReason, JoinState, RuntimeHandle,
};
use crate::playback::output_wasapi::output_thread::state::thread_lifecycle::OutputThreadLifecycle;

// ── RuntimeHandle Tests ──────────────────────────────────────────────

#[test]
fn runtime_handle_new_stores_fields() {
    let handle = ThreadHandle::new(1, 2);
    let rh = RuntimeHandle::new(handle, OutputThreadLifecycle::Running);
    assert_eq!(rh.thread_handle, handle);
    assert_eq!(rh.lifecycle, OutputThreadLifecycle::Running);
    assert_eq!(rh.join_state, JoinState::NotRequested);
}

#[test]
fn runtime_handle_is_terminal_for_stopped() {
    let rh = RuntimeHandle::new(ThreadHandle::new(1, 1), OutputThreadLifecycle::Stopped);
    assert!(rh.is_terminal());
}

#[test]
fn runtime_handle_is_not_terminal_for_running() {
    let rh = RuntimeHandle::new(ThreadHandle::new(1, 1), OutputThreadLifecycle::Running);
    assert!(!rh.is_terminal());
}

#[test]
fn runtime_handle_can_request_join_when_terminal() {
    let rh = RuntimeHandle::new(ThreadHandle::new(1, 1), OutputThreadLifecycle::Stopped);
    assert!(rh.can_request_join());
}

#[test]
fn runtime_handle_cannot_request_join_when_not_terminal() {
    let rh = RuntimeHandle::new(ThreadHandle::new(1, 1), OutputThreadLifecycle::Running);
    assert!(!rh.can_request_join());
}

#[test]
fn runtime_handle_cannot_request_join_when_already_requested() {
    let mut rh = RuntimeHandle::new(ThreadHandle::new(1, 1), OutputThreadLifecycle::Stopped);
    rh.join_state = JoinState::Requested;
    assert!(!rh.can_request_join());
}

// ── create_runtime_handle Tests ──────────────────────────────────────

#[test]
fn create_runtime_handle_auto_start_sets_starting() {
    let request = SpawnRequest::new(ThreadHandle::new(1, 1), ThreadConfig::default());
    let rh = create_runtime_handle(&request);
    assert_eq!(rh.lifecycle, OutputThreadLifecycle::Starting);
}

#[test]
fn create_runtime_handle_no_auto_start_sets_not_started() {
    let request =
        SpawnRequest::new(ThreadHandle::new(1, 1), ThreadConfig::default()).with_auto_start(false);
    let rh = create_runtime_handle(&request);
    assert_eq!(rh.lifecycle, OutputThreadLifecycle::NotStarted);
}

// ── compute_join_outcome Tests ───────────────────────────────────────

#[test]
fn compute_join_outcome_can_join_when_terminal() {
    let rh = RuntimeHandle::new(ThreadHandle::new(5, 1), OutputThreadLifecycle::Stopped);
    let outcome = compute_join_outcome(&rh);
    assert_eq!(outcome, JoinOutcome::CanJoin(ThreadHandle::new(5, 1)));
}

#[test]
fn compute_join_outcome_rejected_when_not_terminal() {
    let rh = RuntimeHandle::new(ThreadHandle::new(1, 1), OutputThreadLifecycle::Running);
    let outcome = compute_join_outcome(&rh);
    assert_eq!(
        outcome,
        JoinOutcome::Rejected(JoinRejectReason::NotTerminal)
    );
}

#[test]
fn compute_join_outcome_rejected_when_already_requested() {
    let mut rh = RuntimeHandle::new(ThreadHandle::new(1, 1), OutputThreadLifecycle::Stopped);
    rh.join_state = JoinState::Requested;
    let outcome = compute_join_outcome(&rh);
    assert_eq!(
        outcome,
        JoinOutcome::Rejected(JoinRejectReason::AlreadyRequested)
    );
}

// ── mark_join_requested Tests ────────────────────────────────────────

#[test]
fn mark_join_requested_succeeds_when_terminal() {
    let mut rh = RuntimeHandle::new(ThreadHandle::new(1, 1), OutputThreadLifecycle::Stopped);
    assert!(mark_join_requested(&mut rh).is_ok());
    assert_eq!(rh.join_state, JoinState::Requested);
}

#[test]
fn mark_join_requested_fails_when_not_terminal() {
    let mut rh = RuntimeHandle::new(ThreadHandle::new(1, 1), OutputThreadLifecycle::Running);
    assert_eq!(
        mark_join_requested(&mut rh),
        Err(JoinRejectReason::NotTerminal)
    );
    assert_eq!(rh.join_state, JoinState::NotRequested);
}

#[test]
fn mark_join_requested_fails_when_already_requested() {
    let mut rh = RuntimeHandle::new(ThreadHandle::new(1, 1), OutputThreadLifecycle::Stopped);
    rh.join_state = JoinState::Requested;
    assert_eq!(
        mark_join_requested(&mut rh),
        Err(JoinRejectReason::AlreadyRequested)
    );
}

// ── complete_join Tests ──────────────────────────────────────────────

#[test]
fn complete_join_sets_completed() {
    let mut rh = RuntimeHandle::new(ThreadHandle::new(1, 1), OutputThreadLifecycle::Stopped);
    rh.join_state = JoinState::Requested;
    complete_join(&mut rh);
    assert_eq!(rh.join_state, JoinState::Completed);
}
