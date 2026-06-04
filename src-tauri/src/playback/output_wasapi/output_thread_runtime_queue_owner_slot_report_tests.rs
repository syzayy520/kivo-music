use super::output_thread_runtime_queue_owner_fixed_slots::OutputThreadRuntimeQueueOwnerFixedSlots;
use super::output_thread_runtime_queue_owner_slot_report::OutputThreadRuntimeQueueOwnerSlotReport;
use super::output_thread_runtime_queue_owner_slot_state::OutputThreadRuntimeQueueOwnerSlotState;

#[test]
fn empty_report_has_no_output_behavior() {
    let report = OutputThreadRuntimeQueueOwnerSlotReport::empty();
    assert!(report.has_no_output_behavior());
}

#[test]
fn record_slot_state_marks_fixed_slots() {
    let slots = OutputThreadRuntimeQueueOwnerFixedSlots::empty();
    let state = OutputThreadRuntimeQueueOwnerSlotState::from_slots(slots);
    let report = OutputThreadRuntimeQueueOwnerSlotReport::empty().record_slot_state(state);
    assert!(report.fixed_slots_used);
    assert_eq!(report.capacity, 8);
}

#[test]
fn report_never_uses_dynamic_collection() {
    let report = OutputThreadRuntimeQueueOwnerSlotReport::empty();
    assert!(report.has_no_dynamic_collection());
}

#[test]
fn report_never_applies_bridge_result() {
    let report = OutputThreadRuntimeQueueOwnerSlotReport::empty();
    assert!(report.has_no_bridge_result_apply());
}

#[test]
fn report_never_connects_adapter() {
    let report = OutputThreadRuntimeQueueOwnerSlotReport::empty();
    assert!(report.has_no_adapter_connection());
}

#[test]
fn report_never_connects_runner() {
    let report = OutputThreadRuntimeQueueOwnerSlotReport::empty();
    assert!(report.has_no_runner_connection());
}
