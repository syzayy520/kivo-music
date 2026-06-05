use super::status::OutputThreadRuntimeStatus;
use super::super::output_thread_state::OutputThreadState;

#[test]
fn inactive_status_is_not_running() {
    let status = OutputThreadRuntimeStatus::inactive();
    assert!(!status.is_running());
    assert_eq!(status.state, OutputThreadState::Created);
}

#[test]
fn scaffold_ready_is_created() {
    let status = OutputThreadRuntimeStatus::scaffold_ready();
    assert_eq!(status.state, OutputThreadState::Created);
    assert!(!status.has_handle);
    assert!(!status.has_queue);
    assert!(!status.has_report_receiver);
    assert!(!status.has_render_target);
}

#[test]
fn running_attached_status_can_accept_frames() {
    let status = OutputThreadRuntimeStatus::new(
        OutputThreadState::Running,
        true,  // has_handle
        true,  // has_queue
        false, // has_report_receiver
        true,  // has_render_target
    );
    assert!(status.can_accept_frames());
}

#[test]
fn running_without_queue_cannot_accept_frames() {
    let status = OutputThreadRuntimeStatus::new(
        OutputThreadState::Running,
        true,  // has_handle
        false, // has_queue
        false, // has_report_receiver
        true,  // has_render_target
    );
    assert!(!status.can_accept_frames());
}

#[test]
fn stopped_status_can_close() {
    let status = OutputThreadRuntimeStatus::new(OutputThreadState::Stopped, true, true, true, true);
    assert!(status.can_close());
}

#[test]
fn fully_attached_requires_all_runtime_parts() {
    let status = OutputThreadRuntimeStatus::new(
        OutputThreadState::Running,
        true, // has_handle
        true, // has_queue
        true, // has_report_receiver
        true, // has_render_target
    );
    assert!(status.is_fully_attached());

    let missing = OutputThreadRuntimeStatus::new(
        OutputThreadState::Running,
        true,
        true,
        false, // missing report receiver
        true,
    );
    assert!(!missing.is_fully_attached());
}
