use super::output_thread_runtime_queue_config::OutputThreadRuntimeQueueConfig;
use super::output_thread_runtime_queue_owner_fixed_slots::OutputThreadRuntimeQueueOwnerFixedSlots;
use super::output_thread_runtime_queue_owner_slot_projection::project_fixed_slots_snapshot;

#[test]
fn projection_creates_snapshot() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    let projection = project_fixed_slots_snapshot(config, slots);
    assert_eq!(projection.snapshot.config, config);
}

#[test]
fn projection_state_matches_slot_occupancy() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let mut slots_arr = [None; 8];
    slots_arr[0] = Some(super::output_thread_runtime_queue_entry::OutputThreadRuntimeQueueEntry::new(
        super::output_thread_runtime_id::OutputThreadRuntimeGeneration::default(),
        super::output_thread_runtime_intent::OutputThreadRuntimeIntent::Start,
        1,
    ));
    slots_arr[1] = Some(super::output_thread_runtime_queue_entry::OutputThreadRuntimeQueueEntry::new(
        super::output_thread_runtime_id::OutputThreadRuntimeGeneration::default(),
        super::output_thread_runtime_intent::OutputThreadRuntimeIntent::Stop,
        2,
    ));
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::from_slots(slots_arr);
    let projection = project_fixed_slots_snapshot(config, slots);
    assert_eq!(projection.snapshot.state.pending_count, 2);
}

#[test]
fn projection_does_not_project_bridge_input() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    let projection = project_fixed_slots_snapshot(config, slots);
    assert!(!projection.bridge_input_projected);
}

#[test]
fn projection_does_not_apply_bridge_result() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    let projection = project_fixed_slots_snapshot(config, slots);
    assert!(!projection.apply_bridge_result);
}

#[test]
fn projection_has_no_output_behavior() {
    let config = OutputThreadRuntimeQueueConfig::default();
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    let projection = project_fixed_slots_snapshot(config, slots);
    assert!(!projection.has_output_behavior);
}
