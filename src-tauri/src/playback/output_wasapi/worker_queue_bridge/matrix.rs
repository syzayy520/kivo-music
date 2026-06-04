//! Worker queue bridge scenario matrix — fixed test scenarios.
//!
//! Provides a fixed array of scenarios exercising the worker queue bridge
//! adapter. Does not hold transport channels or queue bridge state.

use super::super::output_thread_runtime_intent::OutputThreadRuntimeIntent;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerQueueBridgeScenarioKind {
    /// Start intent through queue bridge.
    StartIntent,
    /// Stop intent through queue bridge.
    StopIntent,
    /// Close intent through queue bridge.
    CloseIntent,
    /// Reset device intent through queue bridge.
    ResetDeviceIntent,
    /// Queue at capacity (full).
    FullQueue,
    /// Queue closed.
    ClosedQueue,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerQueueBridgeScenario {
    pub kind: OutputThreadWorkerQueueBridgeScenarioKind,
    pub name: &'static str,
    pub intent: OutputThreadRuntimeIntent,
    pub expects_queue_bridge: bool,
    pub expects_output_behavior: bool,
    pub expects_runtime_loop_pass: bool,
}

/// Return the fixed 6-scenario matrix.
#[allow(dead_code)]
pub(crate) fn worker_queue_bridge_scenario_matrix() -> [OutputThreadWorkerQueueBridgeScenario; 6] {
    [
        OutputThreadWorkerQueueBridgeScenario {
            kind: OutputThreadWorkerQueueBridgeScenarioKind::StartIntent,
            name: "start_intent",
            intent: OutputThreadRuntimeIntent::Start,
            expects_queue_bridge: true,
            expects_output_behavior: false,
            expects_runtime_loop_pass: false,
        },
        OutputThreadWorkerQueueBridgeScenario {
            kind: OutputThreadWorkerQueueBridgeScenarioKind::StopIntent,
            name: "stop_intent",
            intent: OutputThreadRuntimeIntent::Stop,
            expects_queue_bridge: true,
            expects_output_behavior: false,
            expects_runtime_loop_pass: false,
        },
        OutputThreadWorkerQueueBridgeScenario {
            kind: OutputThreadWorkerQueueBridgeScenarioKind::CloseIntent,
            name: "close_intent",
            intent: OutputThreadRuntimeIntent::Close,
            expects_queue_bridge: true,
            expects_output_behavior: false,
            expects_runtime_loop_pass: false,
        },
        OutputThreadWorkerQueueBridgeScenario {
            kind: OutputThreadWorkerQueueBridgeScenarioKind::ResetDeviceIntent,
            name: "reset_device_intent",
            intent: OutputThreadRuntimeIntent::ResetDevice,
            expects_queue_bridge: true,
            expects_output_behavior: false,
            expects_runtime_loop_pass: false,
        },
        OutputThreadWorkerQueueBridgeScenario {
            kind: OutputThreadWorkerQueueBridgeScenarioKind::FullQueue,
            name: "full_queue",
            intent: OutputThreadRuntimeIntent::Start,
            expects_queue_bridge: true,
            expects_output_behavior: false,
            expects_runtime_loop_pass: false,
        },
        OutputThreadWorkerQueueBridgeScenario {
            kind: OutputThreadWorkerQueueBridgeScenarioKind::ClosedQueue,
            name: "closed_queue",
            intent: OutputThreadRuntimeIntent::Start,
            expects_queue_bridge: true,
            expects_output_behavior: false,
            expects_runtime_loop_pass: false,
        },
    ]
}
