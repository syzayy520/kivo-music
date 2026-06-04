//! Worker runtime queue result report pure type.
//!
//! Accumulates counters for a worker runtime queue result adapter run.
//! Does not store command payloads, queue state, or buffer data.

use super::adapter::OutputThreadWorkerRuntimeQueueResultAdapterResult;
use super::decision::OutputThreadWorkerRuntimeQueueResultDecisionKind;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerRuntimeQueueResultReport {
    pub queue_results_seen: usize,
    pub accepted_seen: usize,
    pub rejected_seen: usize,
    pub runtime_decisions: usize,
    pub stops_count: usize,
    pub passes_from_queue_bridge: bool,
    pub has_queue_ownership: bool,
    pub has_output_behavior: bool,
}

impl OutputThreadWorkerRuntimeQueueResultReport {
    /// Create an empty report.
    #[allow(dead_code)]
    pub(crate) fn empty() -> Self {
        Self {
            queue_results_seen: 0,
            accepted_seen: 0,
            rejected_seen: 0,
            runtime_decisions: 0,
            stops_count: 0,
            passes_from_queue_bridge: false,
            has_queue_ownership: false,
            has_output_behavior: false,
        }
    }

    /// Record a single adapter result and return the updated report.
    #[allow(dead_code)]
    pub(crate) fn record_adapter_result(
        self,
        result: OutputThreadWorkerRuntimeQueueResultAdapterResult,
    ) -> Self {
        let (accepted_seen, rejected_seen) = match result.decision.kind {
            OutputThreadWorkerRuntimeQueueResultDecisionKind::AcceptedResultHandled => {
                (self.accepted_seen + 1, self.rejected_seen)
            }
            OutputThreadWorkerRuntimeQueueResultDecisionKind::RejectedResultHandled => {
                (self.accepted_seen, self.rejected_seen + 1)
            }
        };

        let stops_count = if !result.runtime_loop_decision.should_continue {
            self.stops_count + 1
        } else {
            self.stops_count
        };

        Self {
            queue_results_seen: self.queue_results_seen + 1,
            accepted_seen,
            rejected_seen,
            runtime_decisions: self.runtime_decisions + 1,
            stops_count,
            passes_from_queue_bridge: self.passes_from_queue_bridge
                || result.passes_from_queue_bridge,
            has_queue_ownership: self.has_queue_ownership || result.has_queue_ownership,
            has_output_behavior: self.has_output_behavior || result.has_output_behavior,
        }
    }

    /// Always true — always passes from queue bridge in this adapter.
    #[allow(dead_code)]
    pub(crate) fn passes_from_queue_bridge(self) -> bool {
        self.passes_from_queue_bridge
    }

    /// Always true — no queue ownership in this adapter.
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
