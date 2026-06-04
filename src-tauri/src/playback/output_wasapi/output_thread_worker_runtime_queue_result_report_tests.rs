use super::output_thread_runtime_id::OutputThreadRuntimeGeneration;
use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::output_thread_runtime_loop_state::OutputThreadRuntimeLoopState;
use super::runtime_queue_bridge::result::{
    OutputThreadRuntimeQueueBridgeAccepted, OutputThreadRuntimeQueueBridgeRejected,
    OutputThreadRuntimeQueueBridgeResult,
};
use super::runtime_queue::entry::OutputThreadRuntimeQueueEntry;
use super::runtime_queue::result::OutputThreadRuntimeQueueRejectReason;
use super::runtime_queue::state::OutputThreadRuntimeQueueState;
use super::output_thread_worker_runtime_queue_result_adapter::adapt_queue_bridge_result_to_runtime_loop_step;
use super::output_thread_worker_runtime_queue_result_report::OutputThreadWorkerRuntimeQueueResultReport;

fn make_accepted_result(intent: OutputThreadRuntimeIntent) -> OutputThreadRuntimeQueueBridgeResult {
    OutputThreadRuntimeQueueBridgeResult::Accepted(OutputThreadRuntimeQueueBridgeAccepted {
        entry: OutputThreadRuntimeQueueEntry::new(
            OutputThreadRuntimeGeneration::default(),
            intent,
            1,
        ),
        queue_state: OutputThreadRuntimeQueueState::empty(),
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
        reason: OutputThreadRuntimeQueueRejectReason::Full,
        queue_state: OutputThreadRuntimeQueueState::empty(),
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
fn empty_report_has_no_output_behavior() {
    let report = OutputThreadWorkerRuntimeQueueResultReport::empty();
    assert!(report.has_no_output_behavior());
    assert!(report.has_no_queue_ownership());
}

#[test]
fn record_accepted_increments_accepted_seen() {
    let report = OutputThreadWorkerRuntimeQueueResultReport::empty();
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_accepted_result(OutputThreadRuntimeIntent::Start),
        OutputThreadRuntimeLoopState::Idle,
    );
    let updated = report.record_adapter_result(result);
    assert_eq!(updated.accepted_seen, 1);
    assert_eq!(updated.rejected_seen, 0);
    assert_eq!(updated.queue_results_seen, 1);
}

#[test]
fn record_rejected_increments_rejected_seen() {
    let report = OutputThreadWorkerRuntimeQueueResultReport::empty();
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_rejected_result(),
        OutputThreadRuntimeLoopState::Active,
    );
    let updated = report.record_adapter_result(result);
    assert_eq!(updated.rejected_seen, 1);
    assert_eq!(updated.accepted_seen, 0);
    assert_eq!(updated.queue_results_seen, 1);
}

#[test]
fn record_result_sets_passes_from_queue_bridge() {
    let report = OutputThreadWorkerRuntimeQueueResultReport::empty();
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_accepted_result(OutputThreadRuntimeIntent::Start),
        OutputThreadRuntimeLoopState::Idle,
    );
    let updated = report.record_adapter_result(result);
    assert!(updated.passes_from_queue_bridge());
}

#[test]
fn report_never_has_queue_ownership() {
    let report = OutputThreadWorkerRuntimeQueueResultReport::empty();
    let result = adapt_queue_bridge_result_to_runtime_loop_step(
        make_accepted_result(OutputThreadRuntimeIntent::Start),
        OutputThreadRuntimeLoopState::Idle,
    );
    let updated = report.record_adapter_result(result);
    assert!(updated.has_no_queue_ownership());
    assert!(updated.has_no_output_behavior());
}
