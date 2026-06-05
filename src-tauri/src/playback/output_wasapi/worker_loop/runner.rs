//! Worker loop runner — bounded non-blocking skeleton.
//!
//! Iterates a fixed number of steps, each calling try_recv_command
//! on the transport channel. Uses only non-blocking recv.
//!
//! Does not spawn threads, create workers, or reference audio APIs.

use super::super::real_transport::channel::OutputThreadRealTransportChannel;
use super::report::OutputThreadWorkerLoopReport;
use super::state::OutputThreadWorkerLoopState;
use super::step::plan_worker_loop_step;

/// Configuration for a bounded worker loop run.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerLoopRunConfig {
    /// Maximum number of poll iterations.
    pub max_steps: usize,
    /// Initial loop state before first poll.
    pub initial_state: OutputThreadWorkerLoopState,
}

/// Result of a bounded worker loop run.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerLoopRunResult {
    /// Accumulated step report.
    pub report: OutputThreadWorkerLoopReport,
    /// True if the loop stopped before exhausting max_steps.
    pub stopped_early: bool,
}

/// Run a bounded non-blocking worker loop skeleton.
///
/// Borrows the channel without consuming or closing it.
/// Each iteration calls try_recv_command (never blocking recv).
/// Returns a pure report of what happened.
#[allow(dead_code)]
pub(crate) fn run_worker_loop_skeleton(
    channel: &OutputThreadRealTransportChannel,
    config: OutputThreadWorkerLoopRunConfig,
) -> OutputThreadWorkerLoopRunResult {
    let mut report = OutputThreadWorkerLoopReport::empty(config.initial_state);
    let mut stopped_early = false;

    for _ in 0..config.max_steps {
        let recv = channel.try_recv_command();
        let decision = plan_worker_loop_step(recv);
        report = report.record_step(decision);

        if !decision.should_continue {
            stopped_early = true;
            break;
        }
    }

    OutputThreadWorkerLoopRunResult {
        report,
        stopped_early,
    }
}
