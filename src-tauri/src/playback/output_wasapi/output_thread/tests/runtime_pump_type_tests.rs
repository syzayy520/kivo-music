//! Runtime pump type tests.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::pump::pump_context::PumpConfig;
use crate::playback::output_wasapi::output_thread::runtime::pump::pump_outcome::PumpOutcome as PO;
use crate::playback::output_wasapi::output_thread::runtime::pump::{PumpContext, PumpError};
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::dispatch_outcome::DispatchOutcome;
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::{
    loop_result::LoopResult, loop_state::LoopState,
};
use crate::playback::output_wasapi::output_thread::state::thread_lifecycle::OutputThreadLifecycle;

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

#[test]
fn pump_outcome_state_accessor_all_variants() {
    let s1 = LoopState::new();
    assert_eq!(
        PO::Continue {
            state: s1.clone(),
            driver_result: DriverResult::Continue,
            dispatch_outcome: None,
            events: vec![]
        }
        .state(),
        &s1
    );
    let s2 = LoopState::with_lifecycle(OutputThreadLifecycle::Stopped);
    assert_eq!(
        PO::Stopped {
            state: s2.clone(),
            driver_result: DriverResult::Stop,
            events: vec![]
        }
        .state(),
        &s2
    );
    let s3 = LoopState::new();
    assert_eq!(
        PO::Error {
            state: s3.clone(),
            message: "test".into(),
            events: vec![]
        }
        .state(),
        &s3
    );
    let s4 = LoopState::new();
    assert_eq!(
        PO::Idle {
            state: s4.clone(),
            driver_result: DriverResult::Idle,
            events: vec![]
        }
        .state(),
        &s4
    );
}

#[test]
fn pump_outcome_from_loop_result_all_variants() {
    let s1 = LoopState::new();
    assert!(matches!(
        PO::from_loop_result(
            LoopResult::Continue,
            s1,
            DriverResult::Continue,
            None,
            vec![]
        ),
        PO::Continue { .. }
    ));
    let s2 = LoopState::with_lifecycle(OutputThreadLifecycle::Stopped);
    assert!(matches!(
        PO::from_loop_result(LoopResult::Stop, s2, DriverResult::Stop, None, vec![]),
        PO::Stopped { .. }
    ));
    let s3 = LoopState::new();
    assert!(
        matches!(PO::from_loop_result(LoopResult::Error("fail".into()), s3, DriverResult::Error, None, vec![]), PO::Error { ref message, .. } if message == "fail")
    );
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
        events: vec![],
    };
    assert!(matches!(
        outcome,
        PO::Continue {
            dispatch_outcome: Some(_),
            ..
        }
    ));
}

#[test]
fn pump_outcome_events_accessor_all_variants() {
    let s1 = LoopState::new();
    assert!(PO::Continue {
        state: s1,
        driver_result: DriverResult::Continue,
        dispatch_outcome: None,
        events: vec![]
    }
    .events()
    .is_empty());
    let s2 = LoopState::with_lifecycle(OutputThreadLifecycle::Stopped);
    assert!(PO::Stopped {
        state: s2,
        driver_result: DriverResult::Stop,
        events: vec![]
    }
    .events()
    .is_empty());
}

#[test]
fn pump_error_display_all_variants() {
    assert_eq!(
        format!("{}", PumpError::TerminalState),
        "pump tick on terminal state"
    );
    assert!(format!(
        "{}",
        PumpError::DispatchFailed {
            description: "buffer underrun".into()
        }
    )
    .contains("buffer underrun"));
    assert!(format!(
        "{}",
        PumpError::Internal {
            description: "unexpected".into()
        }
    )
    .contains("unexpected"));
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
