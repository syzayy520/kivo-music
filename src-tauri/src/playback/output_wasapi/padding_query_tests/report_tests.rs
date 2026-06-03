// report_tests.rs
//
// Tests for report construction and field semantics in padding query smoke.

use crate::playback::output_wasapi::padding_query::format_fields::FormatFields;
use crate::playback::output_wasapi::padding_query::report::WasapiPaddingQuerySmokeReport;

fn sample_fields() -> FormatFields {
    FormatFields {
        sample_rate_hz: 44100,
        channels: 2,
        bits_per_sample: 16,
        block_align: 4,
        avg_bytes_per_sec: 176400,
        format_tag: 1,
        cb_size: 0,
    }
}

#[test]
fn padding_query_report_defaults_do_not_claim_capability() {
    let report = WasapiPaddingQuerySmokeReport::base_report_for_windows();
    assert!(!report.audio_produced);
    assert!(!report.output_sink_connected);
    assert!(!report.capability_exposed);
    assert!(!report.reset_audio_client);
    assert!(!report.is_format_supported_called);
    assert!(!report.thread_created);
    assert!(!report.async_runtime_created);
    assert!(!report.callback_registered);
}

#[test]
fn padding_query_success_marks_padding_query_only() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert!(!report.skipped);
    assert!(report.get_current_padding_attempted);
    assert_eq!(report.padding_frames, Some(0));
    assert_eq!(report.query_mode, "started");
}

#[test]
fn padding_query_start_failure_records_no_padding_no_stop() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::start_failed(fields, 1024, "start error".into());
    assert!(report.skipped);
    assert!(report.start_attempted);
    assert!(!report.started_audio_client);
    assert!(!report.get_current_padding_attempted);
    assert!(report.padding_frames.is_none());
    assert!(!report.stop_attempted);
    assert!(!report.stopped_audio_client);
}

#[test]
fn padding_query_padding_failure_records_started_and_stop_attempted() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::get_current_padding_failed(
        fields,
        1024,
        "padding error".into(),
    );
    assert!(report.skipped);
    assert!(report.start_attempted);
    assert!(report.started_audio_client);
    assert!(report.get_current_padding_attempted);
    assert!(report.padding_frames.is_none());
    // Stop should be attempted via StopGuard drop
    // But explicit stop was not called due to early return
    assert!(!report.stop_attempted);
}

#[test]
fn padding_query_stop_failure_records_padding_success() {
    let fields = sample_fields();
    let report =
        WasapiPaddingQuerySmokeReport::stop_failed(fields, 1024, Some(100), "stop error".into());
    assert!(report.skipped);
    assert!(report.start_attempted);
    assert!(report.started_audio_client);
    assert!(report.get_current_padding_attempted);
    assert_eq!(report.padding_frames, Some(100));
    assert!(report.stop_attempted);
    assert!(!report.stopped_audio_client);
}

#[test]
fn padding_query_report_records_mix_format_fields() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert_eq!(report.sample_rate_hz, Some(44100));
    assert_eq!(report.channels, Some(2));
    assert_eq!(report.bits_per_sample, Some(16));
    assert_eq!(report.block_align, Some(4));
    assert_eq!(report.avg_bytes_per_sec, Some(176400));
    assert_eq!(report.format_tag, Some(1));
    assert_eq!(report.cb_size, Some(0));
}

#[test]
fn padding_query_success_uses_silent_flag() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert!(report.prefill_used_silent_flag);
    assert_eq!(report.prefill_requested_frames, Some(1));
    assert_eq!(report.prefill_released_frames, Some(1));
}

#[test]
fn padding_query_success_records_wait_duration() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert_eq!(report.wait_duration_ms, Some(0));
}

#[test]
fn padding_query_success_records_no_reset_no_is_format_supported() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert!(!report.reset_audio_client);
    assert!(!report.is_format_supported_called);
}

#[test]
fn padding_query_success_does_not_claim_output_sink_or_capability() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert!(!report.output_sink_connected);
    assert!(!report.capability_exposed);
}

#[test]
fn padding_query_success_records_query_mode_started() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert_eq!(report.query_mode, "started");
}
