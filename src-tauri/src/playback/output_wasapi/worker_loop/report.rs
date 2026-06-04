//! Worker loop report pure type.
//!
//! Accumulates step counters for a bounded worker loop run.
//! Does not store command payloads, audio state, or buffer data.

use super::state::OutputThreadWorkerLoopState;
use super::step::{
    OutputThreadWorkerLoopStepDecision, OutputThreadWorkerLoopStepKind,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerLoopReport {
    pub completed_steps: usize,
    pub commands_handled: usize,
    pub empty_polls: usize,
    pub stopped_by_close_transport: bool,
    pub final_state: OutputThreadWorkerLoopState,
}

impl OutputThreadWorkerLoopReport {
    /// Create an empty report with zero counters.
    #[allow(dead_code)]
    pub(crate) fn empty(initial_state: OutputThreadWorkerLoopState) -> Self {
        Self {
            completed_steps: 0,
            commands_handled: 0,
            empty_polls: 0,
            stopped_by_close_transport: false,
            final_state: initial_state,
        }
    }

    /// Record a single step decision and return the updated report.
    #[allow(dead_code)]
    pub(crate) fn record_step(self, decision: OutputThreadWorkerLoopStepDecision) -> Self {
        let commands_handled = if matches!(
            decision.kind,
            OutputThreadWorkerLoopStepKind::RuntimeIntentHandled
                | OutputThreadWorkerLoopStepKind::StopRequested
        ) {
            self.commands_handled + 1
        } else {
            self.commands_handled
        };

        let empty_polls = if decision.kind == OutputThreadWorkerLoopStepKind::NoCommand {
            self.empty_polls + 1
        } else {
            self.empty_polls
        };

        let stopped_by_close_transport = self.stopped_by_close_transport
            || decision.kind == OutputThreadWorkerLoopStepKind::TransportClosed
            || decision.kind == OutputThreadWorkerLoopStepKind::Disconnected;

        Self {
            completed_steps: self.completed_steps + 1,
            commands_handled,
            empty_polls,
            stopped_by_close_transport,
            final_state: decision.next_state,
        }
    }

    /// Returns true if the report ends in a terminal state.
    #[allow(dead_code)]
    pub(crate) fn is_terminal(self) -> bool {
        self.final_state.is_terminal()
    }

    /// Returns true if no real output behavior was modeled.
    /// Always true — this is a skeleton.
    #[allow(dead_code)]
    pub(crate) fn has_no_output_behavior(self) -> bool {
        true
    }
}
