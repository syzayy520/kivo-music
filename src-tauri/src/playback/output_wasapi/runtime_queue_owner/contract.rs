//! Runtime queue owner contract — ownership boundary descriptor.
//!
//! Defines what a future runtime queue owner should be capable of,
//! but does not implement a real owner. No entry collection, no mutable queue.

/// Stage of the runtime queue owner contract.
#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeQueueOwnerStage {
    /// Contract only — no projections enabled.
    ContractOnly,
    /// Snapshot projection enabled.
    SnapshotProjectionOnly,
    /// Bridge input projection enabled.
    BridgeInputProjectionOnly,
}

/// Contract describing the boundary and capabilities of a runtime queue owner.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueOwnerContract {
    pub stage: OutputThreadRuntimeQueueOwnerStage,
    pub has_entry_collection: bool,
    pub has_mutable_runtime_queue: bool,
    pub can_project_snapshot: bool,
    pub can_project_bridge_input: bool,
    pub can_apply_bridge_result: bool,
    pub adapter_connected: bool,
    pub runner_connected: bool,
    pub has_output_behavior: bool,
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueOwnerContract {
    /// Contract-only: no projections, no entry collection, no mutable queue.
    pub(crate) fn contract_only() -> Self {
        Self {
            stage: OutputThreadRuntimeQueueOwnerStage::ContractOnly,
            has_entry_collection: false,
            has_mutable_runtime_queue: false,
            can_project_snapshot: false,
            can_project_bridge_input: false,
            can_apply_bridge_result: false,
            adapter_connected: false,
            runner_connected: false,
            has_output_behavior: false,
        }
    }

    /// Snapshot projection enabled — can project immutable snapshots.
    pub(crate) fn snapshot_projection_only() -> Self {
        Self {
            stage: OutputThreadRuntimeQueueOwnerStage::SnapshotProjectionOnly,
            has_entry_collection: false,
            has_mutable_runtime_queue: false,
            can_project_snapshot: true,
            can_project_bridge_input: false,
            can_apply_bridge_result: false,
            adapter_connected: false,
            runner_connected: false,
            has_output_behavior: false,
        }
    }

    /// Bridge input projection enabled — can project bridge inputs.
    pub(crate) fn bridge_input_projection_only() -> Self {
        Self {
            stage: OutputThreadRuntimeQueueOwnerStage::BridgeInputProjectionOnly,
            has_entry_collection: false,
            has_mutable_runtime_queue: false,
            can_project_snapshot: true,
            can_project_bridge_input: true,
            can_apply_bridge_result: false,
            adapter_connected: false,
            runner_connected: false,
            has_output_behavior: false,
        }
    }

    /// Whether this contract has no entry collection.
    pub(crate) fn has_no_entry_collection(self) -> bool {
        !self.has_entry_collection
    }

    /// Whether this contract has no mutable runtime queue.
    pub(crate) fn has_no_mutable_runtime_queue(self) -> bool {
        !self.has_mutable_runtime_queue
    }

    /// Whether this contract has no output behavior.
    pub(crate) fn has_no_output_behavior(self) -> bool {
        !self.has_output_behavior
    }
}
