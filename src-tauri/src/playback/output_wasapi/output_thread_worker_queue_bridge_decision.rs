//! Worker queue bridge decision pure type.
//!
//! Describes the outcome of mapping an observed runtime intent
//! through the queue bridge plan. Does not reference transport
//! channels, runtime loop, or audio/output primitives.

use super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::output_thread_runtime_queue_bridge_result::OutputThreadRuntimeQueueBridgeResult;

#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerQueueBridgeDecisionKind {
    /// The queue bridge accepted the intent.
    IntentAccepted,
    /// The queue bridge rejected the intent.
    IntentRejected,
    /// The queue bridge returned an unsupported result.
    IntentUnsupported,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerQueueBridgeDecision {
    pub kind: OutputThreadWorkerQueueBridgeDecisionKind,
    pub observed_runtime_intent: OutputThreadRuntimeIntent,
    pub queue_bridge_used: bool,
    pub accepted: bool,
    pub rejected: bool,
    pub passes_to_runtime_loop: bool,
    pub has_output_behavior: bool,
}

impl OutputThreadWorkerQueueBridgeDecision {
    /// Create a decision from a queue bridge result.
    #[allow(dead_code)]
    pub(crate) fn from_bridge_result(
        intent: OutputThreadRuntimeIntent,
        result: OutputThreadRuntimeQueueBridgeResult,
    ) -> Self {
        match result {
            OutputThreadRuntimeQueueBridgeResult::Accepted(_) => Self {
                kind: OutputThreadWorkerQueueBridgeDecisionKind::IntentAccepted,
                observed_runtime_intent: intent,
                queue_bridge_used: true,
                accepted: true,
                rejected: false,
                passes_to_runtime_loop: false,
                has_output_behavior: false,
            },
            OutputThreadRuntimeQueueBridgeResult::Rejected(_) => Self {
                kind: OutputThreadWorkerQueueBridgeDecisionKind::IntentRejected,
                observed_runtime_intent: intent,
                queue_bridge_used: true,
                accepted: false,
                rejected: true,
                passes_to_runtime_loop: false,
                has_output_behavior: false,
            },
        }
    }

    /// Always true — queue bridge is always used in this adapter.
    #[allow(dead_code)]
    pub(crate) fn uses_queue_bridge(self) -> bool {
        self.queue_bridge_used
    }

    /// Always true — no real output behavior in this adapter.
    #[allow(dead_code)]
    pub(crate) fn has_no_output_behavior(self) -> bool {
        !self.has_output_behavior
    }

    /// Always false — does not pass to runtime loop.
    #[allow(dead_code)]
    pub(crate) fn does_not_pass_to_runtime_loop(self) -> bool {
        !self.passes_to_runtime_loop
    }
}
