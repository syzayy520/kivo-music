use super::super::real_transport::channel::OutputThreadRealTransportRecvResult;
use super::super::real_transport::command::OutputThreadRealTransportCommand;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::state::OutputThreadWorkerLoopState;
use super::step::{
    plan_worker_loop_step, OutputThreadWorkerLoopStepKind,
};
use super::super::worker_lifecycle::shutdown::OutputThreadWorkerShutdownRequest;

#[test]
fn empty_recv_yields_no_command() {
    let decision = plan_worker_loop_step(OutputThreadRealTransportRecvResult::Empty);
    assert_eq!(decision.kind, OutputThreadWorkerLoopStepKind::NoCommand);
    assert_eq!(decision.next_state, OutputThreadWorkerLoopState::NoCommand);
    assert_eq!(
        decision.shutdown_request,
        OutputThreadWorkerShutdownRequest::None
    );
    assert!(decision.should_continue);
}

#[test]
fn disconnected_recv_yields_transport_closed() {
    let decision = plan_worker_loop_step(OutputThreadRealTransportRecvResult::Disconnected);
    assert_eq!(
        decision.kind,
        OutputThreadWorkerLoopStepKind::Disconnected
    );
    assert_eq!(
        decision.next_state,
        OutputThreadWorkerLoopState::TransportClosed
    );
    assert_eq!(
        decision.shutdown_request,
        OutputThreadWorkerShutdownRequest::CloseTransport
    );
    assert!(!decision.should_continue);
}

#[test]
fn close_transport_command_yields_transport_closed() {
    let cmd = OutputThreadRealTransportCommand::CloseTransport;
    let decision = plan_worker_loop_step(OutputThreadRealTransportRecvResult::Command(cmd));
    assert_eq!(
        decision.kind,
        OutputThreadWorkerLoopStepKind::TransportClosed
    );
    assert_eq!(
        decision.next_state,
        OutputThreadWorkerLoopState::TransportClosed
    );
    assert_eq!(
        decision.shutdown_request,
        OutputThreadWorkerShutdownRequest::CloseTransport
    );
    assert!(!decision.should_continue);
}

#[test]
fn stop_intent_yields_stop_requested() {
    let cmd = OutputThreadRealTransportCommand::RuntimeIntent(OutputThreadRuntimeIntent::Stop);
    let decision = plan_worker_loop_step(OutputThreadRealTransportRecvResult::Command(cmd));
    assert_eq!(decision.kind, OutputThreadWorkerLoopStepKind::StopRequested);
    assert_eq!(
        decision.next_state,
        OutputThreadWorkerLoopState::StopRequested
    );
    assert_eq!(
        decision.shutdown_request,
        OutputThreadWorkerShutdownRequest::RequestStop
    );
    assert!(decision.should_continue);
}

#[test]
fn close_intent_yields_stop_requested() {
    let cmd = OutputThreadRealTransportCommand::RuntimeIntent(OutputThreadRuntimeIntent::Close);
    let decision = plan_worker_loop_step(OutputThreadRealTransportRecvResult::Command(cmd));
    assert_eq!(decision.kind, OutputThreadWorkerLoopStepKind::StopRequested);
    assert_eq!(
        decision.next_state,
        OutputThreadWorkerLoopState::StopRequested
    );
    assert!(decision.should_continue);
}

#[test]
fn non_shutdown_intent_yields_handled() {
    let cmd = OutputThreadRealTransportCommand::RuntimeIntent(OutputThreadRuntimeIntent::Start);
    let decision = plan_worker_loop_step(OutputThreadRealTransportRecvResult::Command(cmd));
    assert_eq!(
        decision.kind,
        OutputThreadWorkerLoopStepKind::RuntimeIntentHandled
    );
    assert_eq!(
        decision.next_state,
        OutputThreadWorkerLoopState::CommandHandled
    );
    assert_eq!(
        decision.shutdown_request,
        OutputThreadWorkerShutdownRequest::None
    );
    assert!(decision.should_continue);
}

#[test]
fn step_decision_never_implies_output_behavior() {
    let cases = [
        plan_worker_loop_step(OutputThreadRealTransportRecvResult::Empty),
        plan_worker_loop_step(OutputThreadRealTransportRecvResult::Disconnected),
        plan_worker_loop_step(OutputThreadRealTransportRecvResult::Command(
            OutputThreadRealTransportCommand::CloseTransport,
        )),
        plan_worker_loop_step(OutputThreadRealTransportRecvResult::Command(
            OutputThreadRealTransportCommand::RuntimeIntent(OutputThreadRuntimeIntent::Start),
        )),
        plan_worker_loop_step(OutputThreadRealTransportRecvResult::Command(
            OutputThreadRealTransportCommand::RuntimeIntent(OutputThreadRuntimeIntent::Stop),
        )),
    ];
    for decision in cases {
        let name = format!("{:?}", decision.kind);
        assert!(
            !name.to_lowercase().contains("output"),
            "step kind {:?}",
            decision.kind
        );
    }
}
