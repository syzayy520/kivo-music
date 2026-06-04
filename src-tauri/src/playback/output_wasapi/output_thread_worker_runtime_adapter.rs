//! Worker runtime adapter — maps worker step to runtime loop step.
//!
//! Bridges P0-068 worker loop decisions into P0-066 runtime loop
//! step decisions. Uses plan_runtime_loop_step with queue_result=None.
//!
//! Does not poll transport channels, construct queue bridge results,
//! or reference audio/output primitives.

use super::output_thread_render_plan::{OutputThreadRenderAction, OutputThreadRenderPlan};
use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::output_thread_runtime_loop_state::OutputThreadRuntimeLoopState;
use super::output_thread_runtime_loop_plan::plan_runtime_loop_step;
use super::output_thread_runtime_loop_step::{
    OutputThreadRuntimeLoopStepDecision, OutputThreadRuntimeLoopStepInput,
};
use super::output_thread_worker_loop_step::{
    OutputThreadWorkerLoopStepDecision, OutputThreadWorkerLoopStepKind,
};
use super::output_thread_worker_runtime_decision::OutputThreadWorkerRuntimeDecision;

/// Result of adapting a single worker step into a runtime loop step.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerRuntimeAdapterResult {
    pub worker_runtime_decision: OutputThreadWorkerRuntimeDecision,
    pub runtime_loop_decision: OutputThreadRuntimeLoopStepDecision,
    pub queue_bridge_used: bool,
    pub has_output_behavior: bool,
}

/// Adapt a worker loop step decision into a runtime loop step decision.
///
/// Does not poll transport channels or construct queue bridge results.
/// Always uses queue_result=None.
#[allow(dead_code)]
pub(crate) fn adapt_worker_step_to_runtime_loop(
    worker_decision: OutputThreadWorkerLoopStepDecision,
    observed_runtime_intent: Option<OutputThreadRuntimeIntent>,
    runtime_state: OutputThreadRuntimeLoopState,
) -> OutputThreadWorkerRuntimeAdapterResult {
    let (wr_decision, render_plan) =
        classify_worker_decision(worker_decision, observed_runtime_intent, runtime_state);

    let step_input =
        OutputThreadRuntimeLoopStepInput::without_queue_result(runtime_state, render_plan);
    let rt_decision = plan_runtime_loop_step(step_input);

    OutputThreadWorkerRuntimeAdapterResult {
        worker_runtime_decision: wr_decision,
        runtime_loop_decision: rt_decision,
        queue_bridge_used: false,
        has_output_behavior: false,
    }
}

fn classify_worker_decision(
    worker_decision: OutputThreadWorkerLoopStepDecision,
    observed_runtime_intent: Option<OutputThreadRuntimeIntent>,
    runtime_state: OutputThreadRuntimeLoopState,
) -> (OutputThreadWorkerRuntimeDecision, OutputThreadRenderPlan) {
    match worker_decision.kind {
        OutputThreadWorkerLoopStepKind::NoCommand => (
            OutputThreadWorkerRuntimeDecision::no_runtime_intent(
                worker_decision.kind,
                worker_decision.should_continue,
            ),
            idle_render_plan(runtime_state),
        ),
        OutputThreadWorkerLoopStepKind::RuntimeIntentHandled => {
            let intent = observed_runtime_intent
                .unwrap_or(OutputThreadRuntimeIntent::Start);
            (
                OutputThreadWorkerRuntimeDecision::runtime_intent_observed(
                    intent,
                    worker_decision.kind,
                ),
                idle_render_plan(runtime_state),
            )
        }
        OutputThreadWorkerLoopStepKind::StopRequested => {
            let intent = observed_runtime_intent
                .unwrap_or(OutputThreadRuntimeIntent::Stop);
            (
                OutputThreadWorkerRuntimeDecision::shutdown_observed(
                    intent,
                    worker_decision.kind,
                ),
                idle_render_plan(runtime_state),
            )
        }
        OutputThreadWorkerLoopStepKind::TransportClosed
        | OutputThreadWorkerLoopStepKind::Disconnected => (
            OutputThreadWorkerRuntimeDecision::transport_closed(worker_decision.kind),
            exit_render_plan(),
        ),
    }
}

fn idle_render_plan(runtime_state: OutputThreadRuntimeLoopState) -> OutputThreadRenderPlan {
    if runtime_state.is_terminal() {
        exit_render_plan()
    } else {
        OutputThreadRenderPlan::default()
    }
}

fn exit_render_plan() -> OutputThreadRenderPlan {
    OutputThreadRenderPlan {
        action: OutputThreadRenderAction::Exit,
        should_exit: true,
        ..OutputThreadRenderPlan::default()
    }
}
