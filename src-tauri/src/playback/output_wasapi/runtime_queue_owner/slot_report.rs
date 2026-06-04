//! Runtime queue owner slot report — capability summary.
//!
//! Records slot state and projection capabilities.
//! Does not store queue entries, command payloads, or bridge results.

use super::slot_projection::OutputThreadRuntimeQueueOwnerSlotProjection;
use super::slot_state::OutputThreadRuntimeQueueOwnerSlotState;

/// Report summarizing runtime queue owner slot capabilities.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueOwnerSlotReport {
    pub fixed_slots_used: bool,
    pub dynamic_collection_used: bool,
    pub occupied_count: usize,
    pub capacity: usize,
    pub apply_bridge_result: bool,
    pub adapter_connected: bool,
    pub runner_connected: bool,
    pub has_output_behavior: bool,
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueOwnerSlotReport {
    /// Create an empty report.
    pub(crate) fn empty() -> Self {
        Self {
            fixed_slots_used: true,
            dynamic_collection_used: false,
            occupied_count: 0,
            capacity: 0,
            apply_bridge_result: false,
            adapter_connected: false,
            runner_connected: false,
            has_output_behavior: false,
        }
    }

    /// Record slot state into the report.
    pub(crate) fn record_slot_state(
        self,
        state: OutputThreadRuntimeQueueOwnerSlotState,
    ) -> Self {
        Self {
            fixed_slots_used: state.fixed_slots_used,
            dynamic_collection_used: state.dynamic_collection_used,
            occupied_count: state.occupied_count,
            capacity: state.capacity,
            apply_bridge_result: state.apply_bridge_result,
            adapter_connected: state.adapter_connected,
            runner_connected: state.runner_connected,
            has_output_behavior: state.has_output_behavior,
        }
    }

    /// Record projection into the report.
    pub(crate) fn record_projection(
        self,
        projection: OutputThreadRuntimeQueueOwnerSlotProjection,
    ) -> Self {
        Self {
            fixed_slots_used: projection.fixed_slots_used,
            dynamic_collection_used: projection.dynamic_collection_used,
            apply_bridge_result: projection.apply_bridge_result,
            has_output_behavior: projection.has_output_behavior,
            ..self
        }
    }

    /// Whether this uses no dynamic collection.
    pub(crate) fn has_no_dynamic_collection(self) -> bool {
        !self.dynamic_collection_used
    }

    /// Whether this does not apply bridge results.
    pub(crate) fn has_no_bridge_result_apply(self) -> bool {
        !self.apply_bridge_result
    }

    /// Whether this has no adapter connection.
    pub(crate) fn has_no_adapter_connection(self) -> bool {
        !self.adapter_connected
    }

    /// Whether this has no runner connection.
    pub(crate) fn has_no_runner_connection(self) -> bool {
        !self.runner_connected
    }

    /// Whether this has no output behavior.
    pub(crate) fn has_no_output_behavior(self) -> bool {
        !self.has_output_behavior
    }
}
