use super::super::output_thread_core::render_plan::OutputThreadRenderAction;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::state::OutputThreadRuntimeLoopState;
use super::step::{
    OutputThreadRuntimeLoopStepAction, OutputThreadRuntimeLoopStepDecision,
    OutputThreadRuntimeLoopStepInput,
};
use super::super::runtime_queue_bridge::bridge::OutputThreadRuntimeQueueBridgeResult;

/// Plan a single runtime loop step.
#[allow(dead_code)]
pub(crate) fn plan_runtime_loop_step(
    input: OutputThreadRuntimeLoopStepInput,
) -> OutputThreadRuntimeLoopStepDecision {
    if let Some(result) = input.queue_result {
        return plan_with_queue_result(input.state, result);
    }
    plan_without_queue_result(input.state, input.render_plan.action)
}

fn plan_with_queue_result(
    state: OutputThreadRuntimeLoopState,
    result: OutputThreadRuntimeQueueBridgeResult,
) -> OutputThreadRuntimeLoopStepDecision {
    match result {
        OutputThreadRuntimeQueueBridgeResult::Accepted(accepted) => {
            plan_accepted_intent(state, accepted.entry.intent)
        }
        OutputThreadRuntimeQueueBridgeResult::Rejected(_) => {
            OutputThreadRuntimeLoopStepDecision::new(
                state,
                OutputThreadRuntimeLoopStepAction::IgnoreRejectedIntent,
                true,
            )
        }
    }
}

fn plan_accepted_intent(
    state: OutputThreadRuntimeLoopState,
    intent: OutputThreadRuntimeIntent,
) -> OutputThreadRuntimeLoopStepDecision {
    match intent {
        OutputThreadRuntimeIntent::Start if state.can_accept_start() => {
            OutputThreadRuntimeLoopStepDecision::new(
                OutputThreadRuntimeLoopState::Active,
                OutputThreadRuntimeLoopStepAction::EnterActive,
                true,
            )
        }
        OutputThreadRuntimeIntent::Stop if state.can_accept_stop() => {
            OutputThreadRuntimeLoopStepDecision::new(
                OutputThreadRuntimeLoopState::Stopping,
                OutputThreadRuntimeLoopStepAction::MarkStopping,
                true,
            )
        }
        OutputThreadRuntimeIntent::Close => OutputThreadRuntimeLoopStepDecision::new(
            OutputThreadRuntimeLoopState::Exited,
            OutputThreadRuntimeLoopStepAction::MarkExited,
            false,
        ),
        _ => OutputThreadRuntimeLoopStepDecision::new(
            state,
            OutputThreadRuntimeLoopStepAction::FollowRenderPlan,
            true,
        ),
    }
}

fn plan_without_queue_result(
    state: OutputThreadRuntimeLoopState,
    action: OutputThreadRenderAction,
) -> OutputThreadRuntimeLoopStepDecision {
    match action {
        OutputThreadRenderAction::Exit => OutputThreadRuntimeLoopStepDecision::new(
            OutputThreadRuntimeLoopState::Exited,
            OutputThreadRuntimeLoopStepAction::MarkExited,
            false,
        ),
        OutputThreadRenderAction::Sleep if state == OutputThreadRuntimeLoopState::Idle => {
            OutputThreadRuntimeLoopStepDecision::new(
                OutputThreadRuntimeLoopState::Idle,
                OutputThreadRuntimeLoopStepAction::StayIdle,
                true,
            )
        }
        _ => OutputThreadRuntimeLoopStepDecision::new(
            state,
            OutputThreadRuntimeLoopStepAction::FollowRenderPlan,
            true,
        ),
    }
}
