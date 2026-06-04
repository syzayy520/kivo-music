use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::runtime_queue_bridge::result::{
    OutputThreadRuntimeQueueBridgeAccepted, OutputThreadRuntimeQueueBridgeRejected,
    OutputThreadRuntimeQueueBridgeResult,
};
use super::output_thread_worker_queue_bridge_decision::{
    OutputThreadWorkerQueueBridgeDecision, OutputThreadWorkerQueueBridgeDecisionKind,
};

fn make_accepted_result() -> OutputThreadRuntimeQueueBridgeResult {
    OutputThreadRuntimeQueueBridgeResult::Accepted(OutputThreadRuntimeQueueBridgeAccepted {
        entry: super::runtime_queue::entry::OutputThreadRuntimeQueueEntry::new(
            super::output_thread_runtime_id::OutputThreadRuntimeGeneration::default(),
            OutputThreadRuntimeIntent::Start,
            1,
        ),
        queue_state: super::runtime_queue::state::OutputThreadRuntimeQueueState::empty(),
        projection: super::runtime_queue_bridge::projection::OutputThreadRuntimeQueueBridgeProjection {
            runtime_state: super::output_thread_state::OutputThreadState::Created,
            pending_count: 0,
            accepted_count: 0,
            rejected_count: 0,
            last_sequence: 0,
            queue_closed: false,
            queue_has_capacity: true,
            runtime_can_accept_frames: false,
            bridge_can_accept_intents: true,
        },
    })
}

fn make_rejected_result() -> OutputThreadRuntimeQueueBridgeResult {
    OutputThreadRuntimeQueueBridgeResult::Rejected(OutputThreadRuntimeQueueBridgeRejected {
        reason: super::runtime_queue::result::OutputThreadRuntimeQueueRejectReason::Full,
        queue_state: super::runtime_queue::state::OutputThreadRuntimeQueueState::empty(),
        projection: super::runtime_queue_bridge::projection::OutputThreadRuntimeQueueBridgeProjection {
            runtime_state: super::output_thread_state::OutputThreadState::Running,
            pending_count: 32,
            accepted_count: 0,
            rejected_count: 0,
            last_sequence: 0,
            queue_closed: false,
            queue_has_capacity: false,
            runtime_can_accept_frames: true,
            bridge_can_accept_intents: false,
        },
    })
}

#[test]
fn accepted_decision_uses_queue_bridge() {
    let d = OutputThreadWorkerQueueBridgeDecision::from_bridge_result(
        OutputThreadRuntimeIntent::Start,
        make_accepted_result(),
    );
    assert!(d.uses_queue_bridge());
    assert_eq!(d.kind, OutputThreadWorkerQueueBridgeDecisionKind::IntentAccepted);
    assert!(d.accepted);
    assert!(!d.rejected);
}

#[test]
fn rejected_decision_uses_queue_bridge() {
    let d = OutputThreadWorkerQueueBridgeDecision::from_bridge_result(
        OutputThreadRuntimeIntent::Start,
        make_rejected_result(),
    );
    assert!(d.uses_queue_bridge());
    assert_eq!(d.kind, OutputThreadWorkerQueueBridgeDecisionKind::IntentRejected);
    assert!(!d.accepted);
    assert!(d.rejected);
}

#[test]
fn decision_never_has_output_behavior() {
    let cases = [
        OutputThreadWorkerQueueBridgeDecision::from_bridge_result(
            OutputThreadRuntimeIntent::Start,
            make_accepted_result(),
        ),
        OutputThreadWorkerQueueBridgeDecision::from_bridge_result(
            OutputThreadRuntimeIntent::Stop,
            make_rejected_result(),
        ),
    ];
    for d in cases {
        assert!(d.has_no_output_behavior(), "kind {:?}", d.kind);
    }
}

#[test]
fn decision_never_passes_to_runtime_loop() {
    let cases = [
        OutputThreadWorkerQueueBridgeDecision::from_bridge_result(
            OutputThreadRuntimeIntent::Start,
            make_accepted_result(),
        ),
        OutputThreadWorkerQueueBridgeDecision::from_bridge_result(
            OutputThreadRuntimeIntent::Stop,
            make_rejected_result(),
        ),
    ];
    for d in cases {
        assert!(d.does_not_pass_to_runtime_loop(), "kind {:?}", d.kind);
    }
}
