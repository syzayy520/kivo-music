//! Worker lifecycle scenario matrix.
//!
//! Provides a fixed set of preflight scenarios covering
//! pure lifecycle planning without real workers or transport.

use super::handle_contract::OutputThreadWorkerHandleContract;
use super::lifecycle::OutputThreadWorkerLifecycleStage;
use super::plan::OutputThreadWorkerLifecycleInput;
use super::shutdown::OutputThreadWorkerShutdownRequest;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerLifecycleScenarioKind {
    ContractOnlyNoRequest,
    ContractOnlyStopRequest,
    ContractOnlyCloseTransport,
    StoppedStopRequest,
    FailedStopRequest,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerLifecycleScenario {
    pub kind: OutputThreadWorkerLifecycleScenarioKind,
    pub name: &'static str,
    pub input: OutputThreadWorkerLifecycleInput,
}

/// Return fixed array of 5 lifecycle preflight scenarios.
#[allow(dead_code)]
pub(crate) fn worker_lifecycle_scenario_matrix() -> [OutputThreadWorkerLifecycleScenario; 5] {
    [
        OutputThreadWorkerLifecycleScenario {
            kind: OutputThreadWorkerLifecycleScenarioKind::ContractOnlyNoRequest,
            name: "contract_only_no_request",
            input: OutputThreadWorkerLifecycleInput {
                handle: OutputThreadWorkerHandleContract::contract_only(),
                request: OutputThreadWorkerShutdownRequest::None,
            },
        },
        OutputThreadWorkerLifecycleScenario {
            kind: OutputThreadWorkerLifecycleScenarioKind::ContractOnlyStopRequest,
            name: "contract_only_stop_request",
            input: OutputThreadWorkerLifecycleInput {
                handle: OutputThreadWorkerHandleContract::contract_only(),
                request: OutputThreadWorkerShutdownRequest::RequestStop,
            },
        },
        OutputThreadWorkerLifecycleScenario {
            kind: OutputThreadWorkerLifecycleScenarioKind::ContractOnlyCloseTransport,
            name: "contract_only_close_transport",
            input: OutputThreadWorkerLifecycleInput {
                handle: OutputThreadWorkerHandleContract::contract_only(),
                request: OutputThreadWorkerShutdownRequest::CloseTransport,
            },
        },
        OutputThreadWorkerLifecycleScenario {
            kind: OutputThreadWorkerLifecycleScenarioKind::StoppedStopRequest,
            name: "stopped_stop_request",
            input: OutputThreadWorkerLifecycleInput {
                handle: OutputThreadWorkerHandleContract {
                    lifecycle: OutputThreadWorkerLifecycleStage::Stopped,
                    ..OutputThreadWorkerHandleContract::contract_only()
                },
                request: OutputThreadWorkerShutdownRequest::RequestStop,
            },
        },
        OutputThreadWorkerLifecycleScenario {
            kind: OutputThreadWorkerLifecycleScenarioKind::FailedStopRequest,
            name: "failed_stop_request",
            input: OutputThreadWorkerLifecycleInput {
                handle: OutputThreadWorkerHandleContract {
                    lifecycle: OutputThreadWorkerLifecycleStage::Failed,
                    ..OutputThreadWorkerHandleContract::contract_only()
                },
                request: OutputThreadWorkerShutdownRequest::RequestStop,
            },
        },
    ]
}
