//! Runtime pump step tests.
//!
//! Tests for execute_pump_tick covering idle, command, shutdown,
//! terminal state, and sink dispatch paths.

use crate::playback::output_wasapi::output_thread::command::{
    DrainCommand, ShutdownCommand, ThreadCommand,
};
use crate::playback::output_wasapi::output_thread::runtime::pump::pump_outcome::PumpOutcome;
use crate::playback::output_wasapi::output_thread::runtime::pump::pump_step::execute_pump_tick;
use crate::playback::output_wasapi::output_thread::runtime::pump::PumpContext;
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::dispatch_outcome::DispatchOutcome;
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::loop_state::LoopState;
use crate::playback::output_wasapi::output_thread::sink_boundary::consumer::consumer_contract::SinkConsumer;
use crate::playback::output_wasapi::output_thread::sink_boundary::consumer::consumer_snapshot::ConsumerSnapshot;
use crate::playback::output_wasapi::output_thread::sink_boundary::sink_error::SinkError;
use crate::playback::output_wasapi::output_thread::sink_boundary::sink_request::SinkRequest;
use crate::playback::output_wasapi::output_thread::sink_boundary::sink_result::SinkResult;
use crate::playback::output_wasapi::output_thread::state::thread_lifecycle::OutputThreadLifecycle;

// ── Test double: FakeConsumer ───────────────────────────────────────────

struct FakeConsumer {
    ready: bool,
    response: Result<SinkResult, SinkError>,
}

impl FakeConsumer {
    fn ready_with_success() -> Self {
        Self {
            ready: true,
            response: Ok(SinkResult::Success {
                frames_processed: 1024,
                bytes_written: 4096,
            }),
        }
    }
    fn ready_with_error() -> Self {
        Self {
            ready: true,
            response: Err(SinkError::BufferUnderrun {
                frames_missing: 256,
            }),
        }
    }
    fn not_ready() -> Self {
        Self {
            ready: false,
            response: Ok(SinkResult::Noop),
        }
    }
}

impl SinkConsumer for FakeConsumer {
    fn process_request(&mut self, _: &SinkRequest) -> Result<SinkResult, SinkError> {
        self.response.clone()
    }
    fn snapshot(&self) -> ConsumerSnapshot {
        ConsumerSnapshot::default()
    }
    fn is_ready(&self) -> bool {
        self.ready
    }
    fn reset(&mut self) {}
}

// ── Idle step tests ─────────────────────────────────────────────────────

#[test]
fn pump_tick_idle_returns_continue_and_dispatches() {
    let ctx = PumpContext::with_defaults(LoopState::new(), None);
    let mut consumer = FakeConsumer::ready_with_success();
    let outcome = execute_pump_tick(&ctx, &mut consumer);
    match outcome {
        PumpOutcome::Continue {
            dispatch_outcome, ..
        } => {
            assert!(dispatch_outcome.is_some());
            let dispatch = dispatch_outcome.unwrap();
            assert!(matches!(
                dispatch,
                DispatchOutcome::Success {
                    frames_processed: 1024,
                    bytes_written: 4096
                }
            ));
        }
        _ => panic!("expected Continue"),
    }
}

#[test]
fn pump_tick_idle_not_ready_consumer_skips() {
    let ctx = PumpContext::with_defaults(LoopState::new(), None);
    let mut consumer = FakeConsumer::not_ready();
    let outcome = execute_pump_tick(&ctx, &mut consumer);
    match outcome {
        PumpOutcome::Continue {
            dispatch_outcome, ..
        } => {
            assert_eq!(dispatch_outcome.unwrap(), DispatchOutcome::Skipped);
        }
        _ => panic!("expected Continue"),
    }
}

// ── Command step tests ──────────────────────────────────────────────────

#[test]
fn pump_tick_command_returns_continue() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Running);
    let ctx =
        PumpContext::with_defaults(state, Some(ThreadCommand::Drain(DrainCommand::default())));
    let mut consumer = FakeConsumer::ready_with_success();
    assert!(matches!(
        execute_pump_tick(&ctx, &mut consumer),
        PumpOutcome::Continue { .. }
    ));
}

// ── Shutdown step tests ─────────────────────────────────────────────────

#[test]
fn pump_tick_shutdown_transitions_to_stopping_and_dispatches() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Running);
    let ctx = PumpContext::with_defaults(
        state,
        Some(ThreadCommand::Shutdown(ShutdownCommand::Immediate)),
    );
    let mut consumer = FakeConsumer::ready_with_success();
    match execute_pump_tick(&ctx, &mut consumer) {
        PumpOutcome::Continue {
            dispatch_outcome, ..
        } => {
            assert!(dispatch_outcome.is_some());
        }
        _ => panic!("expected Continue"),
    }
}

// ── Terminal state tests ────────────────────────────────────────────────

#[test]
fn pump_tick_terminal_state_returns_stopped() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Stopped);
    let ctx = PumpContext::with_defaults(state, None);
    let mut consumer = FakeConsumer::ready_with_success();
    assert!(matches!(
        execute_pump_tick(&ctx, &mut consumer),
        PumpOutcome::Stopped { .. }
    ));
}

// ── Sink dispatch tests ─────────────────────────────────────────────────

#[test]
fn pump_tick_dispatch_error_returns_continue_with_none() {
    let ctx = PumpContext::with_defaults(LoopState::new(), None);
    let mut consumer = FakeConsumer::ready_with_error();
    match execute_pump_tick(&ctx, &mut consumer) {
        PumpOutcome::Continue {
            dispatch_outcome, ..
        } => {
            assert!(dispatch_outcome.is_none());
        }
        _ => panic!("expected Continue"),
    }
}

// ── State update tests ──────────────────────────────────────────────────

#[test]
fn pump_tick_idle_increments_idle_count() {
    let ctx = PumpContext::with_defaults(LoopState::new(), None);
    let mut consumer = FakeConsumer::ready_with_success();
    match execute_pump_tick(&ctx, &mut consumer) {
        PumpOutcome::Continue { state, .. } => {
            assert_eq!(state.idle_count, 1);
        }
        _ => panic!("expected Continue"),
    }
}

#[test]
fn pump_tick_command_resets_idle_count() {
    let mut state = LoopState::with_lifecycle(OutputThreadLifecycle::Running);
    state.record_idle();
    state.record_idle();
    assert_eq!(state.idle_count, 2);
    let ctx =
        PumpContext::with_defaults(state, Some(ThreadCommand::Drain(DrainCommand::default())));
    let mut consumer = FakeConsumer::ready_with_success();
    match execute_pump_tick(&ctx, &mut consumer) {
        PumpOutcome::Continue { state, .. } => {
            assert_eq!(state.idle_count, 0);
        }
        _ => panic!("expected Continue"),
    }
}
