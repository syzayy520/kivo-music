use super::super::output_thread_render_plan::OutputThreadRenderPlan;
use super::plan::plan_runtime_loop_step;
use super::scenario::OutputThreadRuntimeLoopScenario;
use super::state::OutputThreadRuntimeLoopState;
use super::step::{
    OutputThreadRuntimeLoopStepDecision, OutputThreadRuntimeLoopStepInput,
};

/// Result of running a mock-only runtime loop scenario.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeLoopScenarioRun {
    pub kind: super::scenario::OutputThreadRuntimeLoopScenarioKind,
    pub name: &'static str,
    pub final_state: OutputThreadRuntimeLoopState,
    pub decisions: Vec<OutputThreadRuntimeLoopStepDecision>,
    pub completed_steps: usize,
    pub stopped_by_decision: bool,
}

/// Plan a runtime loop scenario execution.
#[allow(dead_code)]
pub(crate) fn plan_runtime_loop_scenario(
    scenario: &OutputThreadRuntimeLoopScenario,
) -> OutputThreadRuntimeLoopScenarioRun {
    let mut current_state = scenario.initial_state;
    let mut decisions = Vec::new();
    let mut stopped_by_decision = false;
    let max_queue = scenario.queue_results.len();
    let max_render = scenario.render_plans.len();
    let steps = scenario.max_steps.min(max_queue.max(max_render));

    for index in 0..steps {
        let queue_result = scenario.queue_results.get(index).copied();
        let render_plan = scenario
            .render_plans
            .get(index)
            .copied()
            .unwrap_or(OutputThreadRenderPlan::default());
        let input =
            OutputThreadRuntimeLoopStepInput::new(current_state, queue_result, render_plan);
        let decision = plan_runtime_loop_step(input);
        current_state = decision.next_state;
        decisions.push(decision);
        if !decision.should_continue {
            stopped_by_decision = true;
            break;
        }
    }

    OutputThreadRuntimeLoopScenarioRun {
        kind: scenario.kind,
        name: scenario.name,
        final_state: current_state,
        completed_steps: decisions.len(),
        stopped_by_decision,
        decisions,
    }
}

#[allow(dead_code)]
impl OutputThreadRuntimeLoopScenarioRun {
    pub(crate) fn is_terminal(&self) -> bool {
        self.final_state.is_terminal()
    }

    pub(crate) fn decision_count(&self) -> usize {
        self.decisions.len()
    }

    pub(crate) fn has_decisions(&self) -> bool {
        !self.decisions.is_empty()
    }
}
