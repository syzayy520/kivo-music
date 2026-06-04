use super::output_thread_control::{OutputThreadCommand, OutputThreadControlSnapshot};
use super::output_thread_state::OutputThreadState;

// ---------------------------------------------------------------------------
// OutputThreadCommand — pure intent classification
// ---------------------------------------------------------------------------

#[test]
fn start_command_requires_no_running_thread() {
    assert!(!OutputThreadCommand::Start.requires_running_thread());
}

#[test]
fn pause_command_requires_running_thread() {
    assert!(OutputThreadCommand::Pause.requires_running_thread());
}

#[test]
fn resume_command_requires_running_thread() {
    assert!(OutputThreadCommand::Resume.requires_running_thread());
}

#[test]
fn flush_command_requires_running_thread_and_clears_buffer() {
    assert!(OutputThreadCommand::Flush.requires_running_thread());
    assert!(OutputThreadCommand::Flush.clears_buffer());
}

#[test]
fn stop_command_requests_shutdown() {
    assert!(OutputThreadCommand::Stop.requests_shutdown());
}

#[test]
fn close_command_requests_shutdown_and_clears_buffer() {
    assert!(OutputThreadCommand::Close.requests_shutdown());
    assert!(OutputThreadCommand::Close.clears_buffer());
}

#[test]
fn reset_device_command_requests_shutdown_and_clears_buffer() {
    assert!(OutputThreadCommand::ResetDevice.requests_shutdown());
    assert!(OutputThreadCommand::ResetDevice.clears_buffer());
}

// ---------------------------------------------------------------------------
// OutputThreadControlSnapshot — default and frame acceptance
// ---------------------------------------------------------------------------

#[test]
fn default_control_snapshot_is_created_and_idle() {
    let snap = OutputThreadControlSnapshot::default();
    assert_eq!(snap.state, OutputThreadState::Created);
    assert!(!snap.paused);
    assert!(!snap.shutdown_requested);
    assert!(!snap.flush_requested);
}

#[test]
fn running_unpaused_snapshot_can_accept_frames() {
    let snap = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    };
    assert!(snap.can_accept_frames());
}

#[test]
fn paused_snapshot_cannot_accept_frames() {
    let snap = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: true,
        shutdown_requested: false,
        flush_requested: false,
    };
    assert!(!snap.can_accept_frames());
}

#[test]
fn shutdown_snapshot_cannot_accept_frames() {
    let snap = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: true,
        flush_requested: false,
    };
    assert!(!snap.can_accept_frames());
}

#[test]
fn flush_snapshot_cannot_accept_frames() {
    let snap = OutputThreadControlSnapshot {
        state: OutputThreadState::Running,
        paused: false,
        shutdown_requested: false,
        flush_requested: true,
    };
    assert!(!snap.can_accept_frames());
}

#[test]
fn non_running_snapshot_cannot_accept_frames() {
    let snap = OutputThreadControlSnapshot {
        state: OutputThreadState::Created,
        paused: false,
        shutdown_requested: false,
        flush_requested: false,
    };
    assert!(!snap.can_accept_frames());
}
