//! Driver sink dispatch tests.
//!
//! Tests for the full dispatch chain: dispatch_driver_result.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::driver_sink_dispatch::dispatch_driver_result;
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::{
    DispatchError, DispatchOutcome,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::{
    consumer::{consumer_contract::SinkConsumer, consumer_snapshot::ConsumerSnapshot},
    SinkError, SinkRequest, SinkResult,
};

// ── Fake sink consumer ─────────────────────────────────────────────────

struct FakeConsumer {
    ready: bool,
    captured_request: Option<SinkRequest>,
    response: Result<SinkResult, SinkError>,
}

impl FakeConsumer {
    fn with_result(result: Result<SinkResult, SinkError>) -> Self {
        Self {
            ready: true,
            captured_request: None,
            response: result,
        }
    }

    fn not_ready() -> Self {
        Self {
            ready: false,
            captured_request: None,
            response: Ok(SinkResult::Noop),
        }
    }
}

impl SinkConsumer for FakeConsumer {
    fn process_request(&mut self, request: &SinkRequest) -> Result<SinkResult, SinkError> {
        self.captured_request = Some(request.clone());
        match &self.response {
            Ok(r) => Ok(r.clone()),
            Err(e) => Err(e.clone()),
        }
    }

    fn snapshot(&self) -> ConsumerSnapshot {
        ConsumerSnapshot::default()
    }

    fn is_ready(&self) -> bool {
        self.ready
    }

    fn reset(&mut self) {}
}

// ── dispatch_driver_result tests ───────────────────────────────────────

#[test]
fn chain_continue_maps_to_render() {
    let mut c = FakeConsumer::with_result(Ok(SinkResult::Success {
        frames_processed: 960,
        bytes_written: 3840,
    }));
    let outcome = dispatch_driver_result(&mut c, DriverResult::Continue, 960, 48000, 2).unwrap();
    assert_eq!(
        outcome,
        DispatchOutcome::Success {
            frames_processed: 960,
            bytes_written: 3840,
        }
    );
    // Verify the request was Render with correct params.
    let req = c.captured_request.unwrap();
    assert_eq!(
        req,
        SinkRequest::Render {
            frame_count: 960,
            sample_rate: 48000,
            channel_count: 2,
        }
    );
}

#[test]
fn chain_idle_maps_to_noop() {
    let mut c = FakeConsumer::with_result(Ok(SinkResult::Noop));
    let outcome = dispatch_driver_result(&mut c, DriverResult::Idle, 0, 44100, 2).unwrap();
    assert_eq!(outcome, DispatchOutcome::Noop);
    let req = c.captured_request.unwrap();
    assert_eq!(req, SinkRequest::Noop);
}

#[test]
fn chain_stop_maps_to_flush() {
    let mut c = FakeConsumer::with_result(Ok(SinkResult::Noop));
    let outcome = dispatch_driver_result(&mut c, DriverResult::Stop, 0, 44100, 2).unwrap();
    assert_eq!(outcome, DispatchOutcome::Noop);
    let req = c.captured_request.unwrap();
    assert_eq!(req, SinkRequest::Flush);
}

#[test]
fn chain_error_maps_to_noop_request() {
    let mut c = FakeConsumer::with_result(Ok(SinkResult::Noop));
    let outcome = dispatch_driver_result(&mut c, DriverResult::Error, 0, 44100, 2).unwrap();
    assert_eq!(outcome, DispatchOutcome::Noop);
    let req = c.captured_request.unwrap();
    assert_eq!(req, SinkRequest::Noop);
}

#[test]
fn chain_not_ready_returns_skipped() {
    let mut c = FakeConsumer::not_ready();
    let outcome = dispatch_driver_result(&mut c, DriverResult::Continue, 512, 44100, 2).unwrap();
    assert_eq!(outcome, DispatchOutcome::Skipped);
}

#[test]
fn chain_sink_error_propagates() {
    let mut c = FakeConsumer::with_result(Err(SinkError::DeviceLost));
    let result = dispatch_driver_result(&mut c, DriverResult::Continue, 256, 44100, 2);
    assert_eq!(result.unwrap_err(), DispatchError::DeviceLost);
}

#[test]
fn chain_silence_filled_propagates() {
    let mut c = FakeConsumer::with_result(Ok(SinkResult::SilenceFilled {
        frames_written: 128,
    }));
    let outcome = dispatch_driver_result(&mut c, DriverResult::Continue, 128, 44100, 1).unwrap();
    assert_eq!(
        outcome,
        DispatchOutcome::SilenceFilled {
            frames_written: 128
        }
    );
}
