use super::output_thread_render_plan::OutputThreadRenderPlan;
use super::output_thread_runtime_loop_state::OutputThreadRuntimeLoopState;
use super::runtime_queue_bridge::bridge::OutputThreadRuntimeQueueBridgeResult;

/// Input for a single loop step calculation.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeLoopStepInput {
    pub state: OutputThreadRuntimeLoopState,
    pub queue_result: Option<OutputThreadRuntimeQueueBridgeResult>,
    pub render_plan: OutputThreadRenderPlan,
}

#[allow(dead_code)]
impl OutputThreadRuntimeLoopStepInput {
    pub(crate) fn new(
        state: OutputThreadRuntimeLoopState,
        queue_result: Option<OutputThreadRuntimeQueueBridgeResult>,
        render_plan: OutputThreadRenderPlan,
    ) -> Self {
        Self {
            state,
            queue_result,
            render_plan,
        }
    }

    pub(crate) fn without_queue_result(
        state: OutputThreadRuntimeLoopState,
        render_plan: OutputThreadRenderPlan,
    ) -> Self {
        Self {
            state,
            queue_result: None,
            render_plan,
        }
    }
}

/// Action the loop step produces.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeLoopStepAction {
    StayIdle,
    EnterActive,
    MarkStopping,
    MarkStopped,
    MarkExited,
    FollowRenderPlan,
    IgnoreRejectedIntent,
}

/// Decision from a loop step calculation.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeLoopStepDecision {
    pub next_state: OutputThreadRuntimeLoopState,
    pub action: OutputThreadRuntimeLoopStepAction,
    pub should_continue: bool,
}

#[allow(dead_code)]
impl OutputThreadRuntimeLoopStepDecision {
    pub(crate) fn new(
        next_state: OutputThreadRuntimeLoopState,
        action: OutputThreadRuntimeLoopStepAction,
        should_continue: bool,
    ) -> Self {
        Self {
            next_state,
            action,
            should_continue,
        }
    }

    pub(crate) fn continues(self) -> bool {
        self.should_continue
    }

    pub(crate) fn stops(self) -> bool {
        !self.should_continue
    }
}
