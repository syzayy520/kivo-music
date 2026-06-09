//! Render source mapping tests.
//!
//! Tests for RenderSourceResult -> SinkRequest and SinkRequest -> RenderSourceRequest mappings.

use crate::playback::output_wasapi::output_thread::runtime::render_source_mapping::{
    map_sink_request_to_source_request, map_source_result_to_sink_request,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSourceRequest, RenderSourceResult,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::SinkRequest;

// ===== RenderSourceResult -> SinkRequest tests =====

#[test]
fn source_packet_maps_to_render() {
    let result = RenderSourceResult::Packet {
        frames_provided: 1024,
        bytes_read: 4096,
    };
    let mapped = map_source_result_to_sink_request(&result, 44100, 2);
    assert_eq!(
        mapped,
        SinkRequest::Render {
            frame_count: 1024,
            sample_rate: 44100,
            channel_count: 2,
        }
    );
}

#[test]
fn source_exhausted_maps_to_flush() {
    let result = RenderSourceResult::Exhausted;
    let mapped = map_source_result_to_sink_request(&result, 0, 0);
    assert_eq!(mapped, SinkRequest::Flush);
}

#[test]
fn source_skipped_maps_to_noop() {
    let result = RenderSourceResult::Skipped;
    let mapped = map_source_result_to_sink_request(&result, 0, 0);
    assert_eq!(mapped, SinkRequest::Noop);
}

#[test]
fn source_noop_maps_to_noop() {
    let result = RenderSourceResult::Noop;
    let mapped = map_source_result_to_sink_request(&result, 0, 0);
    assert_eq!(mapped, SinkRequest::Noop);
}

#[test]
fn source_packet_uses_provided_parameters() {
    let result = RenderSourceResult::Packet {
        frames_provided: 512,
        bytes_read: 2048,
    };
    let mapped = map_source_result_to_sink_request(&result, 48000, 1);
    match mapped {
        SinkRequest::Render {
            frame_count,
            sample_rate,
            channel_count,
        } => {
            assert_eq!(frame_count, 512);
            assert_eq!(sample_rate, 48000);
            assert_eq!(channel_count, 1);
        }
        _ => panic!("expected Render"),
    }
}

#[test]
fn source_packet_with_zero_frames() {
    let result = RenderSourceResult::Packet {
        frames_provided: 0,
        bytes_read: 0,
    };
    let mapped = map_source_result_to_sink_request(&result, 44100, 2);
    assert_eq!(
        mapped,
        SinkRequest::Render {
            frame_count: 0,
            sample_rate: 44100,
            channel_count: 2,
        }
    );
}

// ===== SinkRequest -> RenderSourceRequest tests =====

#[test]
fn sink_render_maps_to_read_packet() {
    let request = SinkRequest::Render {
        frame_count: 1024,
        sample_rate: 44100,
        channel_count: 2,
    };
    let mapped = map_sink_request_to_source_request(&request);
    assert_eq!(
        mapped,
        RenderSourceRequest::ReadPacket {
            frame_count: 1024,
            sample_rate: 44100,
            channel_count: 2,
        }
    );
}

#[test]
fn sink_write_silence_maps_to_noop() {
    let request = SinkRequest::WriteSilence { frame_count: 256 };
    let mapped = map_sink_request_to_source_request(&request);
    assert_eq!(mapped, RenderSourceRequest::Noop);
}

#[test]
fn sink_flush_maps_to_flush() {
    let request = SinkRequest::Flush;
    let mapped = map_sink_request_to_source_request(&request);
    assert_eq!(mapped, RenderSourceRequest::Flush);
}

#[test]
fn sink_noop_maps_to_noop() {
    let request = SinkRequest::Noop;
    let mapped = map_sink_request_to_source_request(&request);
    assert_eq!(mapped, RenderSourceRequest::Noop);
}

#[test]
fn sink_render_preserves_parameters() {
    let request = SinkRequest::Render {
        frame_count: 512,
        sample_rate: 48000,
        channel_count: 1,
    };
    let mapped = map_sink_request_to_source_request(&request);
    match mapped {
        RenderSourceRequest::ReadPacket {
            frame_count,
            sample_rate,
            channel_count,
        } => {
            assert_eq!(frame_count, 512);
            assert_eq!(sample_rate, 48000);
            assert_eq!(channel_count, 1);
        }
        _ => panic!("expected ReadPacket"),
    }
}

#[test]
fn sink_write_silence_with_zero_frames_maps_to_noop() {
    let request = SinkRequest::WriteSilence { frame_count: 0 };
    let mapped = map_sink_request_to_source_request(&request);
    assert_eq!(mapped, RenderSourceRequest::Noop);
}
