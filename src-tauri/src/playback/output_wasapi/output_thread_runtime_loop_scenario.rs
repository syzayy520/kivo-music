use super::output_thread_render_plan::OutputThreadRenderPlan;
use super::output_thread_runtime_loop_state::OutputThreadRuntimeLoopState;
use super::output_thread_runtime_queue_bridge::OutputThreadRuntimeQueueBridgeResult;

/// Mock-only scenario data. It does not own production runtime resources.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeLoopScenarioKind {
    AcceptedStart,
    RejectedStart,
    RenderAudioPlan,
    RenderSilencePlan,
    AcceptedStop,
    AcceptedClose,
    RenderExitPlan,
    RejectedIntentIgnored,
}

/// Mock-only scenario definition for runtime loop testing.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeLoopScenario {
    pub kind: OutputThreadRuntimeLoopScenarioKind,
    pub name: &'static str,
    pub initial_state: OutputThreadRuntimeLoopState,
    pub queue_results: Vec<OutputThreadRuntimeQueueBridgeResult>,
    pub render_plans: Vec<OutputThreadRenderPlan>,
    pub max_steps: usize,
}

#[allow(dead_code)]
impl OutputThreadRuntimeLoopScenario {
    pub(crate) fn new(
        kind: OutputThreadRuntimeLoopScenarioKind,
        name: &'static str,
        initial_state: OutputThreadRuntimeLoopState,
        queue_results: Vec<OutputThreadRuntimeQueueBridgeResult>,
        render_plans: Vec<OutputThreadRenderPlan>,
        max_steps: usize,
    ) -> Self {
        Self {
            kind,
            name,
            initial_state,
            queue_results,
            render_plans,
            max_steps,
        }
    }

    pub(crate) fn is_pure_scenario(&self) -> bool {
        true
    }

    pub(crate) fn has_queue_results(&self) -> bool {
        !self.queue_results.is_empty()
    }

    pub(crate) fn has_render_plans(&self) -> bool {
        !self.render_plans.is_empty()
    }
}
