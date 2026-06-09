//! Runtime pump type tests.
//!
//! Tests for PumpContext, PumpOutcome, PumpError types.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::pump::pump_context::PumpConfig;
use crate::playback::output_wasapi::output_thread::runtime::pump::pump_outcome::PumpOutcome as PO;
use crate::playback::output_wasapi::output_thread::runtime::pump::{
    PumpContext, PumpError, PumpOutcome,
};
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::dispatch_outcome::DispatchOutcome;
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::loop_result::LoopResult;
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::loop_state::LoopState;
use crate::playback::output_wasapi::output_thread::state::thread_lifecycle::OutputThreadLifecycle;

// ── PumpConfig tests ────────────────────────────────────────────────────

#[test]
fn pump_config_default_values() {
    let config = PumpConfig::default();
    assert_eq!(config.frame_count, 1024);
    assert_eq!(config.sample_rate, 44100);
    assert_eq!(config.channel_count, 2);
    assert_eq!(config.max_idle_steps, 100);
}

#[test]
fn pump_config_clone() {
    let config = PumpConfig::default();
    let cloned = config.clone();
    assert_eq!(config, cloned);
}

#[test]
fn pump_config_custom_values() {
    let config = PumpConfig {
        frame_count: 512,
        sample_rate: 48000,
        channel_count: 1,
        max_idle_steps: 50,
    };
    assert_eq!(config.frame_count, 512);
    assert_eq!(config.sample_rate, 48000);
}

// ── PumpContext tests ───────────────────────────────────────────────────

#[test]
fn pump_context_new() {
    let state = LoopState::new();
    let ctx = PumpContext::new(state.clone(), None, PumpConfig::default());
    assert_eq!(ctx.state, state);
    assert!(ctx.command.is_none());
}

#[test]
fn pump_context_with_defaults() {
    let state = LoopState::new();
    let ctx = PumpContext::with_defaults(state.clone(), None);
    assert_eq!(ctx.config, PumpConfig::default());
}

#[test]
fn pump_context_clone() {
    let state = LoopState::new();
    let ctx = PumpContext::with_defaults(state, None);
    let cloned = ctx.clone();
    assert_eq!(ctx.state, cloned.state);
}

// ── PumpOutcome tests ───────────────────────────────────────────────────

#[test]
fn pump_outcome_state_accessor_continue() {
    let state = LoopState::new();
    let outcome = PO::Continue {
        state: state.clone(),
        driver_result: DriverResult::Continue,
        dispatch_outcome: None,
    };
    assert_eq!(outcome.state(), &state);
}

#[test]
fn pump_outcome_state_accessor_stopped() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Stopped);
    let outcome = PO::Stopped {
        state: state.clone(),
        driver_result: DriverResult::Stop,
    };
    assert_eq!(outcome.state(), &state);
}

#[test]
fn pump_outcome_state_accessor_error() {
    let state = LoopState::new();
    let outcome = PO::Error {
        state: state.clone(),
        message: "test".into(),
    };
    assert_eq!(outcome.state(), &state);
}

#[test]
fn pump_outcome_state_accessor_idle() {
    let state = LoopState::new();
    let outcome = PO::Idle {
        state: state.clone(),
        driver_result: DriverResult::Idle,
    };
    assert_eq!(outcome.state(), &state);
}

#[test]
fn pump_outcome_from_loop_result_continue() {
    let state = LoopState::new();
    let outcome = PO::from_loop_result(
        LoopResult::Continue,
        state.clone(),
        DriverResult::Continue,
        None,
    );
    assert!(matches!(outcome, PO::Continue { .. }));
}

#[test]
fn pump_outcome_from_loop_result_stop() {
    let state = LoopState::with_lifecycle(OutputThreadLifecycle::Stopped);
    let outcome = PO::from_loop_result(LoopResult::Stop, state.clone(), DriverResult::Stop, None);
    assert!(matches!(outcome, PO::Stopped { .. }));
}

#[test]
fn pump_outcome_from_loop_result_error() {
    let state = LoopState::new();
    let outcome = PO::from_loop_result(
        LoopResult::Error("fail".into()),
        state.clone(),
        DriverResult::Error,
        None,
    );
    match outcome {
        PO::Error { message, .. } => assert_eq!(message, "fail"),
        _ => panic!("expected Error"),
    }
}

#[test]
fn pump_outcome_continue_with_dispatch() {
    let state = LoopState::new();
    let dispatch = DispatchOutcome::Success {
        frames_processed: 1024,
        bytes_written: 4096,
    };
    let outcome = PO::Continue {
        state,
        driver_result: DriverResult::Continue,
        dispatch_outcome: Some(dispatch),
    };
    match outcome {
        PO::Continue {
            dispatch_outcome, ..
        } => {
            assert!(dispatch_outcome.is_some());
        }
        _ => panic!("expected Continue"),
    }
}

// ── PumpError tests ─────────────────────────────────────────────────────

#[test]
fn pump_error_terminal_state_display() {
    let err = PumpError::TerminalState;
    assert_eq!(format!("{}", err), "pump tick on terminal state");
}

#[test]
fn pump_error_dispatch_failed_display() {
    let err = PumpError::DispatchFailed {
        description: "buffer underrun".into(),
    };
    assert!(format!("{}", err).contains("buffer underrun"));
}

#[test]
fn pump_error_internal_display() {
    let err = PumpError::Internal {
        description: "unexpected".into(),
    };
    assert!(format!("{}", err).contains("unexpected"));
}

#[test]
fn pump_error_clone() {
    let err = PumpError::TerminalState;
    let cloned = err.clone();
    assert_eq!(err, cloned);
}

#[test]
fn pump_error_equality() {
    let a = PumpError::DispatchFailed {
        description: "x".into(),
    };
    let b = PumpError::DispatchFailed {
        description: "x".into(),
    };
    assert_eq!(a, b);
}
