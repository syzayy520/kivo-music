//! Worker queue bridge report pure type.
//!
//! Accumulates counters for a worker queue bridge adapter run.
//! Does not store command payloads, queue state, or buffer data.

use super::output_thread_worker_queue_bridge_adapter::OutputThreadWorkerQueueBridgeAdapterResult;
use super::output_thread_worker_queue_bridge_decision::OutputThreadWorkerQueueBridgeDecisionKind;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerQueueBridgeReport {
    pub intents_observed: usize,
    pub accepted_count: usize,
    pub rejected_count: usize,
    pub unsupported_count: usize,
    pub queue_bridge_used: bool,
    pub passes_to_runtime_loop: bool,
    pub has_output_behavior: bool,
}

impl OutputThreadWorkerQueueBridgeReport {
    /// Create an empty report.
    #[allow(dead_code)]
    pub(crate) fn empty() -> Self {
        Self {
            intents_observed: 0,
            accepted_count: 0,
            rejected_count: 0,
            unsupported_count: 0,
            queue_bridge_used: false,
            passes_to_runtime_loop: false,
            has_output_behavior: false,
        }
    }

    /// Record a single adapter result and return the updated report.
    #[allow(dead_code)]
    pub(crate) fn record_adapter_result(
        self,
        result: OutputThreadWorkerQueueBridgeAdapterResult,
    ) -> Self {
        let (accepted_count, rejected_count, unsupported_count) =
            match result.decision.kind {
                OutputThreadWorkerQueueBridgeDecisionKind::IntentAccepted => {
                    (self.accepted_count + 1, self.rejected_count, self.unsupported_count)
                }
                OutputThreadWorkerQueueBridgeDecisionKind::IntentRejected => {
                    (self.accepted_count, self.rejected_count + 1, self.unsupported_count)
                }
                OutputThreadWorkerQueueBridgeDecisionKind::IntentUnsupported => {
                    (self.accepted_count, self.rejected_count, self.unsupported_count + 1)
                }
            };

        Self {
            intents_observed: self.intents_observed + 1,
            accepted_count,
            rejected_count,
            unsupported_count,
            queue_bridge_used: self.queue_bridge_used || result.queue_bridge_used,
            passes_to_runtime_loop: self.passes_to_runtime_loop || result.passes_to_runtime_loop,
            has_output_behavior: self.has_output_behavior || result.has_output_behavior,
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

    /// Always true — does not pass to runtime loop.
    #[allow(dead_code)]
    pub(crate) fn does_not_pass_to_runtime_loop(self) -> bool {
        !self.passes_to_runtime_loop
    }
}
