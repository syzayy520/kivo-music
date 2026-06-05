use super::super::output_thread_core::control::OutputThreadCommand;
use super::handle::OutputThreadRuntimeHandle;
use super::lifecycle::{
    apply_runtime_command, OutputThreadRuntimeLifecycleError,
};
use super::status::OutputThreadRuntimeStatus;
use super::super::output_thread_core::state::OutputThreadState;

fn handle_with_state(state: OutputThreadState) -> OutputThreadRuntimeHandle {
    OutputThreadRuntimeHandle::new(
        super::id::OutputThreadRuntimeId::new(1),
        super::id::OutputThreadRuntimeGeneration::new(0),
        OutputThreadRuntimeStatus::new(state, true, true, true, true),
    )
}

#[test]
fn start_from_created_moves_to_running() {
    let handle = handle_with_state(OutputThreadState::Created);
    let result = apply_runtime_command(handle, OutputThreadCommand::Start).unwrap();
    assert_eq!(result.status.state, OutputThreadState::Running);
}

#[test]
fn start_from_running_fails() {
    let handle = handle_with_state(OutputThreadState::Running);
    let result = apply_runtime_command(handle, OutputThreadCommand::Start);
    assert_eq!(result, Err(OutputThreadRuntimeLifecycleError::CannotStart));
}

#[test]
fn stop_from_running_moves_to_stopping() {
    let handle = handle_with_state(OutputThreadState::Running);
    let result = apply_runtime_command(handle, OutputThreadCommand::Stop).unwrap();
    assert_eq!(result.status.state, OutputThreadState::Stopping);
}

#[test]
fn stop_from_created_fails() {
    let handle = handle_with_state(OutputThreadState::Created);
    let result = apply_runtime_command(handle, OutputThreadCommand::Stop);
    assert_eq!(result, Err(OutputThreadRuntimeLifecycleError::CannotStop));
}

#[test]
fn close_from_created_moves_to_closed() {
    let handle = handle_with_state(OutputThreadState::Created);
    let result = apply_runtime_command(handle, OutputThreadCommand::Close).unwrap();
    assert_eq!(result.status.state, OutputThreadState::Closed);
}

#[test]
fn close_from_running_moves_to_closed() {
    let handle = handle_with_state(OutputThreadState::Running);
    let result = apply_runtime_command(handle, OutputThreadCommand::Close).unwrap();
    assert_eq!(result.status.state, OutputThreadState::Closed);
}

#[test]
fn reset_from_failed_moves_to_created() {
    let handle = handle_with_state(OutputThreadState::Failed);
    let result = apply_runtime_command(handle, OutputThreadCommand::ResetDevice).unwrap();
    assert_eq!(result.status.state, OutputThreadState::Created);
}

#[test]
fn pause_from_created_fails() {
    let handle = handle_with_state(OutputThreadState::Created);
    let result = apply_runtime_command(handle, OutputThreadCommand::Pause);
    assert_eq!(result, Err(OutputThreadRuntimeLifecycleError::CannotPause));
}

#[test]
fn flush_from_running_keeps_running() {
    let handle = handle_with_state(OutputThreadState::Running);
    let result = apply_runtime_command(handle, OutputThreadCommand::Flush).unwrap();
    assert_eq!(result.status.state, OutputThreadState::Running);
}
