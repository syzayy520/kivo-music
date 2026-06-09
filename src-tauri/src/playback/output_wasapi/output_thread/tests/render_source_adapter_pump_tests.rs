//! Render source adapter pump integration tests.
//!
//! Tests for the pump_integration module.

use crate::playback::output_wasapi::output_thread::runtime::pump::pump_context::PumpContext;
use crate::playback::output_wasapi::output_thread::runtime::pump::pump_outcome::PumpOutcome;
use crate::playback::output_wasapi::output_thread::runtime::render_source_adapter::{
    pump_integration, AdapterOutcome,
};
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::DispatchOutcome;
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::loop_state::LoopState;
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSourceError, RenderSourceResult,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::SinkResult;
use crate::playback::output_wasapi::output_thread::tests::render_source_adapter_helpers::{
    MockRenderSource, MockSinkConsumer,
};

fn default_ctx() -> PumpContext {
    PumpContext::with_defaults(LoopState::default(), None)
}

#[test]
fn execute_pump_tick_with_source_success() {
    let mut source = MockRenderSource::new().with_result(Ok(RenderSourceResult::Packet {
        frames_provided: 512,
        bytes_read: 2048,
    }));
    let mut consumer = MockSinkConsumer::new().with_result(Ok(SinkResult::Success {
        frames_processed: 512,
        bytes_written: 2048,
    }));
    let result =
        pump_integration::execute_pump_tick_with_source(&default_ctx(), &mut source, &mut consumer)
            .unwrap();
    assert_eq!(
        result.source_outcome,
        Some(AdapterOutcome::Packet {
            frames_provided: 512,
            bytes_read: 2048,
        })
    );
    assert_eq!(
        result.sink_outcome,
        Some(DispatchOutcome::Success {
            frames_processed: 512,
            bytes_written: 2048,
        })
    );
    match result.pump_outcome {
        PumpOutcome::Continue { .. } => {}
        _ => panic!("Expected Continue variant"),
    }
}

#[test]
fn execute_pump_tick_with_source_exhausted() {
    let mut source = MockRenderSource::new().with_result(Ok(RenderSourceResult::Exhausted));
    let mut consumer =
        MockSinkConsumer::new().with_result(Ok(SinkResult::SilenceFilled { frames_written: 0 }));
    let result =
        pump_integration::execute_pump_tick_with_source(&default_ctx(), &mut source, &mut consumer)
            .unwrap();
    assert_eq!(result.source_outcome, Some(AdapterOutcome::Exhausted));
    assert_eq!(
        result.sink_outcome,
        Some(DispatchOutcome::SilenceFilled { frames_written: 0 })
    );
}

#[test]
fn execute_pump_tick_with_source_not_ready() {
    let mut source = MockRenderSource::new().with_ready(false);
    let mut consumer = MockSinkConsumer::new().with_result(Ok(SinkResult::Noop));
    let result =
        pump_integration::execute_pump_tick_with_source(&default_ctx(), &mut source, &mut consumer)
            .unwrap();
    assert_eq!(result.source_outcome, Some(AdapterOutcome::Skipped));
    assert_eq!(result.sink_outcome, Some(DispatchOutcome::Noop));
}

#[test]
fn execute_pump_tick_with_source_error() {
    let mut source = MockRenderSource::new().with_result(Err(RenderSourceError::SourceExhausted));
    let mut consumer = MockSinkConsumer::new().with_result(Ok(SinkResult::Noop));
    let result =
        pump_integration::execute_pump_tick_with_source(&default_ctx(), &mut source, &mut consumer);
    assert!(result.is_err());
}

#[test]
fn source_pump_outcome_debug_format() {
    let outcome = pump_integration::SourcePumpOutcome {
        pump_outcome: PumpOutcome::Continue {
            state: LoopState::default(),
            driver_result:
                crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult::Continue,
            dispatch_outcome: Some(DispatchOutcome::Success {
                frames_processed: 256,
                bytes_written: 1024,
            }),
            events: Vec::new(),
            consumer_snapshot: None,
        },
        source_outcome: Some(AdapterOutcome::Packet {
            frames_provided: 256,
            bytes_read: 1024,
        }),
        sink_outcome: Some(DispatchOutcome::Success {
            frames_processed: 256,
            bytes_written: 1024,
        }),
    };
    let debug_str = format!("{:?}", outcome);
    assert!(debug_str.contains("SourcePumpOutcome"));
}
