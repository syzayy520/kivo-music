//! Worker runtime queue result adapter — maps queue bridge result to runtime loop step.
//!
//! Calls plan_runtime_loop_step with queue_result=Some(existing bridge result).
//! Does not call plan_runtime_queue_bridge_intent, poll transport channels,
//! or reference audio/output primitives.

use super::output_thread_render_plan::OutputThreadRenderPlan;
use super::output_thread_runtime_loop_state::OutputThreadRuntimeLoopState;
use super::output_thread_runtime_loop_step::{
    OutputThreadRuntimeLoopStepDecision, OutputThreadRuntimeLoopStepInput,
};
use super::output_thread_runtime_loop_plan::plan_runtime_loop_step;
use super::output_thread_runtime_queue_bridge_result::OutputThreadRuntimeQueueBridgeResult;
use super::output_thread_worker_runtime_queue_result_decision::OutputThreadWorkerRuntimeQueueResultDecision;

/// Result of adapting a queue bridge result through the runtime loop plan.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerRuntimeQueueResultAdapterResult {
    pub runtime_loop_decision: OutputThreadRuntimeLoopStepDecision,
    pub decision: OutputThreadWorkerRuntimeQueueResultDecision,
    pub passes_from_queue_bridge: bool,
    pub has_queue_ownership: bool,
    pub has_output_behavior: bool,
}

/// Adapt a queue bridge result through the runtime loop step plan.
///
/// Does not call plan_runtime_queue_bridge_intent or poll transport channels.
/// Always returns passes_from_queue_bridge=true, has_queue_ownership=false,
/// has_output_behavior=false.
#[allow(dead_code)]
pub(crate) fn adapt_queue_bridge_result_to_runtime_loop_step(
    queue_result: OutputThreadRuntimeQueueBridgeResult,
    runtime_state: OutputThreadRuntimeLoopState,
) -> OutputThreadWorkerRuntimeQueueResultAdapterResult {
    let step_input = OutputThreadRuntimeLoopStepInput::new(
        runtime_state,
        Some(queue_result),
        OutputThreadRenderPlan::default(),
    );
    let rt_decision = plan_runtime_loop_step(step_input);

    let decision = if queue_result.is_accepted() {
        OutputThreadWorkerRuntimeQueueResultDecision::accepted(
            rt_decision.action,
            rt_decision.next_state,
            rt_decision.should_continue,
        )
    } else {
        OutputThreadWorkerRuntimeQueueResultDecision::rejected(
            rt_decision.action,
            rt_decision.next_state,
            rt_decision.should_continue,
        )
    };

    OutputThreadWorkerRuntimeQueueResultAdapterResult {
        runtime_loop_decision: rt_decision,
        decision,
        passes_from_queue_bridge: true,
        has_queue_ownership: false,
        has_output_behavior: false,
    }
}
