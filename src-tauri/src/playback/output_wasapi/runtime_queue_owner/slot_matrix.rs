//! Runtime queue owner slot scenario matrix — fixed test scenarios.
//!
//! Provides a fixed array of scenarios exercising the runtime queue
//! owner fixed slots. Does not use dynamic collections or adapters.

/// Scenario kind for the runtime queue owner slot matrix.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeQueueOwnerSlotScenarioKind {
    /// All slots empty.
    EmptySlots,
    /// Single slot occupied.
    SingleOccupiedSlot,
    /// All slots full.
    FullSlots,
    /// Snapshot projection allowed.
    SnapshotProjection,
    /// Bridge result apply denied.
    BridgeResultApplyDenied,
    /// Adapter connection denied.
    AdapterConnectionDenied,
    /// Runner connection denied.
    RunnerConnectionDenied,
}

/// A single scenario in the runtime queue owner slot matrix.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueOwnerSlotScenario {
    pub kind: OutputThreadRuntimeQueueOwnerSlotScenarioKind,
    pub name: &'static str,
    pub expects_fixed_slots: bool,
    pub expects_dynamic_collection: bool,
    pub expects_bridge_result_apply: bool,
    pub expects_adapter_connection: bool,
    pub expects_runner_connection: bool,
    pub expects_output_behavior: bool,
}

/// Return the fixed 7-scenario matrix.
#[allow(dead_code)]
pub(crate) fn runtime_queue_owner_slot_scenario_matrix()
-> [OutputThreadRuntimeQueueOwnerSlotScenario; 7] {
    [
        OutputThreadRuntimeQueueOwnerSlotScenario {
            kind: OutputThreadRuntimeQueueOwnerSlotScenarioKind::EmptySlots,
            name: "empty_slots",
            expects_fixed_slots: true,
            expects_dynamic_collection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
        OutputThreadRuntimeQueueOwnerSlotScenario {
            kind: OutputThreadRuntimeQueueOwnerSlotScenarioKind::SingleOccupiedSlot,
            name: "single_occupied_slot",
            expects_fixed_slots: true,
            expects_dynamic_collection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
        OutputThreadRuntimeQueueOwnerSlotScenario {
            kind: OutputThreadRuntimeQueueOwnerSlotScenarioKind::FullSlots,
            name: "full_slots",
            expects_fixed_slots: true,
            expects_dynamic_collection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
        OutputThreadRuntimeQueueOwnerSlotScenario {
            kind: OutputThreadRuntimeQueueOwnerSlotScenarioKind::SnapshotProjection,
            name: "snapshot_projection",
            expects_fixed_slots: true,
            expects_dynamic_collection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
        OutputThreadRuntimeQueueOwnerSlotScenario {
            kind: OutputThreadRuntimeQueueOwnerSlotScenarioKind::BridgeResultApplyDenied,
            name: "bridge_result_apply_denied",
            expects_fixed_slots: true,
            expects_dynamic_collection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
        OutputThreadRuntimeQueueOwnerSlotScenario {
            kind: OutputThreadRuntimeQueueOwnerSlotScenarioKind::AdapterConnectionDenied,
            name: "adapter_connection_denied",
            expects_fixed_slots: true,
            expects_dynamic_collection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
        OutputThreadRuntimeQueueOwnerSlotScenario {
            kind: OutputThreadRuntimeQueueOwnerSlotScenarioKind::RunnerConnectionDenied,
            name: "runner_connection_denied",
            expects_fixed_slots: true,
            expects_dynamic_collection: false,
            expects_bridge_result_apply: false,
            expects_adapter_connection: false,
            expects_runner_connection: false,
            expects_output_behavior: false,
        },
    ]
}
