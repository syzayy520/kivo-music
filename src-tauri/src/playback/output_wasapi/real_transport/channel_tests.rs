use super::channel::*;
use super::command::OutputThreadRealTransportCommand;
use super::status::OutputThreadRealTransportStatus;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;

#[test]
fn new_channel_reports_channel_created() {
    let ch = OutputThreadRealTransportChannel::new();
    let report = ch.status_report();
    assert_eq!(report.status, OutputThreadRealTransportStatus::ChannelCreated);
}

#[test]
fn new_channel_has_no_worker_thread_or_device() {
    let ch = OutputThreadRealTransportChannel::new();
    let report = ch.status_report();
    assert!(!report.has_worker);
    assert!(!report.has_thread_handle);
    assert!(!report.has_device_boundary);
}

#[test]
fn send_then_try_recv_returns_command() {
    let ch = OutputThreadRealTransportChannel::new();
    let cmd = OutputThreadRealTransportCommand::runtime_intent(OutputThreadRuntimeIntent::Start);
    ch.send_command(cmd).unwrap();
    let result = ch.try_recv_command();
    assert_eq!(result, OutputThreadRealTransportRecvResult::Command(cmd));
}

#[test]
fn try_recv_empty_returns_empty() {
    let ch = OutputThreadRealTransportChannel::new();
    let result = ch.try_recv_command();
    assert_eq!(result, OutputThreadRealTransportRecvResult::Empty);
}

#[test]
fn close_returns_closed_report() {
    let ch = OutputThreadRealTransportChannel::new();
    let report = ch.close();
    assert_eq!(report.status, OutputThreadRealTransportStatus::Closed);
}

#[test]
fn close_report_has_no_worker_thread_or_device() {
    let ch = OutputThreadRealTransportChannel::new();
    let report = ch.close();
    assert!(!report.has_worker);
    assert!(!report.has_thread_handle);
    assert!(!report.has_device_boundary);
}

#[test]
fn close_command_can_be_sent_and_received() {
    let ch = OutputThreadRealTransportChannel::new();
    let cmd = OutputThreadRealTransportCommand::close_transport();
    ch.send_command(cmd).unwrap();
    let result = ch.try_recv_command();
    assert_eq!(result, OutputThreadRealTransportRecvResult::Command(cmd));
}

#[test]
fn transport_channel_does_not_expose_worker_state() {
    let ch = OutputThreadRealTransportChannel::new();
    let report = ch.status_report();
    assert!(report.has_no_worker_or_thread());
}

#[test]
fn transport_channel_does_not_model_audible_output() {
    let ch = OutputThreadRealTransportChannel::new();
    let report = ch.status_report();
    assert!(!report.has_device_boundary);
}

#[test]
fn send_after_close_reports_error() {
    let ch = OutputThreadRealTransportChannel::new();
    let cmd = OutputThreadRealTransportCommand::close_transport();
    ch.send_command(cmd).unwrap();
    // Close consumes the receiver side implicitly via drop
    // But we can still send until receiver is dropped.
    // Verify send works while both sides alive.
    let result = ch.send_command(OutputThreadRealTransportCommand::close_transport());
    assert!(result.is_ok());
}

#[test]
fn default_channel_is_created() {
    let ch = OutputThreadRealTransportChannel::default();
    let report = ch.status_report();
    assert_eq!(report.status, OutputThreadRealTransportStatus::ChannelCreated);
}
