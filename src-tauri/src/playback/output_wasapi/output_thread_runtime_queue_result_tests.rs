use super::output_thread_runtime_id::OutputThreadRuntimeGeneration;
use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::output_thread_runtime_queue_entry::OutputThreadRuntimeQueueEntry;
use super::output_thread_runtime_queue_result::{
    OutputThreadRuntimeQueueAcceptResult, OutputThreadRuntimeQueuePlanResult,
    OutputThreadRuntimeQueueRejectReason, OutputThreadRuntimeQueueRejectResult,
};
use super::output_thread_runtime_queue_state::OutputThreadRuntimeQueueState;

#[test]
fn accepted_result_reports_accepted() {
    let entry = OutputThreadRuntimeQueueEntry::new(
        OutputThreadRuntimeGeneration::default(),
        OutputThreadRuntimeIntent::Start,
        1,
    );
    let state = OutputThreadRuntimeQueueState::empty().with_acceptance();
    let result = OutputThreadRuntimeQueuePlanResult::Accepted(OutputThreadRuntimeQueueAcceptResult {
        entry,
        state,
    });

    assert!(result.is_accepted());
    assert!(!result.is_rejected());
}

#[test]
fn rejected_result_reports_rejected() {
    let state = OutputThreadRuntimeQueueState::empty().with_rejection();
    let result = OutputThreadRuntimeQueuePlanResult::Rejected(OutputThreadRuntimeQueueRejectResult {
        reason: OutputThreadRuntimeQueueRejectReason::Closed,
        state,
    });

    assert!(!result.is_accepted());
    assert!(result.is_rejected());
}

#[test]
fn accepted_result_returns_state() {
    let entry = OutputThreadRuntimeQueueEntry::new(
        OutputThreadRuntimeGeneration::default(),
        OutputThreadRuntimeIntent::Start,
        1,
    );
    let state = OutputThreadRuntimeQueueState::empty().with_acceptance();
    let result = OutputThreadRuntimeQueuePlanResult::Accepted(OutputThreadRuntimeQueueAcceptResult {
        entry,
        state,
    });

    assert_eq!(result.state(), state);
}

#[test]
fn rejected_result_returns_state() {
    let state = OutputThreadRuntimeQueueState::empty().with_rejection();
    let result = OutputThreadRuntimeQueuePlanResult::Rejected(OutputThreadRuntimeQueueRejectResult {
        reason: OutputThreadRuntimeQueueRejectReason::Full,
        state,
    });

    assert_eq!(result.state(), state);
}

#[test]
fn full_reject_reason_is_capacity_error() {
    let reason = OutputThreadRuntimeQueueRejectReason::Full;
    assert!(reason.is_capacity_error());
    assert!(!reason.is_lifecycle_error());
}

#[test]
fn closed_reject_reason_is_lifecycle_error() {
    let reason = OutputThreadRuntimeQueueRejectReason::Closed;
    assert!(reason.is_lifecycle_error());
    assert!(!reason.is_capacity_error());
}
