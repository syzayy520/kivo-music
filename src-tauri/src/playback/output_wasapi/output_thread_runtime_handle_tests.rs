use super::output_thread_runtime_handle::OutputThreadRuntimeHandle;
use super::output_thread_runtime_id::{OutputThreadRuntimeGeneration, OutputThreadRuntimeId};
use super::output_thread_runtime_status::OutputThreadRuntimeStatus;
use super::output_thread_state::OutputThreadState;

#[test]
fn inactive_handle_is_not_attached() {
    let handle = OutputThreadRuntimeHandle::inactive();
    assert!(!handle.is_attached());
    assert!(!handle.is_running());
    assert!(!handle.can_accept_frames());
}

#[test]
fn new_handle_preserves_fields() {
    let id = OutputThreadRuntimeId::new(10);
    let gen = OutputThreadRuntimeGeneration::new(2);
    let status =
        OutputThreadRuntimeStatus::new(OutputThreadState::Running, true, true, false, true);
    let handle = OutputThreadRuntimeHandle::new(id, gen, status);
    assert_eq!(handle.id, id);
    assert_eq!(handle.generation, gen);
    assert_eq!(handle.status, status);
}

#[test]
fn running_handle_reports_running() {
    let status = OutputThreadRuntimeStatus::new(OutputThreadState::Running, true, true, true, true);
    let handle = OutputThreadRuntimeHandle::new(
        OutputThreadRuntimeId::new(1),
        OutputThreadRuntimeGeneration::new(0),
        status,
    );
    assert!(handle.is_running());
}

#[test]
fn handle_can_accept_frames_when_status_allows() {
    let status =
        OutputThreadRuntimeStatus::new(OutputThreadState::Running, true, true, false, true);
    let handle = OutputThreadRuntimeHandle::new(
        OutputThreadRuntimeId::new(1),
        OutputThreadRuntimeGeneration::new(0),
        status,
    );
    assert!(handle.can_accept_frames());
}

#[test]
fn with_status_replaces_status() {
    let handle = OutputThreadRuntimeHandle::inactive();
    let new_status =
        OutputThreadRuntimeStatus::new(OutputThreadState::Running, true, true, true, true);
    let updated = handle.with_status(new_status);
    assert_eq!(updated.status, new_status);
}

#[test]
fn with_state_replaces_state() {
    let handle = OutputThreadRuntimeHandle::inactive();
    let updated = handle.with_state(OutputThreadState::Running);
    assert_eq!(updated.status.state, OutputThreadState::Running);
}
