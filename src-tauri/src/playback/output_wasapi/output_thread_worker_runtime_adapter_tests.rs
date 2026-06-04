use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::output_thread_runtime_loop_state::OutputThreadRuntimeLoopState;
use super::output_thread_runtime_loop_step::OutputThreadRuntimeLoopStepAction;
use super::output_thread_worker_loop_step::{
    OutputThreadWorkerLoopStepDecision, OutputThreadWorkerLoopStepKind,
};
use super::output_thread_worker_runtime_adapter::{
    adapt_worker_step_to_runtime_loop, OutputThreadWorkerRuntimeAdapterResult,
};
use super::output_thread_worker_runtime_decision::OutputThreadWorkerRuntimeDecisionKind;
use super::output_thread_worker_shutdown::OutputThreadWorkerShutdownRequest;

fn make_worker_decision(kind: OutputThreadWorkerLoopStepKind) -> OutputThreadWorkerLoopStepDecision {
    OutputThreadWorkerLoopStepDecision {
        kind,
        next_state: super::output_thread_worker_loop_state::OutputThreadWorkerLoopState::Polling,
        shutdown_request: OutputThreadWorkerShutdownRequest::None,
        should_continue: true,
    }
}

#[test]
fn no_command_calls_runtime_loop_with_no_queue_result() {
    let result = adapt_worker_step_to_runtime_loop(
        make_worker_decision(OutputThreadWorkerLoopStepKind::NoCommand),
        None,
        OutputThreadRuntimeLoopState::Idle,
    );
    assert_eq!(
        result.worker_runtime_decision.kind,
        OutputThreadWorkerRuntimeDecisionKind::NoRuntimeIntent
    );
    assert!(!result.queue_bridge_used);
    assert!(!result.has_output_behavior);
}

#[test]
fn runtime_intent_observed_does_not_use_queue_bridge() {
    let result = adapt_worker_step_to_runtime_loop(
        make_worker_decision(OutputThreadWorkerLoopStepKind::RuntimeIntentHandled),
        Some(OutputThreadRuntimeIntent::Start),
        OutputThreadRuntimeLoopState::Idle,
    );
    assert_eq!(
        result.worker_runtime_decision.kind,
        OutputThreadWorkerRuntimeDecisionKind::RuntimeIntentObserved
    );
    assert!(!result.queue_bridge_used);
    assert!(!result.has_output_behavior);
}

#[test]
fn shutdown_intent_observed_does_not_use_queue_bridge() {
    let result = adapt_worker_step_to_runtime_loop(
        make_worker_decision(OutputThreadWorkerLoopStepKind::StopRequested),
        Some(OutputThreadRuntimeIntent::Stop),
        OutputThreadRuntimeLoopState::Active,
    );
    assert_eq!(
        result.worker_runtime_decision.kind,
        OutputThreadWorkerRuntimeDecisionKind::RuntimeShutdownObserved
    );
    assert!(!result.queue_bridge_used);
    assert!(!result.has_output_behavior);
}

#[test]
fn transport_closed_does_not_use_queue_bridge() {
    let result = adapt_worker_step_to_runtime_loop(
        make_worker_decision(OutputThreadWorkerLoopStepKind::TransportClosed),
        None,
        OutputThreadRuntimeLoopState::Active,
    );
    assert_eq!(
        result.worker_runtime_decision.kind,
        OutputThreadWorkerRuntimeDecisionKind::WorkerTransportClosed
    );
    assert!(!result.queue_bridge_used);
    assert!(!result.has_output_behavior);
    // Transport closed should produce an Exit action.
    assert_eq!(
        result.runtime_loop_decision.action,
        OutputThreadRuntimeLoopStepAction::MarkExited
    );
    assert!(!result.runtime_loop_decision.should_continue);
}

#[test]
fn adapter_result_has_no_output_behavior() {
    let cases = [
        adapt_worker_step_to_runtime_loop(
            make_worker_decision(OutputThreadWorkerLoopStepKind::NoCommand),
            None,
            OutputThreadRuntimeLoopState::Idle,
        ),
        adapt_worker_step_to_runtime_loop(
            make_worker_decision(OutputThreadWorkerLoopStepKind::RuntimeIntentHandled),
            Some(OutputThreadRuntimeIntent::Start),
            OutputThreadRuntimeLoopState::Idle,
        ),
        adapt_worker_step_to_runtime_loop(
            make_worker_decision(OutputThreadWorkerLoopStepKind::StopRequested),
            Some(OutputThreadRuntimeIntent::Stop),
            OutputThreadRuntimeLoopState::Active,
        ),
        adapt_worker_step_to_runtime_loop(
            make_worker_decision(OutputThreadWorkerLoopStepKind::TransportClosed),
            None,
            OutputThreadRuntimeLoopState::Active,
        ),
    ];
    for result in cases {
        assert!(
            !result.has_output_behavior,
            "kind {:?}",
            result.worker_runtime_decision.kind
        );
    }
}

#[test]
fn adapter_does_not_poll_transport_channel() {
    // This test verifies the adapter is pure — no channel needed.
    let result = adapt_worker_step_to_runtime_loop(
        make_worker_decision(OutputThreadWorkerLoopStepKind::NoCommand),
        None,
        OutputThreadRuntimeLoopState::Idle,
    );
    assert!(!result.queue_bridge_used);
}

#[test]
fn adapter_does_not_construct_queue_bridge_result() {
    // Verify queue_bridge_used is always false across all step kinds.
    let kinds = [
        OutputThreadWorkerLoopStepKind::NoCommand,
        OutputThreadWorkerLoopStepKind::RuntimeIntentHandled,
        OutputThreadWorkerLoopStepKind::StopRequested,
        OutputThreadWorkerLoopStepKind::TransportClosed,
        OutputThreadWorkerLoopStepKind::Disconnected,
    ];
    for kind in kinds {
        let result = adapt_worker_step_to_runtime_loop(
            make_worker_decision(kind),
            None,
            OutputThreadRuntimeLoopState::Idle,
        );
        assert!(
            !result.queue_bridge_used,
            "kind {:?}",
            kind
        );
    }
}

#[test]
fn idle_state_gets_stay_idle_action() {
    let result = adapt_worker_step_to_runtime_loop(
        make_worker_decision(OutputThreadWorkerLoopStepKind::NoCommand),
        None,
        OutputThreadRuntimeLoopState::Idle,
    );
    assert_eq!(
        result.runtime_loop_decision.action,
        OutputThreadRuntimeLoopStepAction::StayIdle
    );
    assert!(result.runtime_loop_decision.should_continue);
}
