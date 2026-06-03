// reset_boundary_tests/report_tests.rs
//
// Tests for WasapiResetBoundarySmokeReport fields and builders.

use crate::playback::output_wasapi::reset_boundary::format_fields::FormatFields;
use crate::playback::output_wasapi::reset_boundary::report::WasapiResetBoundarySmokeReport;

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
    let r = WasapiResetBoundarySmokeReport::base_report_for_windows();
    assert!(!r.is_format_supported_called);
    assert!(!r.loop_executed);
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
    let r = WasapiResetBoundarySmokeReport::base_report_for_non_windows();
    assert!(!r.is_format_supported_called);
    assert!(!r.loop_executed);
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
fn query_mode_is_reset() {
    let r = WasapiResetBoundarySmokeReport::base_report_for_windows();
    assert_eq!(r.query_mode, "reset");
}

#[test]
fn wait_duration_ms_is_zero() {
    let r = WasapiResetBoundarySmokeReport::base_report_for_windows();
    assert_eq!(r.wait_duration_ms, Some(0));
}

#[test]
fn success_report_reset_fields() {
    let fields = sample_fields();
    let r = WasapiResetBoundarySmokeReport::success(fields, 1024, 960, 0);
    assert!(r.reset_attempted);
    assert!(r.reset_succeeded);
    assert_eq!(r.reset_hresult, Some(0));
    assert_eq!(r.current_padding_frames, Some(960));
    assert_eq!(r.buffer_size_frames, Some(1024));
}

#[test]
fn success_report_stop_succeeded_reset_succeeded() {
    let fields = sample_fields();
    let r = WasapiResetBoundarySmokeReport::success(fields, 1024, 960, 0);
    assert!(r.stop_attempted);
    assert!(r.stopped_audio_client);
    assert!(r.reset_attempted);
    assert!(r.reset_succeeded);
}

#[test]
fn stop_failed_reset_attempted_false() {
    let fields = sample_fields();
    let r = WasapiResetBoundarySmokeReport::stop_failed(fields, 1024, "stop error".to_string());
    assert!(!r.reset_attempted);
    assert!(!r.reset_succeeded);
    assert_eq!(r.reset_hresult, None);
    assert!(r.stop_attempted);
    assert!(!r.stopped_audio_client);
}

#[test]
fn reset_failed_stopped_audio_client_true() {
    // When Reset fails, Stop must have succeeded.
    // We can't directly construct this state via a single builder,
    // but we can verify the report fields are consistent.
    let fields = sample_fields();
    let r = WasapiResetBoundarySmokeReport::reset_failed(
        fields,
        1024,
        960,
        -1,
        "reset error".to_string(),
    );
    assert!(r.reset_attempted);
    assert!(!r.reset_succeeded);
    assert_eq!(r.reset_hresult, Some(-1));
    assert!(r.stopped_audio_client);
    assert!(r.stop_attempted);
}

#[test]
fn success_report_format_fields() {
    let fields = sample_fields();
    let r = WasapiResetBoundarySmokeReport::success(fields, 1024, 960, 0);
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
    let r = WasapiResetBoundarySmokeReport::base_report_for_windows();
    assert_eq!(r.share_mode, "shared");
    assert_eq!(r.stream_flags, 0);
    assert_eq!(r.buffer_duration_hns, 0);
    assert_eq!(r.periodicity_hns, 0);
}

#[test]
fn success_report_prohibited_still_false() {
    let fields = sample_fields();
    let r = WasapiResetBoundarySmokeReport::success(fields, 1024, 960, 0);
    assert!(!r.ring_buffer_created);
    assert!(!r.decoder_connected);
    assert!(!r.pipeline_connected);
    assert!(!r.is_format_supported_called);
    assert!(!r.loop_executed);
    assert!(!r.audio_produced);
    assert!(!r.output_sink_connected);
    assert!(!r.capability_exposed);
    assert!(!r.thread_created);
    assert!(!r.async_runtime_created);
    assert!(!r.callback_registered);
}
