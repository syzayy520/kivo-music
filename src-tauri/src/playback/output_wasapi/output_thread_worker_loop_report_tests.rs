use super::output_thread_worker_loop_report::OutputThreadWorkerLoopReport;
use super::output_thread_worker_loop_state::OutputThreadWorkerLoopState;
use super::output_thread_worker_loop_step::{
    OutputThreadWorkerLoopStepDecision, OutputThreadWorkerLoopStepKind,
};
use super::output_thread_worker_shutdown::OutputThreadWorkerShutdownRequest;

fn make_decision(kind: OutputThreadWorkerLoopStepKind) -> OutputThreadWorkerLoopStepDecision {
    OutputThreadWorkerLoopStepDecision {
        kind,
        next_state: OutputThreadWorkerLoopState::Polling,
        shutdown_request: OutputThreadWorkerShutdownRequest::None,
        should_continue: true,
    }
}

#[test]
fn empty_report_has_zero_steps() {
    let report = OutputThreadWorkerLoopReport::empty(OutputThreadWorkerLoopState::NotStarted);
    assert_eq!(report.completed_steps, 0);
    assert_eq!(report.commands_handled, 0);
    assert_eq!(report.empty_polls, 0);
    assert!(!report.stopped_by_close_transport);
    assert_eq!(report.final_state, OutputThreadWorkerLoopState::NotStarted);
}

#[test]
fn record_empty_step_increments_empty_polls() {
    let report = OutputThreadWorkerLoopReport::empty(OutputThreadWorkerLoopState::NotStarted);
    let updated = report.record_step(make_decision(OutputThreadWorkerLoopStepKind::NoCommand));
    assert_eq!(updated.completed_steps, 1);
    assert_eq!(updated.empty_polls, 1);
    assert_eq!(updated.commands_handled, 0);
}

#[test]
fn record_runtime_command_increments_commands_handled() {
    let report = OutputThreadWorkerLoopReport::empty(OutputThreadWorkerLoopState::NotStarted);
    let updated = report.record_step(make_decision(
        OutputThreadWorkerLoopStepKind::RuntimeIntentHandled,
    ));
    assert_eq!(updated.completed_steps, 1);
    assert_eq!(updated.commands_handled, 1);
    assert_eq!(updated.empty_polls, 0);
}

#[test]
fn record_stop_requested_increments_commands_handled() {
    let report = OutputThreadWorkerLoopReport::empty(OutputThreadWorkerLoopState::NotStarted);
    let updated = report.record_step(make_decision(
        OutputThreadWorkerLoopStepKind::StopRequested,
    ));
    assert_eq!(updated.commands_handled, 1);
}

#[test]
fn record_close_transport_marks_stopped_by_close_transport() {
    let report = OutputThreadWorkerLoopReport::empty(OutputThreadWorkerLoopState::NotStarted);
    let updated = report.record_step(make_decision(
        OutputThreadWorkerLoopStepKind::TransportClosed,
    ));
    assert!(updated.stopped_by_close_transport);
}

#[test]
fn record_disconnected_marks_stopped_by_close_transport() {
    let report = OutputThreadWorkerLoopReport::empty(OutputThreadWorkerLoopState::NotStarted);
    let updated = report.record_step(make_decision(
        OutputThreadWorkerLoopStepKind::Disconnected,
    ));
    assert!(updated.stopped_by_close_transport);
}

#[test]
fn report_has_no_output_behavior() {
    let report = OutputThreadWorkerLoopReport::empty(OutputThreadWorkerLoopState::NotStarted);
    assert!(report.has_no_output_behavior());
}

#[test]
fn terminal_state_propagated_to_report() {
    let report = OutputThreadWorkerLoopReport::empty(OutputThreadWorkerLoopState::NotStarted);
    let decision = OutputThreadWorkerLoopStepDecision {
        kind: OutputThreadWorkerLoopStepKind::TransportClosed,
        next_state: OutputThreadWorkerLoopState::TransportClosed,
        shutdown_request: OutputThreadWorkerShutdownRequest::CloseTransport,
        should_continue: false,
    };
    let updated = report.record_step(decision);
    assert!(updated.is_terminal());
}

#[test]
fn report_is_copy_and_debug() {
    let a = OutputThreadWorkerLoopReport::empty(OutputThreadWorkerLoopState::NotStarted);
    let b = a;
    assert_eq!(a, b);
    let _ = format!("{:?}", a);
}
