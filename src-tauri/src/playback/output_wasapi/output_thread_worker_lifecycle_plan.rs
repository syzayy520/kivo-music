//! Worker lifecycle planning pure function.
//!
//! Computes the next lifecycle stage and shutdown outcome
//! from a handle contract and shutdown request.
//!
//! Does not reference real transport channels, threads, or sync primitives.

use super::output_thread_worker_handle_contract::OutputThreadWorkerHandleContract;
use super::output_thread_worker_lifecycle::OutputThreadWorkerLifecycleStage;
use super::output_thread_worker_shutdown::{
    OutputThreadWorkerShutdownOutcome, OutputThreadWorkerShutdownRequest,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerLifecycleInput {
    pub handle: OutputThreadWorkerHandleContract,
    pub request: OutputThreadWorkerShutdownRequest,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerLifecycleDecision {
    pub next_stage: OutputThreadWorkerLifecycleStage,
    pub outcome: OutputThreadWorkerShutdownOutcome,
    pub should_continue_scaffold: bool,
}

/// Pure planning function: compute next lifecycle decision from input.
#[allow(dead_code)]
pub(crate) fn plan_worker_lifecycle(
    input: OutputThreadWorkerLifecycleInput,
) -> OutputThreadWorkerLifecycleDecision {
    let current = input.handle.lifecycle;

    match input.request {
        OutputThreadWorkerShutdownRequest::None => OutputThreadWorkerLifecycleDecision {
            next_stage: current,
            outcome: OutputThreadWorkerShutdownOutcome::Unsupported,
            should_continue_scaffold: true,
        },
        OutputThreadWorkerShutdownRequest::RequestStop => {
            if current.is_terminal() {
                OutputThreadWorkerLifecycleDecision {
                    next_stage: current,
                    outcome: OutputThreadWorkerShutdownOutcome::AlreadyStopped,
                    should_continue_scaffold: true,
                }
            } else if input.handle.has_no_worker_loop() {
                OutputThreadWorkerLifecycleDecision {
                    next_stage: current,
                    outcome: OutputThreadWorkerShutdownOutcome::NoWorker,
                    should_continue_scaffold: true,
                }
            } else {
                OutputThreadWorkerLifecycleDecision {
                    next_stage: OutputThreadWorkerLifecycleStage::StopRequested,
                    outcome: OutputThreadWorkerShutdownOutcome::StopMarked,
                    should_continue_scaffold: true,
                }
            }
        }
        OutputThreadWorkerShutdownRequest::CloseTransport => OutputThreadWorkerLifecycleDecision {
            next_stage: OutputThreadWorkerLifecycleStage::Stopped,
            outcome: OutputThreadWorkerShutdownOutcome::TransportClosed,
            should_continue_scaffold: false,
        },
    }
}
