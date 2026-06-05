//! Worker runtime queue result decision pure type.
//!
//! Describes the outcome of mapping a queue bridge result
//! through the runtime loop step plan. Does not reference
//! transport channels, queue bridge plan, or audio/output primitives.

use super::super::runtime_loop::state::OutputThreadRuntimeLoopState;
use super::super::runtime_loop::step::OutputThreadRuntimeLoopStepAction;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerRuntimeQueueResultDecisionKind {
    /// An accepted queue bridge result was handled.
    AcceptedResultHandled,
    /// A rejected queue bridge result was handled.
    RejectedResultHandled,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerRuntimeQueueResultDecision {
    pub kind: OutputThreadWorkerRuntimeQueueResultDecisionKind,
    pub runtime_action: OutputThreadRuntimeLoopStepAction,
    pub runtime_next_state: OutputThreadRuntimeLoopState,
    pub should_continue: bool,
    pub passes_from_queue_bridge: bool,
    pub has_queue_ownership: bool,
    pub has_output_behavior: bool,
}

impl OutputThreadWorkerRuntimeQueueResultDecision {
    /// Create a decision from an accepted queue bridge result.
    #[allow(dead_code)]
    pub(crate) fn accepted(
        runtime_action: OutputThreadRuntimeLoopStepAction,
        runtime_next_state: OutputThreadRuntimeLoopState,
        should_continue: bool,
    ) -> Self {
        Self {
            kind: OutputThreadWorkerRuntimeQueueResultDecisionKind::AcceptedResultHandled,
            runtime_action,
            runtime_next_state,
            should_continue,
            passes_from_queue_bridge: true,
            has_queue_ownership: false,
            has_output_behavior: false,
        }
    }

    /// Create a decision from a rejected queue bridge result.
    #[allow(dead_code)]
    pub(crate) fn rejected(
        runtime_action: OutputThreadRuntimeLoopStepAction,
        runtime_next_state: OutputThreadRuntimeLoopState,
        should_continue: bool,
    ) -> Self {
        Self {
            kind: OutputThreadWorkerRuntimeQueueResultDecisionKind::RejectedResultHandled,
            runtime_action,
            runtime_next_state,
            should_continue,
            passes_from_queue_bridge: true,
            has_queue_ownership: false,
            has_output_behavior: false,
        }
    }

    /// Always true — always passes from queue bridge in this adapter.
    #[allow(dead_code)]
    pub(crate) fn passes_from_queue_bridge(self) -> bool {
        self.passes_from_queue_bridge
    }

    /// Always false — no queue ownership in this adapter.
    #[allow(dead_code)]
    pub(crate) fn has_no_queue_ownership(self) -> bool {
        !self.has_queue_ownership
    }

    /// Always true — no real output behavior in this adapter.
    #[allow(dead_code)]
    pub(crate) fn has_no_output_behavior(self) -> bool {
        !self.has_output_behavior
    }
}
