use super::fixed_slots::{
    OutputThreadRuntimeQueueOwnerFixedSlots, RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY,
};

#[test]
fn empty_slots_have_fixed_capacity() {
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    assert_eq!(slots.capacity(), RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY);
}

#[test]
fn empty_slots_have_zero_occupied_count() {
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    assert_eq!(slots.occupied(), 0);
}

#[test]
fn single_slot_occupied_updates_occupied_count() {
    let mut slots_arr = [None; RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY];
    slots_arr[0] = Some(super::super::runtime_queue::entry::OutputThreadRuntimeQueueEntry::new(
        super::super::runtime_core::id::OutputThreadRuntimeGeneration::default(),
        super::super::runtime_core::intent::OutputThreadRuntimeIntent::Start,
        1,
    ));
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::from_slots(slots_arr);
    assert_eq!(slots.occupied(), 1);
}

#[test]
fn full_slots_report_full() {
    let mut slots_arr = [None; RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY];
    for i in 0..RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY {
        slots_arr[i] = Some(super::super::runtime_queue::entry::OutputThreadRuntimeQueueEntry::new(
            super::super::runtime_core::id::OutputThreadRuntimeGeneration::default(),
            super::super::runtime_core::intent::OutputThreadRuntimeIntent::Start,
            i as u64 + 1,
        ));
    }
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::from_slots(slots_arr);
    assert!(slots.is_full());
    assert!(!slots.is_empty());
}

#[test]
fn first_empty_index_is_reported() {
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    assert_eq!(slots.first_empty_index(), Some(0));
}

#[test]
fn fixed_slots_never_use_dynamic_collection() {
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    assert!(slots.has_no_dynamic_collection());
}

#[test]
fn fixed_slots_never_apply_bridge_result() {
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    assert!(slots.has_no_bridge_result_apply());
}

#[test]
fn fixed_slots_never_connect_adapter_or_runner() {
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    assert!(!slots.adapter_connected);
    assert!(!slots.runner_connected);
}

#[test]
fn fixed_slots_have_no_output_behavior() {
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    assert!(slots.has_no_output_behavior());
}
