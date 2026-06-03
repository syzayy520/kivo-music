// silent_loop_tests/report_tests.rs
//
// Tests for WasapiSilentLoopSmokeReport fields and builders.

use crate::playback::output_wasapi::silent_loop::format_fields::FormatFields;
use crate::playback::output_wasapi::silent_loop::report::WasapiSilentLoopSmokeReport;
use crate::playback::output_wasapi::silent_loop::report_success_builders::LoopStats;

fn sample_fields() -> FormatFields {
    FormatFields {
        sample_rate_hz: 48000,
        channels: 2,
        bits_per_sample: 16,
        block_align: 4,
        avg_bytes_per_sec: 192000,
        format_tag: 1,
        cb_size: 0,
    }
}

#[test]
fn default_prohibited_fields_all_false_windows() {
    let r = WasapiSilentLoopSmokeReport::base_report_for_windows();
    assert!(!r.is_format_supported_called);
    assert!(!r.reset_audio_client);
    assert!(!r.audio_produced);
    assert!(!r.output_sink_connected);
    assert!(!r.capability_exposed);
    assert!(!r.thread_created);
    assert!(!r.async_runtime_created);
    assert!(!r.callback_registered);
    assert!(!r.ring_buffer_created);
    assert!(!r.decoder_connected);
    assert!(!r.pipeline_connected);
}

#[test]
fn default_prohibited_fields_all_false_non_windows() {
    let r = WasapiSilentLoopSmokeReport::base_report_for_non_windows();
    assert!(!r.ring_buffer_created);
    assert!(!r.decoder_connected);
    assert!(!r.pipeline_connected);
}

#[test]
fn query_mode_is_silent_loop() {
    let r = WasapiSilentLoopSmokeReport::base_report_for_windows();
    assert_eq!(r.query_mode, "silent_loop");
}

#[test]
fn wait_duration_ms_is_zero() {
    let r = WasapiSilentLoopSmokeReport::base_report_for_windows();
    assert_eq!(r.wait_duration_ms, Some(0));
}

#[test]
fn success_report_loop_fields() {
    let fields = sample_fields();
    let stats = LoopStats {
        loop_iterations_completed: 3,
        zero_available_count: 0,
        current_padding_success_count: 3,
        first_padding_frames: 960,
        last_padding_frames: 960,
        min_padding_observed: 960,
        max_padding_observed: 960,
        last_available_frames: 0,
        last_writable_frames: 1,
        loop_get_buffer_success_count: 3,
        loop_release_buffer_success_count: 3,
    };
    let r = WasapiSilentLoopSmokeReport::success(fields, 1024, stats);
    assert_eq!(r.loop_iterations_configured, Some(3));
    assert_eq!(r.loop_iterations_completed, Some(3));
    assert_eq!(r.small_frame_count, Some(1));
    assert_eq!(r.zero_available_count, Some(0));
    assert_eq!(r.first_padding_frames, Some(960));
    assert_eq!(r.last_padding_frames, Some(960));
    assert_eq!(r.min_padding_observed, Some(960));
    assert_eq!(r.max_padding_observed, Some(960));
    assert_eq!(r.last_available_frames, Some(0));
    assert_eq!(r.last_writable_frames, Some(1));
    assert_eq!(r.loop_get_buffer_success_count, Some(3));
    assert_eq!(r.loop_release_buffer_success_count, Some(3));
    assert!(r.loop_all_releases_silent);
}

#[test]
fn success_report_zero_available_count() {
    let fields = sample_fields();
    let stats = LoopStats {
        loop_iterations_completed: 3,
        zero_available_count: 2,
        current_padding_success_count: 3,
        first_padding_frames: 1024,
        last_padding_frames: 1024,
        min_padding_observed: 1024,
        max_padding_observed: 1024,
        last_available_frames: 0,
        last_writable_frames: 0,
        loop_get_buffer_success_count: 1,
        loop_release_buffer_success_count: 1,
    };
    let r = WasapiSilentLoopSmokeReport::success(fields, 1024, stats);
    assert_eq!(r.zero_available_count, Some(2));
    assert_eq!(r.loop_get_buffer_success_count, Some(1));
}

#[test]
fn success_report_format_fields() {
    let fields = sample_fields();
    let stats = LoopStats {
        loop_iterations_completed: 3,
        zero_available_count: 0,
        current_padding_success_count: 3,
        first_padding_frames: 0,
        last_padding_frames: 0,
        min_padding_observed: 0,
        max_padding_observed: 0,
        last_available_frames: 1,
        last_writable_frames: 1,
        loop_get_buffer_success_count: 3,
        loop_release_buffer_success_count: 3,
    };
    let r = WasapiSilentLoopSmokeReport::success(fields, 1024, stats);
    assert_eq!(r.sample_rate_hz, Some(48000));
    assert_eq!(r.channels, Some(2));
    assert_eq!(r.bits_per_sample, Some(16));
    assert_eq!(r.block_align, Some(4));
    assert_eq!(r.avg_bytes_per_sec, Some(192000));
    assert_eq!(r.format_tag, Some(1));
    assert_eq!(r.cb_size, Some(0));
}

#[test]
fn success_report_initialize_params() {
    let r = WasapiSilentLoopSmokeReport::base_report_for_windows();
    assert_eq!(r.share_mode, "shared");
    assert_eq!(r.stream_flags, 0);
    assert_eq!(r.buffer_duration_hns, 0);
    assert_eq!(r.periodicity_hns, 0);
}

#[test]
fn success_report_prohibited_still_false() {
    let fields = sample_fields();
    let stats = LoopStats {
        loop_iterations_completed: 3,
        zero_available_count: 0,
        current_padding_success_count: 3,
        first_padding_frames: 0,
        last_padding_frames: 0,
        min_padding_observed: 0,
        max_padding_observed: 0,
        last_available_frames: 1,
        last_writable_frames: 1,
        loop_get_buffer_success_count: 3,
        loop_release_buffer_success_count: 3,
    };
    let r = WasapiSilentLoopSmokeReport::success(fields, 1024, stats);
    assert!(!r.ring_buffer_created);
    assert!(!r.decoder_connected);
    assert!(!r.pipeline_connected);
    assert!(!r.is_format_supported_called);
    assert!(!r.reset_audio_client);
    assert!(!r.audio_produced);
    assert!(!r.output_sink_connected);
    assert!(!r.capability_exposed);
    assert!(!r.thread_created);
    assert!(!r.async_runtime_created);
    assert!(!r.callback_registered);
}
