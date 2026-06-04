use super::output_thread_runtime_queue_owner_fixed_slots::{
    OutputThreadRuntimeQueueOwnerFixedSlots, RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY,
};
use super::output_thread_runtime_queue_owner_slot_state::OutputThreadRuntimeQueueOwnerSlotState;

fn make_empty_state() -> OutputThreadRuntimeQueueOwnerSlotState {
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    OutputThreadRuntimeQueueOwnerSlotState::from_slots(slots)
}

#[test]
fn state_from_empty_slots_reports_capacity() {
    let state = make_empty_state();
    assert_eq!(state.capacity, RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY);
    assert_eq!(state.occupied_count, 0);
    assert_eq!(state.empty_count, RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY);
}

#[test]
fn state_from_full_slots_has_no_availability() {
    let mut slots_arr = [None; RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY];
    for i in 0..RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY {
        slots_arr[i] = Some(super::output_thread_runtime_queue_entry::OutputThreadRuntimeQueueEntry::new(
            super::output_thread_runtime_id::OutputThreadRuntimeGeneration::default(),
            super::output_thread_runtime_intent::OutputThreadRuntimeIntent::Start,
            i as u64 + 1,
        ));
    }
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::from_slots(slots_arr);
    let state = OutputThreadRuntimeQueueOwnerSlotState::from_slots(slots);
    assert!(!state.has_availability());
}

#[test]
fn state_from_partial_slots_has_availability() {
    let mut slots_arr = [None; RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY];
    slots_arr[0] = Some(super::output_thread_runtime_queue_entry::OutputThreadRuntimeQueueEntry::new(
        super::output_thread_runtime_id::OutputThreadRuntimeGeneration::default(),
        super::output_thread_runtime_intent::OutputThreadRuntimeIntent::Start,
        1,
    ));
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::from_slots(slots_arr);
    let state = OutputThreadRuntimeQueueOwnerSlotState::from_slots(slots);
    assert!(state.has_availability());
}

#[test]
fn state_has_no_dynamic_collection() {
    let state = make_empty_state();
    assert!(state.has_no_dynamic_collection());
}

#[test]
fn state_has_no_mutable_runtime_queue() {
    let state = make_empty_state();
    assert!(state.has_no_mutable_runtime_queue());
}

#[test]
fn state_has_no_bridge_result_apply() {
    let state = make_empty_state();
    assert!(state.has_no_bridge_result_apply());
}

#[test]
fn state_has_no_output_behavior() {
    let state = make_empty_state();
    assert!(state.has_no_output_behavior());
}
