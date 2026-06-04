//! Worker runtime queue result scenario matrix — fixed test scenarios.
//!
//! Provides a fixed array of scenarios exercising the worker runtime
//! queue result adapter. Does not hold queue state or transport channels.

use super::output_thread_runtime_loop_state::OutputThreadRuntimeLoopState;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerRuntimeQueueResultScenarioKind {
    /// Accepted Start intent from Idle state.
    AcceptedStartFromIdle,
    /// Accepted Stop intent from Active state.
    AcceptedStopFromActive,
    /// Accepted Close intent from Active state.
    AcceptedCloseFromActive,
    /// Rejected result from Active state.
    RejectedFromActive,
    /// Accepted Start intent from Active state (cannot accept start).
    AcceptedStartFromActive,
    /// Accepted Stop intent from Idle state (cannot accept stop).
    AcceptedStopFromIdle,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerRuntimeQueueResultScenario {
    pub kind: OutputThreadWorkerRuntimeQueueResultScenarioKind,
    pub name: &'static str,
    pub runtime_state: OutputThreadRuntimeLoopState,
    pub queue_result_is_accepted: bool,
    pub expects_queue_bridge_pass: bool,
    pub expects_queue_ownership: bool,
    pub expects_output_behavior: bool,
}

/// Return the fixed 6-scenario matrix.
#[allow(dead_code)]
pub(crate) fn worker_runtime_queue_result_scenario_matrix(
) -> [OutputThreadWorkerRuntimeQueueResultScenario; 6] {
    [
        OutputThreadWorkerRuntimeQueueResultScenario {
            kind: OutputThreadWorkerRuntimeQueueResultScenarioKind::AcceptedStartFromIdle,
            name: "accepted_start_from_idle",
            runtime_state: OutputThreadRuntimeLoopState::Idle,
            queue_result_is_accepted: true,
            expects_queue_bridge_pass: true,
            expects_queue_ownership: false,
            expects_output_behavior: false,
        },
        OutputThreadWorkerRuntimeQueueResultScenario {
            kind: OutputThreadWorkerRuntimeQueueResultScenarioKind::AcceptedStopFromActive,
            name: "accepted_stop_from_active",
            runtime_state: OutputThreadRuntimeLoopState::Active,
            queue_result_is_accepted: true,
            expects_queue_bridge_pass: true,
            expects_queue_ownership: false,
            expects_output_behavior: false,
        },
        OutputThreadWorkerRuntimeQueueResultScenario {
            kind: OutputThreadWorkerRuntimeQueueResultScenarioKind::AcceptedCloseFromActive,
            name: "accepted_close_from_active",
            runtime_state: OutputThreadRuntimeLoopState::Active,
            queue_result_is_accepted: true,
            expects_queue_bridge_pass: true,
            expects_queue_ownership: false,
            expects_output_behavior: false,
        },
        OutputThreadWorkerRuntimeQueueResultScenario {
            kind: OutputThreadWorkerRuntimeQueueResultScenarioKind::RejectedFromActive,
            name: "rejected_from_active",
            runtime_state: OutputThreadRuntimeLoopState::Active,
            queue_result_is_accepted: false,
            expects_queue_bridge_pass: true,
            expects_queue_ownership: false,
            expects_output_behavior: false,
        },
        OutputThreadWorkerRuntimeQueueResultScenario {
            kind: OutputThreadWorkerRuntimeQueueResultScenarioKind::AcceptedStartFromActive,
            name: "accepted_start_from_active",
            runtime_state: OutputThreadRuntimeLoopState::Active,
            queue_result_is_accepted: true,
            expects_queue_bridge_pass: true,
            expects_queue_ownership: false,
            expects_output_behavior: false,
        },
        OutputThreadWorkerRuntimeQueueResultScenario {
            kind: OutputThreadWorkerRuntimeQueueResultScenarioKind::AcceptedStopFromIdle,
            name: "accepted_stop_from_idle",
            runtime_state: OutputThreadRuntimeLoopState::Idle,
            queue_result_is_accepted: true,
            expects_queue_bridge_pass: true,
            expects_queue_ownership: false,
            expects_output_behavior: false,
        },
    ]
}
