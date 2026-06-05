use super::super::runtime_core::handle::OutputThreadRuntimeHandle;
use super::super::runtime_core::id::{OutputThreadRuntimeGeneration, OutputThreadRuntimeId};
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::bridge::*;
use super::super::runtime_queue::config::OutputThreadRuntimeQueueConfig;
use super::super::runtime_queue::result::OutputThreadRuntimeQueueRejectReason;
use super::super::runtime_queue::snapshot::OutputThreadRuntimeQueueSnapshot;
use super::super::runtime_queue::state::OutputThreadRuntimeQueueState;
use super::super::runtime_core::status::OutputThreadRuntimeStatus;
use super::super::output_thread_state::OutputThreadState;

fn handle_with_state(state: OutputThreadState, gen: u64) -> OutputThreadRuntimeHandle {
    let id = OutputThreadRuntimeId::new(1);
    let generation = OutputThreadRuntimeGeneration::new(gen);
    let status = OutputThreadRuntimeStatus::new(state, true, true, false, true);
    OutputThreadRuntimeHandle::new(id, generation, status)
}
fn empty_snapshot(config: OutputThreadRuntimeQueueConfig) -> OutputThreadRuntimeQueueSnapshot {
    OutputThreadRuntimeQueueSnapshot::empty(config)
}
#[test]
fn bridge_input_exposes_generation_status_and_queue_state() {
    let handle = handle_with_state(OutputThreadState::Running, 5);
    let config = OutputThreadRuntimeQueueConfig::default();
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, empty_snapshot(config));
    assert_eq!(input.generation(), OutputThreadRuntimeGeneration::new(5));
    assert_eq!(input.status(), handle.status);
    assert_eq!(input.state(), OutputThreadState::Running);
    assert_eq!(input.queue_state(), OutputThreadRuntimeQueueState::empty());
    assert_eq!(input.queue_config(), config);
}
#[test]
fn bridge_projection_reports_pending_and_capacity() {
    let handle = handle_with_state(OutputThreadState::Running, 0);
    let config = OutputThreadRuntimeQueueConfig::default();
    let state = OutputThreadRuntimeQueueState::new(5, 10, 2, 10, false);
    let queue = OutputThreadRuntimeQueueSnapshot::new(config, state);
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, queue);
    let result = plan_runtime_queue_bridge_intent(input, OutputThreadRuntimeIntent::Pause);
    let proj = result.projection();
    assert_eq!(proj.pending_count, 6);
    assert!(proj.queue_has_capacity);
    assert!(proj.bridge_can_accept_intents);
    assert_eq!(proj.accepted_count, 11);
}
#[test]
fn bridge_accepts_when_queue_has_capacity_and_runtime_allows_intent() {
    let handle = handle_with_state(OutputThreadState::Running, 0);
    let config = OutputThreadRuntimeQueueConfig::default();
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, empty_snapshot(config));
    let result = plan_runtime_queue_bridge_intent(input, OutputThreadRuntimeIntent::Pause);
    assert!(result.is_accepted());
}
#[test]
fn bridge_rejects_closed_queue() {
    let handle = handle_with_state(OutputThreadState::Running, 0);
    let config = OutputThreadRuntimeQueueConfig::default();
    let state = OutputThreadRuntimeQueueState::empty().closed();
    let queue = OutputThreadRuntimeQueueSnapshot::new(config, state);
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, queue);
    let result = plan_runtime_queue_bridge_intent(input, OutputThreadRuntimeIntent::Flush);
    assert!(result.is_rejected());
    assert!(result.projection().queue_closed);
}
#[test]
fn bridge_rejects_full_queue() {
    let handle = handle_with_state(OutputThreadState::Running, 0);
    let config = OutputThreadRuntimeQueueConfig::default();
    let state = OutputThreadRuntimeQueueState::new(32, 32, 0, 32, false);
    let queue = OutputThreadRuntimeQueueSnapshot::new(config, state);
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, queue);
    let result = plan_runtime_queue_bridge_intent(input, OutputThreadRuntimeIntent::Flush);
    assert!(result.is_rejected());
    assert!(!result.projection().queue_has_capacity);
}
#[test]
fn bridge_rejects_wrong_runtime_status_as_invalid_for_runtime() {
    let handle = handle_with_state(OutputThreadState::Created, 0);
    let config = OutputThreadRuntimeQueueConfig::default();
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, empty_snapshot(config));
    let result = plan_runtime_queue_bridge_intent(input, OutputThreadRuntimeIntent::Pause);
    assert!(result.is_rejected());
    if let OutputThreadRuntimeQueueBridgeResult::Rejected(rejected) = result {
        assert_eq!(rejected.reason, OutputThreadRuntimeQueueRejectReason::InvalidForRuntime);
    } else {
        panic!("expected rejection");
    }
}
#[test]
fn bridge_rejects_stopped_reset_when_policy_disallows_it() {
    let handle = handle_with_state(OutputThreadState::Stopped, 0);
    let config = OutputThreadRuntimeQueueConfig::strict();
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, empty_snapshot(config));
    let result = plan_runtime_queue_bridge_intent(input, OutputThreadRuntimeIntent::ResetDevice);
    assert!(result.is_rejected());
    if let OutputThreadRuntimeQueueBridgeResult::Rejected(rejected) = result {
        assert_eq!(rejected.reason, OutputThreadRuntimeQueueRejectReason::InvalidForRuntime);
    } else {
        panic!("expected rejection");
    }
}
#[test]
fn bridge_accepts_stopped_reset_when_policy_allows_it() {
    let handle = handle_with_state(OutputThreadState::Stopped, 0);
    let config = OutputThreadRuntimeQueueConfig::default();
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, empty_snapshot(config));
    let result = plan_runtime_queue_bridge_intent(input, OutputThreadRuntimeIntent::ResetDevice);
    assert!(result.is_accepted());
}
#[test]
fn bridge_invalid_runtime_rejection_rolls_back_pending_count() {
    let handle = handle_with_state(OutputThreadState::Created, 0);
    let config = OutputThreadRuntimeQueueConfig::default();
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, empty_snapshot(config));
    let result = plan_runtime_queue_bridge_intent(input, OutputThreadRuntimeIntent::Pause);
    assert!(result.is_rejected());
    assert_eq!(result.queue_state().pending_count, 0);
}
#[test]
fn bridge_invalid_runtime_rejection_increments_rejected_count() {
    let handle = handle_with_state(OutputThreadState::Created, 0);
    let config = OutputThreadRuntimeQueueConfig::default();
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, empty_snapshot(config));
    let result = plan_runtime_queue_bridge_intent(input, OutputThreadRuntimeIntent::Pause);
    assert!(result.is_rejected());
    assert_eq!(result.queue_state().rejected_count, 1);
}
#[test]
fn bridge_accepted_path_increments_pending_accepted_and_sequence() {
    let handle = handle_with_state(OutputThreadState::Running, 0);
    let config = OutputThreadRuntimeQueueConfig::default();
    let state = OutputThreadRuntimeQueueState::new(3, 5, 0, 5, false);
    let queue = OutputThreadRuntimeQueueSnapshot::new(config, state);
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, queue);
    let result = plan_runtime_queue_bridge_intent(input, OutputThreadRuntimeIntent::Pause);
    assert!(result.is_accepted());
    assert_eq!(result.queue_state().pending_count, 4);
    assert_eq!(result.queue_state().accepted_count, 6);
    assert_eq!(result.queue_state().last_sequence, 6);
}
#[test]
fn duplicate_lifecycle_policy_is_reserved_in_bridge() {
    let handle = handle_with_state(OutputThreadState::Running, 0);
    let config1 = OutputThreadRuntimeQueueConfig::default();
    let config2 = OutputThreadRuntimeQueueConfig::permissive();
    let input1 = OutputThreadRuntimeQueueBridgeInput::new(handle, empty_snapshot(config1));
    let input2 = OutputThreadRuntimeQueueBridgeInput::new(handle, empty_snapshot(config2));
    let result1 = plan_runtime_queue_bridge_intent(input1, OutputThreadRuntimeIntent::Pause);
    let result2 = plan_runtime_queue_bridge_intent(input2, OutputThreadRuntimeIntent::Pause);
    assert!(result1.is_accepted());
    assert!(result2.is_accepted());
    assert_eq!(result1.queue_state().pending_count, 1);
    assert_eq!(result2.queue_state().pending_count, 1);
}
#[test]
fn bridge_result_returns_projection() {
    let handle = handle_with_state(OutputThreadState::Running, 0);
    let config = OutputThreadRuntimeQueueConfig::default();
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, empty_snapshot(config));
    let result = plan_runtime_queue_bridge_intent(input, OutputThreadRuntimeIntent::Pause);
    let proj = result.projection();
    assert_eq!(proj.runtime_state, OutputThreadState::Running);
    assert!(proj.bridge_can_accept_intents);
}
#[test]
fn bridge_projection_does_not_imply_real_output() {
    let handle = handle_with_state(OutputThreadState::Running, 0);
    let config = OutputThreadRuntimeQueueConfig::default();
    let input = OutputThreadRuntimeQueueBridgeInput::new(handle, empty_snapshot(config));
    let result = plan_runtime_queue_bridge_intent(input, OutputThreadRuntimeIntent::Pause);
    let proj = result.projection();
    assert!(proj.bridge_can_accept_intents);
    assert!(proj.runtime_can_accept_frames);
}