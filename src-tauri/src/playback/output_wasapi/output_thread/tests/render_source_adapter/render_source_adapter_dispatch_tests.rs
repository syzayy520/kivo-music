//! Render source adapter dispatch tests.
//!
//! Tests for the source_to_sink_dispatch module.

use crate::playback::output_wasapi::output_thread::runtime::render_source_adapter::{
    source_to_sink_dispatch, AdapterOutcome,
};
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::DispatchOutcome;
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSourceError, RenderSourceRequest, RenderSourceResult,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::SinkResult;
use crate::playback::output_wasapi::output_thread::tests::render_source_adapter::render_source_adapter_helpers::{
    MockRenderSource, MockSinkConsumer,
};

#[test]
fn dispatch_source_to_sink_packet_success() {
    let mut source = MockRenderSource::new().with_result(Ok(RenderSourceResult::Packet {
        frames_provided: 512,
        bytes_read: 2048,
    }));
    let mut consumer = MockSinkConsumer::new().with_result(Ok(SinkResult::Success {
        frames_processed: 512,
        bytes_written: 2048,
    }));
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 512,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result = source_to_sink_dispatch::dispatch_source_to_sink(
        &mut source,
        &mut consumer,
        &request,
        44100,
        2,
    )
    .unwrap();
    assert_eq!(
        result.source_outcome,
        AdapterOutcome::Packet {
            frames_provided: 512,
            bytes_read: 2048,
        }
    );
    assert_eq!(
        result.sink_outcome,
        DispatchOutcome::Success {
            frames_processed: 512,
            bytes_written: 2048,
        }
    );
}

#[test]
fn dispatch_source_to_sink_exhausted_triggers_flush() {
    let mut source = MockRenderSource::new().with_result(Ok(RenderSourceResult::Exhausted));
    let mut consumer =
        MockSinkConsumer::new().with_result(Ok(SinkResult::SilenceFilled { frames_written: 0 }));
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 512,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result = source_to_sink_dispatch::dispatch_source_to_sink(
        &mut source,
        &mut consumer,
        &request,
        44100,
        2,
    )
    .unwrap();
    assert_eq!(result.source_outcome, AdapterOutcome::Exhausted);
    assert_eq!(
        result.sink_outcome,
        DispatchOutcome::SilenceFilled { frames_written: 0 }
    );
}

#[test]
fn dispatch_source_to_sink_source_not_ready() {
    let mut source = MockRenderSource::new().with_ready(false);
    let mut consumer = MockSinkConsumer::new().with_result(Ok(SinkResult::Noop));
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 512,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result = source_to_sink_dispatch::dispatch_source_to_sink(
        &mut source,
        &mut consumer,
        &request,
        44100,
        2,
    )
    .unwrap();
    assert_eq!(result.source_outcome, AdapterOutcome::Skipped);
}

#[test]
fn dispatch_source_to_sink_source_error() {
    let mut source = MockRenderSource::new().with_result(Err(RenderSourceError::SourceExhausted));
    let mut consumer = MockSinkConsumer::new().with_result(Ok(SinkResult::Noop));
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 512,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result = source_to_sink_dispatch::dispatch_source_to_sink(
        &mut source,
        &mut consumer,
        &request,
        44100,
        2,
    );
    assert!(result.is_err());
}

#[test]
fn dispatch_source_to_sink_sink_not_ready() {
    let mut source = MockRenderSource::new().with_result(Ok(RenderSourceResult::Packet {
        frames_provided: 512,
        bytes_read: 2048,
    }));
    let mut consumer = MockSinkConsumer::new().with_result(Ok(SinkResult::Noop));
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 512,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result = source_to_sink_dispatch::dispatch_source_to_sink(
        &mut source,
        &mut consumer,
        &request,
        44100,
        2,
    )
    .unwrap();
    assert_eq!(
        result.source_outcome,
        AdapterOutcome::Packet {
            frames_provided: 512,
            bytes_read: 2048,
        }
    );
}

#[test]
fn source_to_sink_outcome_debug_format() {
    let outcome = source_to_sink_dispatch::SourceToSinkOutcome {
        source_outcome: AdapterOutcome::Packet {
            frames_provided: 256,
            bytes_read: 1024,
        },
        sink_outcome: DispatchOutcome::Success {
            frames_processed: 256,
            bytes_written: 1024,
        },
    };
    let debug_str = format!("{:?}", outcome);
    assert!(debug_str.contains("SourceToSinkOutcome"));
}
