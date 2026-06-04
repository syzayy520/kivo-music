//! Runtime queue owner fixed slots — fixed-capacity slot array.
//!
//! Uses a hard-coded fixed array to hold queue entries.
//! Does not use dynamic collections, apply bridge results, or connect adapters.

use super::output_thread_runtime_queue_entry::OutputThreadRuntimeQueueEntry;

/// Fixed slot capacity for the runtime queue owner.
#[allow(dead_code)]
pub(crate) const RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY: usize = 8;

/// Fixed-capacity slots holding queue entries.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueOwnerFixedSlots {
    pub slots: [Option<OutputThreadRuntimeQueueEntry>; RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY],
    pub occupied_count: usize,
    pub fixed_slots_used: bool,
    pub dynamic_collection_used: bool,
    pub apply_bridge_result: bool,
    pub adapter_connected: bool,
    pub runner_connected: bool,
    pub has_output_behavior: bool,
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueOwnerFixedSlots {
    /// Create empty slots with no entries.
    pub(crate) fn empty() -> Self {
        Self {
            slots: [None; RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY],
            occupied_count: 0,
            fixed_slots_used: true,
            dynamic_collection_used: false,
            apply_bridge_result: false,
            adapter_connected: false,
            runner_connected: false,
            has_output_behavior: false,
        }
    }

    /// Create slots from an existing array.
    pub(crate) fn from_slots(
        slots: [Option<OutputThreadRuntimeQueueEntry>; RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY],
    ) -> Self {
        let occupied_count = slots.iter().filter(|s| s.is_some()).count();
        Self {
            slots,
            occupied_count,
            fixed_slots_used: true,
            dynamic_collection_used: false,
            apply_bridge_result: false,
            adapter_connected: false,
            runner_connected: false,
            has_output_behavior: false,
        }
    }

    /// Whether all slots are occupied.
    pub(crate) fn is_full(self) -> bool {
        self.occupied_count >= RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY
    }

    /// Whether no slots are occupied.
    pub(crate) fn is_empty(self) -> bool {
        self.occupied_count == 0
    }

    /// Total slot capacity.
    pub(crate) fn capacity(self) -> usize {
        RUNTIME_QUEUE_OWNER_FIXED_SLOT_CAPACITY
    }

    /// Index of the first empty slot, if any.
    pub(crate) fn first_empty_index(self) -> Option<usize> {
        self.slots.iter().position(|s| s.is_none())
    }

    /// Number of occupied slots.
    pub(crate) fn occupied(self) -> usize {
        self.occupied_count
    }

    /// Whether this uses no dynamic collection.
    pub(crate) fn has_no_dynamic_collection(self) -> bool {
        !self.dynamic_collection_used
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
