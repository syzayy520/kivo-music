//! Runtime queue owner slot state — occupancy descriptor.
//!
//! Describes availability and occupancy of fixed slots.
//! Does not use dynamic collections or apply bridge results.

use super::output_thread_runtime_queue_owner_fixed_slots::OutputThreadRuntimeQueueOwnerFixedSlots;

/// Slot state descriptor for the runtime queue owner.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueOwnerSlotState {
    pub capacity: usize,
    pub occupied_count: usize,
    pub empty_count: usize,
    pub first_empty_index: Option<usize>,
    pub fixed_slots_used: bool,
    pub dynamic_collection_used: bool,
    pub has_mutable_runtime_queue: bool,
    pub apply_bridge_result: bool,
    pub adapter_connected: bool,
    pub runner_connected: bool,
    pub has_output_behavior: bool,
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueOwnerSlotState {
    /// Create slot state from fixed slots.
    pub(crate) fn from_slots(slots: OutputThreadRuntimeQueueOwnerFixedSlots) -> Self {
        let capacity = slots.capacity();
        let occupied_count = slots.occupied();
        Self {
            capacity,
            occupied_count,
            empty_count: capacity - occupied_count,
            first_empty_index: slots.first_empty_index(),
            fixed_slots_used: true,
            dynamic_collection_used: false,
            has_mutable_runtime_queue: false,
            apply_bridge_result: false,
            adapter_connected: false,
            runner_connected: false,
            has_output_behavior: false,
        }
    }

    /// Whether there are empty slots available.
    pub(crate) fn has_availability(self) -> bool {
        self.empty_count > 0
    }

    /// Whether this uses no dynamic collection.
    pub(crate) fn has_no_dynamic_collection(self) -> bool {
        !self.dynamic_collection_used
    }

    /// Whether this has no mutable runtime queue.
    pub(crate) fn has_no_mutable_runtime_queue(self) -> bool {
        !self.has_mutable_runtime_queue
    }

    /// Whether this does not apply bridge results.
    pub(crate) fn has_no_bridge_result_apply(self) -> bool {
        !self.apply_bridge_result
    }

    /// Whether this has no output behavior.
    pub(crate) fn has_no_output_behavior(self) -> bool {
        !self.has_output_behavior
    }
}
