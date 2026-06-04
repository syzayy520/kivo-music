use super::super::output_thread_runtime_handle::OutputThreadRuntimeHandle;
use super::super::output_thread_runtime_id::OutputThreadRuntimeGeneration;
use super::super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::super::runtime_queue_bridge::input::OutputThreadRuntimeQueueBridgeInput;
use super::super::runtime_queue::config::OutputThreadRuntimeQueueConfig;
use super::super::runtime_queue::snapshot::OutputThreadRuntimeQueueSnapshot;
use super::super::runtime_queue::state::OutputThreadRuntimeQueueState;
use super::super::output_thread_runtime_status::OutputThreadRuntimeStatus;
use super::super::output_thread_state::OutputThreadState;
use super::adapter::adapt_worker_intent_to_queue_bridge;
use super::decision::OutputThreadWorkerQueueBridgeDecisionKind;

fn make_bridge_input(
    state: OutputThreadState,
    queue_state: OutputThreadRuntimeQueueState,
) -> OutputThreadRuntimeQueueBridgeInput {
    let config = OutputThreadRuntimeQueueConfig::default();
    let status = OutputThreadRuntimeStatus::new(state, true, true, false, false);
    let handle = OutputThreadRuntimeHandle::new(
        super::super::output_thread_runtime_id::OutputThreadRuntimeId::default(),
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
fn start_intent_calls_queue_bridge() {
    let result = adapt_worker_intent_to_queue_bridge(
        make_empty_queue_input(OutputThreadState::Created),
        OutputThreadRuntimeIntent::Start,
    );
    assert!(result.queue_bridge_used);
    assert!(!result.passes_to_runtime_loop);
    assert!(!result.has_output_behavior);
}

#[test]
fn stop_intent_calls_queue_bridge() {
    let result = adapt_worker_intent_to_queue_bridge(
        make_empty_queue_input(OutputThreadState::Running),
        OutputThreadRuntimeIntent::Stop,
    );
    assert!(result.queue_bridge_used);
    assert!(!result.passes_to_runtime_loop);
    assert!(!result.has_output_behavior);
}

#[test]
fn closed_queue_is_rejected() {
    let closed_state = OutputThreadRuntimeQueueState::empty().closed();
    let result = adapt_worker_intent_to_queue_bridge(
        make_bridge_input(OutputThreadState::Running, closed_state),
        OutputThreadRuntimeIntent::Start,
    );
    assert!(result.queue_bridge_used);
    assert_eq!(
        result.decision.kind,
        OutputThreadWorkerQueueBridgeDecisionKind::IntentRejected
    );
    assert!(result.decision.rejected);
}

#[test]
fn full_queue_is_rejected() {
    let config = OutputThreadRuntimeQueueConfig::strict();
    let full_state = OutputThreadRuntimeQueueState::new(
        config.max_pending_commands,
        0,
        0,
        0,
        false,
    );
    let handle = OutputThreadRuntimeHandle::new(
        super::super::output_thread_runtime_id::OutputThreadRuntimeId::default(),
        OutputThreadRuntimeGeneration::default(),
        OutputThreadRuntimeStatus::new(OutputThreadState::Running, true, true, false, false),
    );
    let snapshot = OutputThreadRuntimeQueueSnapshot::new(config, full_state);
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, snapshot);

    let result = adapt_worker_intent_to_queue_bridge(input, OutputThreadRuntimeIntent::Start);
    assert!(result.queue_bridge_used);
    assert_eq!(
        result.decision.kind,
        OutputThreadWorkerQueueBridgeDecisionKind::IntentRejected
    );
}

#[test]
fn invalid_runtime_status_is_rejected() {
    let result = adapt_worker_intent_to_queue_bridge(
        make_empty_queue_input(OutputThreadState::Created),
        OutputThreadRuntimeIntent::Stop,
    );
    assert!(result.queue_bridge_used);
    assert_eq!(
        result.decision.kind,
        OutputThreadWorkerQueueBridgeDecisionKind::IntentRejected
    );
}

#[test]
fn adapter_does_not_pass_result_to_runtime_loop() {
    let result = adapt_worker_intent_to_queue_bridge(
        make_empty_queue_input(OutputThreadState::Created),
        OutputThreadRuntimeIntent::Start,
    );
    assert!(!result.passes_to_runtime_loop);
    assert!(result.decision.does_not_pass_to_runtime_loop());
}

#[test]
fn adapter_has_no_output_behavior() {
    let intents = [
        OutputThreadRuntimeIntent::Start,
        OutputThreadRuntimeIntent::Stop,
        OutputThreadRuntimeIntent::Close,
        OutputThreadRuntimeIntent::ResetDevice,
    ];
    for intent in intents {
        let result = adapt_worker_intent_to_queue_bridge(
            make_empty_queue_input(OutputThreadState::Running),
            intent,
        );
        assert!(
            !result.has_output_behavior,
            "intent {:?}",
            intent
        );
    }
}

#[test]
fn adapter_does_not_poll_transport_channel() {
    let result = adapt_worker_intent_to_queue_bridge(
        make_empty_queue_input(OutputThreadState::Created),
        OutputThreadRuntimeIntent::Start,
    );
    assert!(result.queue_bridge_used);
    assert!(!result.passes_to_runtime_loop);
}
