//! Runtime queue owner state descriptor — stage and transition tracking.
//!
//! Combines existing queue config and counter state with owner stage.
//! Does not hold entry collections, mutable queues, or real resources.

use super::super::output_thread_runtime_queue_config::OutputThreadRuntimeQueueConfig;
use super::contract::OutputThreadRuntimeQueueOwnerStage;
use super::super::output_thread_runtime_queue_state::OutputThreadRuntimeQueueState;

/// Transition kind for the runtime queue owner.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeQueueOwnerTransitionKind {
    /// Initial contract created.
    CreatedContract,
    /// Snapshot projection enabled.
    EnabledSnapshotProjection,
    /// Bridge input projection enabled.
    EnabledBridgeInputProjection,
    /// Bridge result apply denied.
    BridgeResultApplyDenied,
    /// Entry collection denied.
    EntryCollectionDenied,
    /// Adapter connection denied.
    AdapterConnectionDenied,
    /// Runner connection denied.
    RunnerConnectionDenied,
}

/// State descriptor for the runtime queue owner.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueOwnerState {
    pub config: OutputThreadRuntimeQueueConfig,
    pub queue_state: OutputThreadRuntimeQueueState,
    pub stage: OutputThreadRuntimeQueueOwnerStage,
    pub last_transition: OutputThreadRuntimeQueueOwnerTransitionKind,
    pub has_entry_collection: bool,
    pub has_mutable_runtime_queue: bool,
    pub has_output_behavior: bool,
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueOwnerState {
    /// Create a contract-only owner state.
    pub(crate) fn contract_only(
        config: OutputThreadRuntimeQueueConfig,
        queue_state: OutputThreadRuntimeQueueState,
    ) -> Self {
        Self {
            config,
            queue_state,
            stage: OutputThreadRuntimeQueueOwnerStage::ContractOnly,
            last_transition: OutputThreadRuntimeQueueOwnerTransitionKind::CreatedContract,
            has_entry_collection: false,
            has_mutable_runtime_queue: false,
            has_output_behavior: false,
        }
    }

    /// Enable snapshot projection stage.
    pub(crate) fn enable_snapshot_projection(self) -> Self {
        Self {
            stage: OutputThreadRuntimeQueueOwnerStage::SnapshotProjectionOnly,
            last_transition:
                OutputThreadRuntimeQueueOwnerTransitionKind::EnabledSnapshotProjection,
            ..self
        }
    }

    /// Enable bridge input projection stage.
    pub(crate) fn enable_bridge_input_projection(self) -> Self {
        Self {
            stage: OutputThreadRuntimeQueueOwnerStage::BridgeInputProjectionOnly,
            last_transition:
                OutputThreadRuntimeQueueOwnerTransitionKind::EnabledBridgeInputProjection,
            ..self
        }
    }

    /// Deny bridge result apply — returns self unchanged.
    pub(crate) fn deny_bridge_result_apply(self) -> Self {
        Self {
            last_transition: OutputThreadRuntimeQueueOwnerTransitionKind::BridgeResultApplyDenied,
            ..self
        }
    }

    /// Deny entry collection — returns self unchanged.
    pub(crate) fn deny_entry_collection(self) -> Self {
        Self {
            last_transition: OutputThreadRuntimeQueueOwnerTransitionKind::EntryCollectionDenied,
            ..self
        }
    }

    /// Deny adapter connection — returns self unchanged.
    pub(crate) fn deny_adapter_connection(self) -> Self {
        Self {
            last_transition: OutputThreadRuntimeQueueOwnerTransitionKind::AdapterConnectionDenied,
            ..self
        }
    }

    /// Deny runner connection — returns self unchanged.
    pub(crate) fn deny_runner_connection(self) -> Self {
        Self {
            last_transition: OutputThreadRuntimeQueueOwnerTransitionKind::RunnerConnectionDenied,
            ..self
        }
    }

    /// Whether this state has no entry collection.
    pub(crate) fn has_no_entry_collection(self) -> bool {
        !self.has_entry_collection
    }

    /// Whether this state has no mutable runtime queue.
    pub(crate) fn has_no_mutable_runtime_queue(self) -> bool {
        !self.has_mutable_runtime_queue
    }

    /// Whether this state has no output behavior.
    pub(crate) fn has_no_output_behavior(self) -> bool {
        !self.has_output_behavior
    }
}
