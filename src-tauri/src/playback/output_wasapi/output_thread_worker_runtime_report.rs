//! Worker runtime report pure type.
//!
//! Accumulates counters for a worker-runtime adapter run.
//! Does not store command payloads, queue state, or buffer data.

use super::output_thread_worker_loop_report::OutputThreadWorkerLoopReport;
use super::output_thread_worker_runtime_adapter::OutputThreadWorkerRuntimeAdapterResult;
use super::output_thread_worker_runtime_decision::OutputThreadWorkerRuntimeDecisionKind;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerRuntimeReport {
    pub worker_steps: usize,
    pub runtime_decisions: usize,
    pub runtime_intents_observed: usize,
    pub shutdown_intents_observed: usize,
    pub queue_bridge_used: bool,
    pub has_output_behavior: bool,
}

impl OutputThreadWorkerRuntimeReport {
    /// Create an empty report.
    #[allow(dead_code)]
    pub(crate) fn empty() -> Self {
        Self {
            worker_steps: 0,
            runtime_decisions: 0,
            runtime_intents_observed: 0,
            shutdown_intents_observed: 0,
            queue_bridge_used: false,
            has_output_behavior: false,
        }
    }

    /// Create a report from a worker loop report, copying step count.
    #[allow(dead_code)]
    pub(crate) fn from_worker_report(worker_report: OutputThreadWorkerLoopReport) -> Self {
        Self {
            worker_steps: worker_report.completed_steps,
            runtime_decisions: 0,
            runtime_intents_observed: 0,
            shutdown_intents_observed: 0,
            queue_bridge_used: false,
            has_output_behavior: false,
        }
    }

    /// Record a single adapter result and return the updated report.
    #[allow(dead_code)]
    pub(crate) fn record_adapter_result(
        self,
        result: OutputThreadWorkerRuntimeAdapterResult,
    ) -> Self {
        let runtime_intents_observed = match result.worker_runtime_decision.kind {
            OutputThreadWorkerRuntimeDecisionKind::RuntimeIntentObserved => {
                self.runtime_intents_observed + 1
            }
            _ => self.runtime_intents_observed,
        };

        let shutdown_intents_observed = match result.worker_runtime_decision.kind {
            OutputThreadWorkerRuntimeDecisionKind::RuntimeShutdownObserved => {
                self.shutdown_intents_observed + 1
            }
            _ => self.shutdown_intents_observed,
        };

        Self {
            worker_steps: self.worker_steps,
            runtime_decisions: self.runtime_decisions + 1,
            runtime_intents_observed,
            shutdown_intents_observed,
            queue_bridge_used: self.queue_bridge_used || result.queue_bridge_used,
            has_output_behavior: self.has_output_behavior || result.has_output_behavior,
        }
    }

    /// Always false — no queue bridge in P0-069.
    #[allow(dead_code)]
    pub(crate) fn uses_queue_bridge(self) -> bool {
        self.queue_bridge_used
    }

    /// Always true — no real output behavior in P0-069.
    #[allow(dead_code)]
    pub(crate) fn has_no_output_behavior(self) -> bool {
        !self.has_output_behavior
    }
}
