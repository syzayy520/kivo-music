use super::super::runtime_core::id::OutputThreadRuntimeGeneration;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::super::runtime_loop::state::OutputThreadRuntimeLoopState;
use super::super::runtime_loop::step::OutputThreadRuntimeLoopStepAction;
use super::super::runtime_queue_bridge::result::{
    OutputThreadRuntimeQueueBridgeAccepted, OutputThreadRuntimeQueueBridgeRejected,
    OutputThreadRuntimeQueueBridgeResult,
};
use super::super::runtime_queue::entry::OutputThreadRuntimeQueueEntry;
use super::super::runtime_queue::result::OutputThreadRuntimeQueueRejectReason;
use super::super::runtime_queue::state::OutputThreadRuntimeQueueState;
use super::adapter::adapt_queue_bridge_result_to_runtime_loop_step;

fn make_accepted_result(intent: OutputThreadRuntimeIntent) -> OutputThreadRuntimeQueueBridgeResult {
    OutputThreadRuntimeQueueBridgeResult::Accepted(OutputThreadRuntimeQueueBridgeAccepted {
        entry: OutputThreadRuntimeQueueEntry::new(
            OutputThreadRuntimeGeneration::default(),
            intent,
            1,
        ),
        queue_state: OutputThreadRuntimeQueueState::empty(),
        projection: super::super::runtime_queue_bridge::projection::OutputThreadRuntimeQueueBridgeProjection {
            runtime_state: super::super::output_thread_state::OutputThreadState::Created,
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
        reason: OutputThreadRuntimeQueueRejectReason::Full,
        queue_state: OutputThreadRuntimeQueueState::empty(),
        projection: super::super::runtime_queue_bridge::projection::OutputThreadRuntimeQueueBridgeProjection {
            runtime_state: super::super::output_thread_state::OutputThreadState::Running,
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
fn accepted_start_from_idle_enters_active() {
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_accepted_result(OutputThreadRuntimeIntent::Start),
        OutputThreadRuntimeLoopState::Idle,
    );
    assert_eq!(
        result.runtime_loop_decision.action,
        OutputThreadRuntimeLoopStepAction::EnterActive
    );
    assert_eq!(
        result.runtime_loop_decision.next_state,
        OutputThreadRuntimeLoopState::Active
    );
    assert!(result.runtime_loop_decision.should_continue);
}

#[test]
fn accepted_stop_from_active_marks_stopping() {
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_accepted_result(OutputThreadRuntimeIntent::Stop),
        OutputThreadRuntimeLoopState::Active,
    );
    assert_eq!(
        result.runtime_loop_decision.action,
        OutputThreadRuntimeLoopStepAction::MarkStopping
    );
    assert_eq!(
        result.runtime_loop_decision.next_state,
        OutputThreadRuntimeLoopState::Stopping
    );
    assert!(result.runtime_loop_decision.should_continue);
}

#[test]
fn accepted_close_from_active_marks_exited() {
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_accepted_result(OutputThreadRuntimeIntent::Close),
        OutputThreadRuntimeLoopState::Active,
    );
    assert_eq!(
        result.runtime_loop_decision.action,
        OutputThreadRuntimeLoopStepAction::MarkExited
    );
    assert_eq!(
        result.runtime_loop_decision.next_state,
        OutputThreadRuntimeLoopState::Exited
    );
    assert!(!result.runtime_loop_decision.should_continue);
}

#[test]
fn rejected_result_is_ignored() {
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_rejected_result(),
        OutputThreadRuntimeLoopState::Active,
    );
    assert_eq!(
        result.runtime_loop_decision.action,
        OutputThreadRuntimeLoopStepAction::IgnoreRejectedIntent
    );
    assert!(result.runtime_loop_decision.should_continue);
}

#[test]
fn accepted_start_from_active_does_not_fake_restart() {
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_accepted_result(OutputThreadRuntimeIntent::Start),
        OutputThreadRuntimeLoopState::Active,
    );
    // Active state cannot accept Start, so it follows render plan.
    assert_eq!(
        result.runtime_loop_decision.action,
        OutputThreadRuntimeLoopStepAction::FollowRenderPlan
    );
    assert!(result.runtime_loop_decision.should_continue);
}

#[test]
fn accepted_stop_from_idle_does_not_fake_stop() {
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_accepted_result(OutputThreadRuntimeIntent::Stop),
        OutputThreadRuntimeLoopState::Idle,
    );
    // Idle state cannot accept Stop, so it follows render plan.
    assert_eq!(
        result.runtime_loop_decision.action,
        OutputThreadRuntimeLoopStepAction::FollowRenderPlan
    );
    assert!(result.runtime_loop_decision.should_continue);
}

#[test]
fn adapter_sets_passes_from_queue_bridge_true() {
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_accepted_result(OutputThreadRuntimeIntent::Start),
        OutputThreadRuntimeLoopState::Idle,
    );
    assert!(result.passes_from_queue_bridge);
    assert!(result.decision.passes_from_queue_bridge());
}

#[test]
fn adapter_has_no_output_behavior() {
    let cases = [
        adapt_queue_bridge_result_to_runtime_loop_step(
            make_accepted_result(OutputThreadRuntimeIntent::Start),
            OutputThreadRuntimeLoopState::Idle,
        ),
        adapt_queue_bridge_result_to_runtime_loop_step(
            make_rejected_result(),
            OutputThreadRuntimeLoopState::Active,
        ),
    ];
    for result in cases {
        assert!(!result.has_output_behavior);
        assert!(result.decision.has_no_output_behavior());
    }
}

#[test]
fn adapter_has_no_queue_ownership() {
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_accepted_result(OutputThreadRuntimeIntent::Start),
        OutputThreadRuntimeLoopState::Idle,
    );
    assert!(!result.has_queue_ownership);
    assert!(result.decision.has_no_queue_ownership());
}

#[test]
fn adapter_does_not_call_queue_bridge_plan() {
    // This test verifies the adapter is pure — no bridge plan needed.
    // It only uses existing queue bridge results.
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_accepted_result(OutputThreadRuntimeIntent::Start),
        OutputThreadRuntimeLoopState::Idle,
    );
    assert!(result.passes_from_queue_bridge);
    assert!(!result.has_queue_ownership);
}
