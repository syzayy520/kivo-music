//! Runtime sink mapping tests.
//!
//! Tests for DriverResult -> SinkRequest and SinkResult -> ThreadEvent mappings.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::sink_mapping::{
    map_driver_result_to_sink_request, map_sink_result_to_thread_event,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::{SinkRequest, SinkResult};

// ===== DriverResult -> SinkRequest tests =====

#[test]
fn driver_continue_maps_to_render() {
    let result = map_driver_result_to_sink_request(DriverResult::Continue, 1024, 44100, 2);
    assert_eq!(
        result,
        SinkRequest::Render {
            frame_count: 1024,
            sample_rate: 44100,
            channel_count: 2,
        }
    );
}

#[test]
fn driver_idle_maps_to_noop() {
    let result = map_driver_result_to_sink_request(DriverResult::Idle, 0, 0, 0);
    assert_eq!(result, SinkRequest::Noop);
}

#[test]
fn driver_stop_maps_to_flush() {
    let result = map_driver_result_to_sink_request(DriverResult::Stop, 0, 0, 0);
    assert_eq!(result, SinkRequest::Flush);
}

#[test]
fn driver_error_maps_to_noop() {
    let result = map_driver_result_to_sink_request(DriverResult::Error, 0, 0, 0);
    assert_eq!(result, SinkRequest::Noop);
}

#[test]
fn driver_continue_uses_provided_parameters() {
    let result = map_driver_result_to_sink_request(DriverResult::Continue, 512, 48000, 1);
    match result {
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
fn driver_continue_with_zero_frames() {
    let result = map_driver_result_to_sink_request(DriverResult::Continue, 0, 44100, 2);
    assert_eq!(
        result,
        SinkRequest::Render {
            frame_count: 0,
            sample_rate: 44100,
            channel_count: 2,
        }
    );
}

// ===== SinkResult -> ThreadEvent tests =====

#[test]
fn sink_success_returns_none() {
    let result = SinkResult::Success {
        frames_processed: 1024,
        bytes_written: 4096,
    };
    assert!(map_sink_result_to_thread_event(&result).is_none());
}

#[test]
fn sink_silence_filled_returns_none() {
    let result = SinkResult::SilenceFilled {
        frames_written: 256,
    };
    assert!(map_sink_result_to_thread_event(&result).is_none());
}

#[test]
fn sink_skipped_returns_none() {
    let result = SinkResult::Skipped;
    assert!(map_sink_result_to_thread_event(&result).is_none());
}

#[test]
fn sink_noop_returns_none() {
    let result = SinkResult::Noop;
    assert!(map_sink_result_to_thread_event(&result).is_none());
}

#[test]
fn sink_success_with_zero_values_returns_none() {
    let result = SinkResult::Success {
        frames_processed: 0,
        bytes_written: 0,
    };
    assert!(map_sink_result_to_thread_event(&result).is_none());
}

#[test]
fn sink_silence_filled_with_zero_returns_none() {
    let result = SinkResult::SilenceFilled { frames_written: 0 };
    assert!(map_sink_result_to_thread_event(&result).is_none());
}
