//! Runtime queue owner slot projection — snapshot generation.
//!
//! Projects an immutable snapshot from fixed slots.
//! Does not generate bridge input or apply bridge results.

use super::super::runtime_queue::config::OutputThreadRuntimeQueueConfig;
use super::fixed_slots::OutputThreadRuntimeQueueOwnerFixedSlots;
use super::super::runtime_queue::snapshot::OutputThreadRuntimeQueueSnapshot;
use super::super::runtime_queue::state::OutputThreadRuntimeQueueState;

/// Projection result from fixed slots.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueOwnerSlotProjection {
    pub snapshot: OutputThreadRuntimeQueueSnapshot,
    pub fixed_slots_used: bool,
    pub dynamic_collection_used: bool,
    pub bridge_input_projected: bool,
    pub apply_bridge_result: bool,
    pub has_output_behavior: bool,
}

/// Project a snapshot from fixed slots and config.
#[allow(dead_code)]
pub(crate) fn project_fixed_slots_snapshot(
    config: OutputThreadRuntimeQueueConfig,
    slots: OutputThreadRuntimeQueueOwnerFixedSlots,
) -> OutputThreadRuntimeQueueOwnerSlotProjection {
    let state = OutputThreadRuntimeQueueState::new(
        slots.occupied() as u16,
        slots.occupied() as u64,
        0,
        slots.occupied() as u64,
        false,
    );
    let snapshot = OutputThreadRuntimeQueueSnapshot::new(config, state);

    OutputThreadRuntimeQueueOwnerSlotProjection {
        snapshot,
        fixed_slots_used: true,
        dynamic_collection_used: false,
        bridge_input_projected: false,
        apply_bridge_result: false,
        has_output_behavior: false,
    }
}
