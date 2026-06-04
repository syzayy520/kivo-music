use super::output_thread_runtime_loop_state::OutputThreadRuntimeLoopState;
use super::output_thread_runtime_loop_step::OutputThreadRuntimeLoopStepAction;
use super::output_thread_worker_runtime_queue_result_decision::{
    OutputThreadWorkerRuntimeQueueResultDecision, OutputThreadWorkerRuntimeQueueResultDecisionKind,
};

#[test]
fn accepted_result_decision_passes_from_queue_bridge() {
    let d = OutputThreadWorkerRuntimeQueueResultDecision::accepted(
        OutputThreadRuntimeLoopStepAction::EnterActive,
        OutputThreadRuntimeLoopState::Active,
        true,
    );
    assert!(d.passes_from_queue_bridge());
    assert_eq!(
        d.kind,
        OutputThreadWorkerRuntimeQueueResultDecisionKind::AcceptedResultHandled
    );
}

#[test]
fn rejected_result_decision_passes_from_queue_bridge() {
    let d = OutputThreadWorkerRuntimeQueueResultDecision::rejected(
        OutputThreadRuntimeLoopStepAction::IgnoreRejectedIntent,
        OutputThreadRuntimeLoopState::Active,
        true,
    );
    assert!(d.passes_from_queue_bridge());
    assert_eq!(
        d.kind,
        OutputThreadWorkerRuntimeQueueResultDecisionKind::RejectedResultHandled
    );
}

#[test]
fn decision_never_has_queue_ownership() {
    let cases = [
        OutputThreadWorkerRuntimeQueueResultDecision::accepted(
            OutputThreadRuntimeLoopStepAction::EnterActive,
            OutputThreadRuntimeLoopState::Active,
            true,
        ),
        OutputThreadWorkerRuntimeQueueResultDecision::rejected(
            OutputThreadRuntimeLoopStepAction::IgnoreRejectedIntent,
            OutputThreadRuntimeLoopState::Active,
            true,
        ),
    ];
    for d in cases {
        assert!(d.has_no_queue_ownership(), "kind {:?}", d.kind);
    }
}

#[test]
fn decision_never_has_output_behavior() {
    let cases = [
        OutputThreadWorkerRuntimeQueueResultDecision::accepted(
            OutputThreadRuntimeLoopStepAction::EnterActive,
            OutputThreadRuntimeLoopState::Active,
            true,
        ),
        OutputThreadWorkerRuntimeQueueResultDecision::rejected(
            OutputThreadRuntimeLoopStepAction::IgnoreRejectedIntent,
            OutputThreadRuntimeLoopState::Active,
            true,
        ),
    ];
    for d in cases {
        assert!(d.has_no_output_behavior(), "kind {:?}", d.kind);
    }
}
