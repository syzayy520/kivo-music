use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::output_thread_worker_loop_step::OutputThreadWorkerLoopStepKind;
use super::output_thread_worker_runtime_decision::{
    OutputThreadWorkerRuntimeDecision, OutputThreadWorkerRuntimeDecisionKind,
};

#[test]
fn no_runtime_intent_does_not_use_queue_bridge() {
    let d = OutputThreadWorkerRuntimeDecision::no_runtime_intent(
        OutputThreadWorkerLoopStepKind::NoCommand,
        true,
    );
    assert!(!d.uses_queue_bridge());
    assert_eq!(
        d.kind,
        OutputThreadWorkerRuntimeDecisionKind::NoRuntimeIntent
    );
    assert!(d.observed_runtime_intent.is_none());
}

#[test]
fn runtime_intent_observed_records_intent() {
    let d = OutputThreadWorkerRuntimeDecision::runtime_intent_observed(
        OutputThreadRuntimeIntent::Start,
        OutputThreadWorkerLoopStepKind::RuntimeIntentHandled,
    );
    assert_eq!(
        d.kind,
        OutputThreadWorkerRuntimeDecisionKind::RuntimeIntentObserved
    );
    assert_eq!(
        d.observed_runtime_intent,
        Some(OutputThreadRuntimeIntent::Start)
    );
    assert!(!d.uses_queue_bridge());
    assert!(d.should_continue_scaffold);
}

#[test]
fn shutdown_observed_records_shutdown_intent() {
    let d = OutputThreadWorkerRuntimeDecision::shutdown_observed(
        OutputThreadRuntimeIntent::Stop,
        OutputThreadWorkerLoopStepKind::StopRequested,
    );
    assert_eq!(
        d.kind,
        OutputThreadWorkerRuntimeDecisionKind::RuntimeShutdownObserved
    );
    assert_eq!(
        d.observed_runtime_intent,
        Some(OutputThreadRuntimeIntent::Stop)
    );
    assert!(!d.uses_queue_bridge());
}

#[test]
fn transport_closed_does_not_use_queue_bridge() {
    let d = OutputThreadWorkerRuntimeDecision::transport_closed(
        OutputThreadWorkerLoopStepKind::TransportClosed,
    );
    assert_eq!(
        d.kind,
        OutputThreadWorkerRuntimeDecisionKind::WorkerTransportClosed
    );
    assert!(!d.uses_queue_bridge());
    assert!(!d.should_continue_scaffold);
    assert!(d.observed_runtime_intent.is_none());
}

#[test]
fn all_decisions_have_no_output_behavior() {
    let cases = [
        OutputThreadWorkerRuntimeDecision::no_runtime_intent(
            OutputThreadWorkerLoopStepKind::NoCommand,
            true,
        ),
        OutputThreadWorkerRuntimeDecision::runtime_intent_observed(
            OutputThreadRuntimeIntent::Start,
            OutputThreadWorkerLoopStepKind::RuntimeIntentHandled,
        ),
        OutputThreadWorkerRuntimeDecision::shutdown_observed(
            OutputThreadRuntimeIntent::Stop,
            OutputThreadWorkerLoopStepKind::StopRequested,
        ),
        OutputThreadWorkerRuntimeDecision::transport_closed(
            OutputThreadWorkerLoopStepKind::TransportClosed,
        ),
    ];
    for d in cases {
        assert!(d.has_no_output_behavior(), "kind {:?}", d.kind);
    }
}

#[test]
fn decision_is_copy_and_debug() {
    let a = OutputThreadWorkerRuntimeDecision::no_runtime_intent(
        OutputThreadWorkerLoopStepKind::NoCommand,
        true,
    );
    let b = a;
    assert_eq!(a, b);
    let _ = format!("{:?}", a);
}
