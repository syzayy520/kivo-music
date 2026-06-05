use super::super::runtime_core::handle::OutputThreadRuntimeHandle;
use super::super::runtime_core::id::OutputThreadRuntimeGeneration;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::super::runtime_queue_bridge::input::OutputThreadRuntimeQueueBridgeInput;
use super::super::runtime_queue::config::OutputThreadRuntimeQueueConfig;
use super::super::runtime_queue::snapshot::OutputThreadRuntimeQueueSnapshot;
use super::super::runtime_queue::state::OutputThreadRuntimeQueueState;
use super::super::runtime_core::status::OutputThreadRuntimeStatus;
use super::super::output_thread_state::OutputThreadState;
use super::adapter::adapt_worker_intent_to_queue_bridge;
use super::report::OutputThreadWorkerQueueBridgeReport;

fn make_bridge_input(
    state: OutputThreadState,
    queue_state: OutputThreadRuntimeQueueState,
) -> OutputThreadRuntimeQueueBridgeInput {
    let config = OutputThreadRuntimeQueueConfig::default();
    let status = OutputThreadRuntimeStatus::new(state, true, true, false, false);
    let handle = OutputThreadRuntimeHandle::new(
        super::super::runtime_core::id::OutputThreadRuntimeId::default(),
        OutputThreadRuntimeGeneration::default(),
        status,
    );
    let snapshot = OutputThreadRuntimeQueueSnapshot::new(config, queue_state);
    OutputThreadRuntimeQueueBridgeInput::new(handle, snapshot)
}

fn make_empty_queue_input(state: OutputThreadState) -> OutputThreadRuntimeQueueBridgeInput {
    make_bridge_input(state, OutputThreadRuntimeQueueState::empty())
}

#[test]
fn empty_report_has_no_output_behavior() {
    let report = OutputThreadWorkerQueueBridgeReport::empty();
    assert!(report.has_no_output_behavior());
    assert!(report.does_not_pass_to_runtime_loop());
}

#[test]
fn record_accepted_increments_accepted_count() {
    let report = OutputThreadWorkerQueueBridgeReport::empty();
    let result = adapt_worker_intent_to_queue_bridge(
        make_empty_queue_input(OutputThreadState::Created),
        OutputThreadRuntimeIntent::Start,
    );
    let updated = report.record_adapter_result(result);
    assert_eq!(updated.accepted_count, 1);
    assert_eq!(updated.rejected_count, 0);
    assert_eq!(updated.intents_observed, 1);
}

#[test]
fn record_rejected_increments_rejected_count() {
    let report = OutputThreadWorkerQueueBridgeReport::empty();
    let closed_state = OutputThreadRuntimeQueueState::empty().closed();
    let result = adapt_worker_intent_to_queue_bridge(
        make_bridge_input(OutputThreadState::Running, closed_state),
        OutputThreadRuntimeIntent::Start,
    );
    let updated = report.record_adapter_result(result);
    assert_eq!(updated.rejected_count, 1);
    assert_eq!(updated.accepted_count, 0);
    assert_eq!(updated.intents_observed, 1);
}

#[test]
fn record_result_sets_queue_bridge_used() {
    let report = OutputThreadWorkerQueueBridgeReport::empty();
    let result = adapt_worker_intent_to_queue_bridge(
        make_empty_queue_input(OutputThreadState::Created),
        OutputThreadRuntimeIntent::Start,
    );
    let updated = report.record_adapter_result(result);
    assert!(updated.uses_queue_bridge());
}

#[test]
fn report_never_passes_to_runtime_loop() {
    let report = OutputThreadWorkerQueueBridgeReport::empty();
    let result = adapt_worker_intent_to_queue_bridge(
        make_empty_queue_input(OutputThreadState::Created),
        OutputThreadRuntimeIntent::Start,
    );
    let updated = report.record_adapter_result(result);
    assert!(updated.does_not_pass_to_runtime_loop());
    assert!(updated.has_no_output_behavior());
}
