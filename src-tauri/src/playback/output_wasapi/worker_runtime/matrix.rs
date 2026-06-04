//! Worker runtime scenario matrix — fixed test scenarios.
//!
//! Provides a fixed array of scenarios exercising the worker-runtime
//! adapter. Does not hold transport channels or queue bridge state.

use super::super::worker_loop::step::OutputThreadWorkerLoopStepKind;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerRuntimeScenarioKind {
    /// No command from transport (empty poll).
    NoCommand,
    /// A non-shutdown runtime intent was observed.
    RuntimeIntentObserved,
    /// A shutdown-type runtime intent was observed.
    ShutdownIntentObserved,
    /// The worker transport channel closed or disconnected.
    WorkerTransportClosed,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerRuntimeScenario {
    pub kind: OutputThreadWorkerRuntimeScenarioKind,
    pub name: &'static str,
    pub worker_kind: OutputThreadWorkerLoopStepKind,
    pub has_runtime_intent: bool,
    pub expects_queue_bridge: bool,
    pub expects_output_behavior: bool,
}

/// Return the fixed 4-scenario matrix.
#[allow(dead_code)]
pub(crate) fn worker_runtime_scenario_matrix() -> [OutputThreadWorkerRuntimeScenario; 4] {
    [
        OutputThreadWorkerRuntimeScenario {
            kind: OutputThreadWorkerRuntimeScenarioKind::NoCommand,
            name: "no_command",
            worker_kind: OutputThreadWorkerLoopStepKind::NoCommand,
            has_runtime_intent: false,
            expects_queue_bridge: false,
            expects_output_behavior: false,
        },
        OutputThreadWorkerRuntimeScenario {
            kind: OutputThreadWorkerRuntimeScenarioKind::RuntimeIntentObserved,
            name: "runtime_intent_observed",
            worker_kind: OutputThreadWorkerLoopStepKind::RuntimeIntentHandled,
            has_runtime_intent: true,
            expects_queue_bridge: false,
            expects_output_behavior: false,
        },
        OutputThreadWorkerRuntimeScenario {
            kind: OutputThreadWorkerRuntimeScenarioKind::ShutdownIntentObserved,
            name: "shutdown_intent_observed",
            worker_kind: OutputThreadWorkerLoopStepKind::StopRequested,
            has_runtime_intent: true,
            expects_queue_bridge: false,
            expects_output_behavior: false,
        },
        OutputThreadWorkerRuntimeScenario {
            kind: OutputThreadWorkerRuntimeScenarioKind::WorkerTransportClosed,
            name: "worker_transport_closed",
            worker_kind: OutputThreadWorkerLoopStepKind::TransportClosed,
            has_runtime_intent: false,
            expects_queue_bridge: false,
            expects_output_behavior: false,
        },
    ]
}
