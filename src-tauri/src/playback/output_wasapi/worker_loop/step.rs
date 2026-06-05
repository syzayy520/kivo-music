//! Worker loop step pure types and decision function.
//!
//! Maps a single transport recv result into a loop step decision.
//! Does not reference thread, sync, audio, or blocking primitives.

use super::super::real_transport::channel::OutputThreadRealTransportRecvResult;
use super::super::real_transport::command::OutputThreadRealTransportCommand;
use super::super::output_thread_runtime_intent::OutputThreadRuntimeIntent;
use super::state::OutputThreadWorkerLoopState;
use super::super::worker_lifecycle::shutdown::OutputThreadWorkerShutdownRequest;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerLoopStepKind {
    /// No command was available.
    NoCommand,
    /// A runtime intent was handled.
    RuntimeIntentHandled,
    /// A stop-type intent was received.
    StopRequested,
    /// A close-transport command was received.
    TransportClosed,
    /// The sender side has been disconnected.
    Disconnected,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadWorkerLoopStepDecision {
    pub kind: OutputThreadWorkerLoopStepKind,
    pub next_state: OutputThreadWorkerLoopState,
    pub shutdown_request: OutputThreadWorkerShutdownRequest,
    pub should_continue: bool,
}

/// Pure function mapping a single recv result into a step decision.
///
/// Does not call channel, lifecycle plan, runtime loop plan,
/// queue bridge plan, or any audio/output API.
#[allow(dead_code)]
pub(crate) fn plan_worker_loop_step(
    recv: OutputThreadRealTransportRecvResult,
) -> OutputThreadWorkerLoopStepDecision {
    match recv {
        OutputThreadRealTransportRecvResult::Empty => OutputThreadWorkerLoopStepDecision {
            kind: OutputThreadWorkerLoopStepKind::NoCommand,
            next_state: OutputThreadWorkerLoopState::NoCommand,
            shutdown_request: OutputThreadWorkerShutdownRequest::None,
            should_continue: true,
        },
        OutputThreadRealTransportRecvResult::Disconnected => OutputThreadWorkerLoopStepDecision {
            kind: OutputThreadWorkerLoopStepKind::Disconnected,
            next_state: OutputThreadWorkerLoopState::TransportClosed,
            shutdown_request: OutputThreadWorkerShutdownRequest::CloseTransport,
            should_continue: false,
        },
        OutputThreadRealTransportRecvResult::Command(cmd) => match cmd {
            OutputThreadRealTransportCommand::CloseTransport => {
                OutputThreadWorkerLoopStepDecision {
                    kind: OutputThreadWorkerLoopStepKind::TransportClosed,
                    next_state: OutputThreadWorkerLoopState::TransportClosed,
                    shutdown_request: OutputThreadWorkerShutdownRequest::CloseTransport,
                    should_continue: false,
                }
            }
            OutputThreadRealTransportCommand::RuntimeIntent(intent) => {
                plan_runtime_intent_step(intent)
            }
        },
    }
}

fn plan_runtime_intent_step(
    intent: OutputThreadRuntimeIntent,
) -> OutputThreadWorkerLoopStepDecision {
    if intent.requests_shutdown() {
        OutputThreadWorkerLoopStepDecision {
            kind: OutputThreadWorkerLoopStepKind::StopRequested,
            next_state: OutputThreadWorkerLoopState::StopRequested,
            shutdown_request: OutputThreadWorkerShutdownRequest::RequestStop,
            should_continue: true,
        }
    } else {
        OutputThreadWorkerLoopStepDecision {
            kind: OutputThreadWorkerLoopStepKind::RuntimeIntentHandled,
            next_state: OutputThreadWorkerLoopState::CommandHandled,
            shutdown_request: OutputThreadWorkerShutdownRequest::None,
            should_continue: true,
        }
    }
}
