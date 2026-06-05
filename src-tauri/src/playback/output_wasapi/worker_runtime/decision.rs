//! Worker runtime decision pure type.
//!
//! Describes the outcome of mapping a worker loop step decision
//! into a runtime-aware decision. Does not reference transport
//! channels, queue bridge, or audio/output primitives.

use super::super::runtime_core::intent::OutputThreadRuntimeIntent;
use super::super::worker_loop::step::OutputThreadWorkerLoopStepKind;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerRuntimeDecisionKind {
    /// No runtime intent was observed in this step.
    NoRuntimeIntent,
    /// A non-shutdown runtime intent was observed.
    RuntimeIntentObserved,
    /// A shutdown-type runtime intent was observed.
    RuntimeShutdownObserved,
    /// The worker transport channel closed or disconnected.
    WorkerTransportClosed,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerRuntimeDecision {
    pub kind: OutputThreadWorkerRuntimeDecisionKind,
    pub worker_kind: OutputThreadWorkerLoopStepKind,
    pub observed_runtime_intent: Option<OutputThreadRuntimeIntent>,
    pub queue_bridge_used: bool,
    pub has_output_behavior: bool,
    pub should_continue_scaffold: bool,
}

impl OutputThreadWorkerRuntimeDecision {
    /// No runtime intent in this step (e.g., empty poll).
    #[allow(dead_code)]
    pub(crate) fn no_runtime_intent(
        worker_kind: OutputThreadWorkerLoopStepKind,
        should_continue: bool,
    ) -> Self {
        Self {
            kind: OutputThreadWorkerRuntimeDecisionKind::NoRuntimeIntent,
            worker_kind,
            observed_runtime_intent: None,
            queue_bridge_used: false,
            has_output_behavior: false,
            should_continue_scaffold: should_continue,
        }
    }

    /// A non-shutdown runtime intent was observed.
    #[allow(dead_code)]
    pub(crate) fn runtime_intent_observed(
        intent: OutputThreadRuntimeIntent,
        worker_kind: OutputThreadWorkerLoopStepKind,
    ) -> Self {
        Self {
            kind: OutputThreadWorkerRuntimeDecisionKind::RuntimeIntentObserved,
            worker_kind,
            observed_runtime_intent: Some(intent),
            queue_bridge_used: false,
            has_output_behavior: false,
            should_continue_scaffold: true,
        }
    }

    /// A shutdown-type runtime intent was observed.
    #[allow(dead_code)]
    pub(crate) fn shutdown_observed(
        intent: OutputThreadRuntimeIntent,
        worker_kind: OutputThreadWorkerLoopStepKind,
    ) -> Self {
        Self {
            kind: OutputThreadWorkerRuntimeDecisionKind::RuntimeShutdownObserved,
            worker_kind,
            observed_runtime_intent: Some(intent),
            queue_bridge_used: false,
            has_output_behavior: false,
            should_continue_scaffold: true,
        }
    }

    /// Transport channel closed or disconnected.
    #[allow(dead_code)]
    pub(crate) fn transport_closed(
        worker_kind: OutputThreadWorkerLoopStepKind,
    ) -> Self {
        Self {
            kind: OutputThreadWorkerRuntimeDecisionKind::WorkerTransportClosed,
            worker_kind,
            observed_runtime_intent: None,
            queue_bridge_used: false,
            has_output_behavior: false,
            should_continue_scaffold: false,
        }
    }

    /// Always false — no queue bridge in this adapter.
    #[allow(dead_code)]
    pub(crate) fn uses_queue_bridge(self) -> bool {
        self.queue_bridge_used
    }

    /// Always false — no real output behavior in this adapter.
    #[allow(dead_code)]
    pub(crate) fn has_no_output_behavior(self) -> bool {
        !self.has_output_behavior
    }
}
