use super::super::output_thread_core::control::OutputThreadCommand;
use super::intent::OutputThreadRuntimeIntent;
use super::status::OutputThreadRuntimeStatus;
use super::super::output_thread_core::state::OutputThreadState;

#[test]
fn intent_maps_to_command() {
    assert_eq!(
        OutputThreadRuntimeIntent::Start.to_command(),
        OutputThreadCommand::Start
    );
    assert_eq!(
        OutputThreadRuntimeIntent::Pause.to_command(),
        OutputThreadCommand::Pause
    );
    assert_eq!(
        OutputThreadRuntimeIntent::Stop.to_command(),
        OutputThreadCommand::Stop
    );
    assert_eq!(
        OutputThreadRuntimeIntent::Close.to_command(),
        OutputThreadCommand::Close
    );
}

#[test]
fn stop_and_close_request_shutdown() {
    assert!(OutputThreadRuntimeIntent::Stop.requests_shutdown());
    assert!(OutputThreadRuntimeIntent::Close.requests_shutdown());
    assert!(!OutputThreadRuntimeIntent::Start.requests_shutdown());
    assert!(!OutputThreadRuntimeIntent::Pause.requests_shutdown());
}

#[test]
fn flush_and_reset_clear_buffer() {
    assert!(OutputThreadRuntimeIntent::Flush.clears_buffer());
    assert!(OutputThreadRuntimeIntent::ResetDevice.clears_buffer());
    assert!(!OutputThreadRuntimeIntent::Start.clears_buffer());
    assert!(!OutputThreadRuntimeIntent::Stop.clears_buffer());
}

#[test]
fn pause_requires_running() {
    assert!(OutputThreadRuntimeIntent::Pause.requires_running());
    assert!(OutputThreadRuntimeIntent::Resume.requires_running());
    assert!(OutputThreadRuntimeIntent::Flush.requires_running());
    assert!(OutputThreadRuntimeIntent::Stop.requires_running());
    assert!(!OutputThreadRuntimeIntent::Start.requires_running());
    assert!(!OutputThreadRuntimeIntent::Close.requires_running());
}

#[test]
fn close_can_apply_to_inactive_status() {
    let status = OutputThreadRuntimeStatus::inactive();
    assert!(OutputThreadRuntimeIntent::Close.can_apply_to(status));
}

#[test]
fn reset_can_apply_to_failed_status() {
    let status = OutputThreadRuntimeStatus::new(OutputThreadState::Failed, true, true, true, true);
    assert!(OutputThreadRuntimeIntent::ResetDevice.can_apply_to(status));
}

#[test]
fn start_can_apply_to_scaffold_ready_status() {
    let status = OutputThreadRuntimeStatus::scaffold_ready();
    assert!(OutputThreadRuntimeIntent::Start.can_apply_to(status));
}
