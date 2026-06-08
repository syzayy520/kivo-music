//! Consumer invoker tests.
//! Tests for invoke_sink_consumer, map_sink_result_to_outcome, map_sink_error_to_dispatch_error.

use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::consumer_invoker::{
    invoke_sink_consumer, map_sink_error_to_dispatch_error, map_sink_result_to_outcome,
};
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::{
    DispatchContext, DispatchError, DispatchOutcome,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::{
    consumer::{consumer_contract::SinkConsumer, consumer_snapshot::ConsumerSnapshot},
    SinkError, SinkRequest, SinkResult,
};

struct FakeConsumer {
    ready: bool,
    response: Result<SinkResult, SinkError>,
}

impl FakeConsumer {
    fn with_result(result: Result<SinkResult, SinkError>) -> Self {
        Self { ready: true, response: result }
    }
    fn not_ready() -> Self {
        Self { ready: false, response: Ok(SinkResult::Noop) }
    }
}

impl SinkConsumer for FakeConsumer {
    fn process_request(&mut self, _request: &SinkRequest) -> Result<SinkResult, SinkError> {
        match &self.response {
            Ok(r) => Ok(r.clone()),
            Err(e) => Err(e.clone()),
        }
    }
    fn snapshot(&self) -> ConsumerSnapshot { ConsumerSnapshot::default() }
    fn is_ready(&self) -> bool { self.ready }
    fn reset(&mut self) {}
}

#[test]
fn invoker_success_maps_to_outcome() {
    let mut c = FakeConsumer::with_result(Ok(SinkResult::Success {
        frames_processed: 512,
        bytes_written: 2048,
    }));
    let ctx = DispatchContext::default();
    let result = invoke_sink_consumer(&mut c, &SinkRequest::Noop, &ctx).unwrap();
    assert_eq!(result, DispatchOutcome::Success {
        frames_processed: 512,
        bytes_written: 2048,
    });
}

#[test]
fn invoker_silence_filled_maps_to_outcome() {
    let mut c = FakeConsumer::with_result(Ok(SinkResult::SilenceFilled { frames_written: 256 }));
    let ctx = DispatchContext::default();
    let result = invoke_sink_consumer(&mut c, &SinkRequest::Noop, &ctx).unwrap();
    assert_eq!(result, DispatchOutcome::SilenceFilled { frames_written: 256 });
}

#[test]
fn invoker_skipped_maps_to_outcome() {
    let mut c = FakeConsumer::with_result(Ok(SinkResult::Skipped));
    let ctx = DispatchContext::default();
    let result = invoke_sink_consumer(&mut c, &SinkRequest::Noop, &ctx).unwrap();
    assert_eq!(result, DispatchOutcome::Skipped);
}

#[test]
fn invoker_noop_maps_to_outcome() {
    let mut c = FakeConsumer::with_result(Ok(SinkResult::Noop));
    let ctx = DispatchContext::default();
    let result = invoke_sink_consumer(&mut c, &SinkRequest::Noop, &ctx).unwrap();
    assert_eq!(result, DispatchOutcome::Noop);
}

#[test]
fn invoker_not_ready_returns_skipped() {
    let mut c = FakeConsumer::not_ready();
    let ctx = DispatchContext::default();
    let result = invoke_sink_consumer(&mut c, &SinkRequest::Noop, &ctx).unwrap();
    assert_eq!(result, DispatchOutcome::Skipped);
}

#[test]
fn invoker_sink_error_buffer_underrun() {
    let mut c = FakeConsumer::with_result(Err(SinkError::BufferUnderrun { frames_missing: 64 }));
    let ctx = DispatchContext::default();
    let result = invoke_sink_consumer(&mut c, &SinkRequest::Noop, &ctx);
    assert_eq!(result.unwrap_err(), DispatchError::BufferUnderrun { frames_missing: 64 });
}

#[test]
fn invoker_sink_error_device_lost() {
    let mut c = FakeConsumer::with_result(Err(SinkError::DeviceLost));
    let ctx = DispatchContext::default();
    let result = invoke_sink_consumer(&mut c, &SinkRequest::Noop, &ctx);
    assert_eq!(result.unwrap_err(), DispatchError::DeviceLost);
}

#[test]
fn invoker_sink_error_invalid_request() {
    let mut c = FakeConsumer::with_result(Err(SinkError::InvalidRequest { reason: "bad".into() }));
    let ctx = DispatchContext::default();
    let result = invoke_sink_consumer(&mut c, &SinkRequest::Noop, &ctx);
    match result.unwrap_err() {
        DispatchError::InvalidRequest { reason } => assert_eq!(reason, "bad"),
        _ => panic!("expected InvalidRequest"),
    }
}

#[test]
fn invoker_sink_error_internal() {
    let mut c = FakeConsumer::with_result(Err(SinkError::Internal { description: "oops".into() }));
    let ctx = DispatchContext::default();
    let result = invoke_sink_consumer(&mut c, &SinkRequest::Noop, &ctx);
    match result.unwrap_err() {
        DispatchError::Internal { description } => assert_eq!(description, "oops"),
        _ => panic!("expected Internal"),
    }
}

#[test]
fn map_result_success() {
    let result = map_sink_result_to_outcome(&SinkResult::Success {
        frames_processed: 10,
        bytes_written: 40,
    });
    assert_eq!(result, DispatchOutcome::Success {
        frames_processed: 10,
        bytes_written: 40,
    });
}

#[test]
fn map_result_silence_filled() {
    let result = map_sink_result_to_outcome(&SinkResult::SilenceFilled { frames_written: 5 });
    assert_eq!(result, DispatchOutcome::SilenceFilled { frames_written: 5 });
}

#[test]
fn map_result_skipped() {
    assert_eq!(map_sink_result_to_outcome(&SinkResult::Skipped), DispatchOutcome::Skipped);
}

#[test]
fn map_result_noop() {
    assert_eq!(map_sink_result_to_outcome(&SinkResult::Noop), DispatchOutcome::Noop);
}

#[test]
fn map_error_buffer_underrun() {
    let err = map_sink_error_to_dispatch_error(SinkError::BufferUnderrun { frames_missing: 32 });
    assert_eq!(err, DispatchError::BufferUnderrun { frames_missing: 32 });
}

#[test]
fn map_error_device_lost() {
    assert_eq!(map_sink_error_to_dispatch_error(SinkError::DeviceLost), DispatchError::DeviceLost);
}

#[test]
fn map_error_invalid_request() {
    let err = map_sink_error_to_dispatch_error(SinkError::InvalidRequest { reason: "x".into() });
    match err {
        DispatchError::InvalidRequest { reason } => assert_eq!(reason, "x"),
        _ => panic!("expected InvalidRequest"),
    }
}

#[test]
fn map_error_internal() {
    let err = map_sink_error_to_dispatch_error(SinkError::Internal { description: "y".into() });
    match err {
        DispatchError::Internal { description } => assert_eq!(description, "y"),
        _ => panic!("expected Internal"),
    }
}
