use super::super::real_transport::channel::OutputThreadRealTransportChannel;
use super::super::real_transport::command::OutputThreadRealTransportCommand;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::runner::{
    run_worker_loop_skeleton, OutputThreadWorkerLoopRunConfig,
};
use super::state::OutputThreadWorkerLoopState;

fn config(max_steps: usize) -> OutputThreadWorkerLoopRunConfig {
    OutputThreadWorkerLoopRunConfig {
        max_steps,
        initial_state: OutputThreadWorkerLoopState::NotStarted,
    }
}

#[test]
fn max_steps_zero_does_not_poll() {
    let channel = OutputThreadRealTransportChannel::new();
    let result = run_worker_loop_skeleton(&channel, config(0));
    assert_eq!(result.report.completed_steps, 0);
    assert!(!result.stopped_early);
}

#[test]
fn empty_channel_polls_once_and_reports_empty() {
    let channel = OutputThreadRealTransportChannel::new();
    let result = run_worker_loop_skeleton(&channel, config(1));
    assert_eq!(result.report.completed_steps, 1);
    assert_eq!(result.report.empty_polls, 1);
    assert_eq!(result.report.commands_handled, 0);
}

#[test]
fn close_transport_stops_early() {
    let channel = OutputThreadRealTransportChannel::new();
    channel
        .send_command(OutputThreadRealTransportCommand::CloseTransport)
        .unwrap();
    let result = run_worker_loop_skeleton(&channel, config(100));
    assert!(result.stopped_early);
    assert_eq!(result.report.completed_steps, 1);
    assert!(result.report.stopped_by_close_transport);
}

#[test]
fn runtime_intent_is_handled_once() {
    let channel = OutputThreadRealTransportChannel::new();
    channel
        .send_command(OutputThreadRealTransportCommand::RuntimeIntent(
            OutputThreadRuntimeIntent::Start,
        ))
        .unwrap();
    let result = run_worker_loop_skeleton(&channel, config(5));
    assert_eq!(result.report.commands_handled, 1);
    assert!(!result.stopped_early);
}

#[test]
fn max_steps_limits_number_of_polls() {
    let channel = OutputThreadRealTransportChannel::new();
    let result = run_worker_loop_skeleton(&channel, config(3));
    assert_eq!(result.report.completed_steps, 3);
    assert_eq!(result.report.empty_polls, 3);
}

#[test]
fn runner_borrows_channel_without_closing_it() {
    let channel = OutputThreadRealTransportChannel::new();
    channel
        .send_command(OutputThreadRealTransportCommand::RuntimeIntent(
            OutputThreadRuntimeIntent::Start,
        ))
        .unwrap();
    let _result = run_worker_loop_skeleton(&channel, config(1));
    // Channel is still usable after runner returns.
    let status = channel.status_report();
    assert!(status.has_sender);
    assert!(status.has_receiver);
}

#[test]
fn runner_never_blocks_on_recv() {
    // If runner used blocking recv, this test would hang on an empty channel.
    let channel = OutputThreadRealTransportChannel::new();
    let result = run_worker_loop_skeleton(&channel, config(50));
    assert_eq!(result.report.completed_steps, 50);
    assert_eq!(result.report.empty_polls, 50);
}

#[test]
fn runner_does_not_model_output_behavior() {
    let channel = OutputThreadRealTransportChannel::new();
    let result = run_worker_loop_skeleton(&channel, config(5));
    assert!(result.report.has_no_output_behavior());
}

#[test]
fn multiple_commands_consumed_in_order() {
    let channel = OutputThreadRealTransportChannel::new();
    channel
        .send_command(OutputThreadRealTransportCommand::RuntimeIntent(
            OutputThreadRuntimeIntent::Start,
        ))
        .unwrap();
    channel
        .send_command(OutputThreadRealTransportCommand::CloseTransport)
        .unwrap();
    let result = run_worker_loop_skeleton(&channel, config(100));
    assert!(result.stopped_early);
    assert_eq!(result.report.commands_handled, 1);
    assert!(result.report.stopped_by_close_transport);
}
