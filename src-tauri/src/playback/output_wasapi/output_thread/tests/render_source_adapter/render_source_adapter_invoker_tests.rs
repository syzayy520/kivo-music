//! Render source adapter invoker tests.
//!
//! Tests for the source_invoker module.

use crate::playback::output_wasapi::output_thread::runtime::render_source_adapter::{
    source_invoker, AdapterContext, AdapterError, AdapterOutcome,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSourceError, RenderSourceRequest, RenderSourceResult,
};
use crate::playback::output_wasapi::output_thread::tests::render_source_adapter::render_source_adapter_helpers::MockRenderSource;

#[test]
fn invoke_render_source_skipped_when_not_ready() {
    let mut source = MockRenderSource::new().with_ready(false);
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 512,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result =
        source_invoker::invoke_render_source(&mut source, &request, &AdapterContext::default())
            .unwrap();
    assert_eq!(result, AdapterOutcome::Skipped);
}

#[test]
fn invoke_render_source_packet_success() {
    let mut source = MockRenderSource::new().with_result(Ok(RenderSourceResult::Packet {
        frames_provided: 512,
        bytes_read: 2048,
    }));
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 512,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result =
        source_invoker::invoke_render_source(&mut source, &request, &AdapterContext::default())
            .unwrap();
    assert_eq!(
        result,
        AdapterOutcome::Packet {
            frames_provided: 512,
            bytes_read: 2048,
        }
    );
}

#[test]
fn invoke_render_source_exhausted() {
    let mut source = MockRenderSource::new().with_result(Ok(RenderSourceResult::Exhausted));
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 512,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result =
        source_invoker::invoke_render_source(&mut source, &request, &AdapterContext::default())
            .unwrap();
    assert_eq!(result, AdapterOutcome::Exhausted);
}

#[test]
fn invoke_render_source_error() {
    let mut source = MockRenderSource::new().with_result(Err(RenderSourceError::SourceExhausted));
    let request = RenderSourceRequest::ReadPacket {
        frame_count: 512,
        sample_rate: 44100,
        channel_count: 2,
    };
    let result =
        source_invoker::invoke_render_source(&mut source, &request, &AdapterContext::default());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), AdapterError::SourceExhausted);
}

#[test]
fn map_source_result_to_outcome_packet() {
    let result = RenderSourceResult::Packet {
        frames_provided: 256,
        bytes_read: 1024,
    };
    let outcome = source_invoker::map_source_result_to_outcome(&result);
    assert_eq!(
        outcome,
        AdapterOutcome::Packet {
            frames_provided: 256,
            bytes_read: 1024,
        }
    );
}

#[test]
fn map_source_result_to_outcome_exhausted() {
    let outcome = source_invoker::map_source_result_to_outcome(&RenderSourceResult::Exhausted);
    assert_eq!(outcome, AdapterOutcome::Exhausted);
}

#[test]
fn map_source_result_to_outcome_skipped() {
    let outcome = source_invoker::map_source_result_to_outcome(&RenderSourceResult::Skipped);
    assert_eq!(outcome, AdapterOutcome::Skipped);
}

#[test]
fn map_source_result_to_outcome_noop() {
    let outcome = source_invoker::map_source_result_to_outcome(&RenderSourceResult::Noop);
    assert_eq!(outcome, AdapterOutcome::Noop);
}

#[test]
fn map_source_error_to_adapter_error_source_exhausted() {
    let err = source_invoker::map_source_error_to_adapter_error(RenderSourceError::SourceExhausted);
    assert_eq!(err, AdapterError::SourceExhausted);
}

#[test]
fn map_source_error_to_adapter_error_format_mismatch() {
    let err =
        source_invoker::map_source_error_to_adapter_error(RenderSourceError::FormatMismatch {
            expected: "f32le".to_string(),
            actual: "s16le".to_string(),
        });
    assert_eq!(
        err,
        AdapterError::FormatMismatch {
            expected: "f32le".to_string(),
            actual: "s16le".to_string(),
        }
    );
}

#[test]
fn map_source_error_to_adapter_error_source_closed() {
    let err = source_invoker::map_source_error_to_adapter_error(RenderSourceError::SourceClosed);
    assert_eq!(err, AdapterError::SourceClosed);
}

#[test]
fn map_source_error_to_adapter_error_internal() {
    let err = source_invoker::map_source_error_to_adapter_error(RenderSourceError::Internal {
        description: "test error".to_string(),
    });
    assert_eq!(
        err,
        AdapterError::Internal {
            description: "test error".to_string(),
        }
    );
}
