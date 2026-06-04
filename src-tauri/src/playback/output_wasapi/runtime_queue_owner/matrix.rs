//! Runtime queue owner scenario matrix — fixed test scenarios.
//!
//! Provides a fixed array of scenarios exercising the runtime queue
//! owner contract. Does not hold queue state or transport channels.

/// Scenario kind for the runtime queue owner matrix.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeQueueOwnerScenarioKind {
    /// Contract only — no projections.
    ContractOnly,
    /// Snapshot projection allowed.
    SnapshotProjectionAllowed,
    /// Bridge input projection allowed.
    BridgeInputProjectionAllowed,
    /// Bridge result apply denied.
    BridgeResultApplyDenied,
    /// Entry collection denied.
    EntryCollectionDenied,
    /// Adapter connection denied.
    AdapterConnectionDenied,
    /// Runner connection denied.
    RunnerConnectionDenied,
}

/// A single scenario in the runtime queue owner matrix.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueOwnerScenario {
    pub kind: OutputThreadRuntimeQueueOwnerScenarioKind,
    pub name: &'static str,
    pub expects_entry_collection: bool,
    pub expects_mutable_queue: bool,
    pub expects_snapshot_projection: bool,
    pub expects_bridge_input_projection: bool,
    pub expects_bridge_result_apply: bool,
    pub expects_adapter_connection: bool,
    pub expects_runner_connection: bool,
    pub expects_output_behavior: bool,
}

/// Return the fixed 7-scenario matrix.
#[allow(dead_code)]
pub(crate) fn runtime_queue_owner_scenario_matrix()
-> [OutputThreadRuntimeQueueOwnerScenario; 7] {
    [
        OutputThreadRuntimeQueueOwnerScenario {
            kind: OutputThreadRuntimeQueueOwnerScenarioKind::ContractOnly,
            name: "contract_only",
            expects_entry_collection: false,
            expects_mutable_queue: false,
            expects_snapshot_projection: false,
            expects_bridge_input_projection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
        OutputThreadRuntimeQueueOwnerScenario {
            kind: OutputThreadRuntimeQueueOwnerScenarioKind::SnapshotProjectionAllowed,
            name: "snapshot_projection_allowed",
            expects_entry_collection: false,
            expects_mutable_queue: false,
            expects_snapshot_projection: true,
            expects_bridge_input_projection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
        OutputThreadRuntimeQueueOwnerScenario {
            kind: OutputThreadRuntimeQueueOwnerScenarioKind::BridgeInputProjectionAllowed,
            name: "bridge_input_projection_allowed",
            expects_entry_collection: false,
            expects_mutable_queue: false,
            expects_snapshot_projection: true,
            expects_bridge_input_projection: true,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
        OutputThreadRuntimeQueueOwnerScenario {
            kind: OutputThreadRuntimeQueueOwnerScenarioKind::BridgeResultApplyDenied,
            name: "bridge_result_apply_denied",
            expects_entry_collection: false,
            expects_mutable_queue: false,
            expects_snapshot_projection: false,
            expects_bridge_input_projection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
        OutputThreadRuntimeQueueOwnerScenario {
            kind: OutputThreadRuntimeQueueOwnerScenarioKind::EntryCollectionDenied,
            name: "entry_collection_denied",
            expects_entry_collection: false,
            expects_mutable_queue: false,
            expects_snapshot_projection: false,
            expects_bridge_input_projection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
        OutputThreadRuntimeQueueOwnerScenario {
            kind: OutputThreadRuntimeQueueOwnerScenarioKind::AdapterConnectionDenied,
            name: "adapter_connection_denied",
            expects_entry_collection: false,
            expects_mutable_queue: false,
            expects_snapshot_projection: false,
            expects_bridge_input_projection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
        OutputThreadRuntimeQueueOwnerScenario {
            kind: OutputThreadRuntimeQueueOwnerScenarioKind::RunnerConnectionDenied,
            name: "runner_connection_denied",
            expects_entry_collection: false,
            expects_mutable_queue: false,
            expects_snapshot_projection: false,
            expects_bridge_input_projection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
    ]
}
