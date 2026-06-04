//! Runtime queue owner report — ownership capability summary.
//!
//! Records contract creation and capability flags.
//! Does not store entry collections, mutable queues, or real resources.

use super::output_thread_runtime_queue_owner_contract::OutputThreadRuntimeQueueOwnerContract;

/// Report summarizing runtime queue owner capabilities.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueOwnerReport {
    pub contracts_created: usize,
    pub snapshot_projection_allowed: bool,
    pub bridge_input_projection_allowed: bool,
    pub bridge_result_apply_allowed: bool,
    pub entry_collection_owned: bool,
    pub mutable_queue_owned: bool,
    pub adapter_connected: bool,
    pub runner_connected: bool,
    pub has_output_behavior: bool,
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueOwnerReport {
    /// Create an empty report with no contracts.
    pub(crate) fn empty() -> Self {
        Self {
            contracts_created: 0,
            snapshot_projection_allowed: false,
            bridge_input_projection_allowed: false,
            bridge_result_apply_allowed: false,
            entry_collection_owned: false,
            mutable_queue_owned: false,
            adapter_connected: false,
            runner_connected: false,
            has_output_behavior: false,
        }
    }

    /// Record a contract and return updated report.
    pub(crate) fn record_contract(self, contract: OutputThreadRuntimeQueueOwnerContract) -> Self {
        Self {
            contracts_created: self.contracts_created + 1,
            snapshot_projection_allowed: contract.can_project_snapshot,
            bridge_input_projection_allowed: contract.can_project_bridge_input,
            bridge_result_apply_allowed: contract.can_apply_bridge_result,
            entry_collection_owned: contract.has_entry_collection,
            mutable_queue_owned: contract.has_mutable_runtime_queue,
            adapter_connected: contract.adapter_connected,
            runner_connected: contract.runner_connected,
            has_output_behavior: contract.has_output_behavior,
        }
    }

    /// Whether this report has no entry collection.
    pub(crate) fn has_no_entry_collection(self) -> bool {
        !self.entry_collection_owned
    }

    /// Whether this report has no mutable queue.
    pub(crate) fn has_no_mutable_queue(self) -> bool {
        !self.mutable_queue_owned
    }

    /// Whether this report has no adapter connection.
    pub(crate) fn has_no_adapter_connection(self) -> bool {
        !self.adapter_connected
    }

    /// Whether this report has no runner connection.
    pub(crate) fn has_no_runner_connection(self) -> bool {
        !self.runner_connected
    }

    /// Whether this report has no output behavior.
    pub(crate) fn has_no_output_behavior(self) -> bool {
        !self.has_output_behavior
    }
}
