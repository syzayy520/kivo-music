//! Worker loop scenario matrix — fixed test scenarios.
//!
//! Provides a fixed array of scenarios exercising the bounded
//! non-blocking worker loop skeleton. Does not hold a channel
//! or reference real thread/audio primitives.

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerLoopScenarioKind {
    /// Channel with no commands.
    EmptyChannel,
    /// Channel with a single runtime intent command.
    SingleRuntimeIntent,
    /// Channel with a close-transport command.
    CloseTransport,
    /// Config with zero step budget.
    ZeroStepBudget,
    /// Config with exactly one step budget.
    OneStepBudget,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerLoopScenario {
    pub kind: OutputThreadWorkerLoopScenarioKind,
    pub name: &'static str,
    pub max_steps: usize,
    pub seed_close_command: bool,
    pub seed_runtime_intent: bool,
}

/// Return the fixed 5-scenario matrix.
#[allow(dead_code)]
pub(crate) fn worker_loop_scenario_matrix() -> [OutputThreadWorkerLoopScenario; 5] {
    [
        OutputThreadWorkerLoopScenario {
            kind: OutputThreadWorkerLoopScenarioKind::EmptyChannel,
            name: "empty_channel",
            max_steps: 10,
            seed_close_command: false,
            seed_runtime_intent: false,
        },
        OutputThreadWorkerLoopScenario {
            kind: OutputThreadWorkerLoopScenarioKind::SingleRuntimeIntent,
            name: "single_runtime_intent",
            max_steps: 10,
            seed_close_command: false,
            seed_runtime_intent: true,
        },
        OutputThreadWorkerLoopScenario {
            kind: OutputThreadWorkerLoopScenarioKind::CloseTransport,
            name: "close_transport",
            max_steps: 10,
            seed_close_command: true,
            seed_runtime_intent: false,
        },
        OutputThreadWorkerLoopScenario {
            kind: OutputThreadWorkerLoopScenarioKind::ZeroStepBudget,
            name: "zero_step_budget",
            max_steps: 0,
            seed_close_command: false,
            seed_runtime_intent: false,
        },
        OutputThreadWorkerLoopScenario {
            kind: OutputThreadWorkerLoopScenarioKind::OneStepBudget,
            name: "one_step_budget",
            max_steps: 1,
            seed_close_command: false,
            seed_runtime_intent: false,
        },
    ]
}
