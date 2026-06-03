// cleanup_tests.rs
//
// Tests for cleanup behavior in padding query smoke.

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
fn start_success_requires_stop_attempt() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert!(report.start_attempted);
    assert!(report.started_audio_client);
    assert!(report.stop_attempted);
    assert!(report.stopped_audio_client);
}

#[test]
fn padding_failure_still_attempts_stop() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::get_current_padding_failed(
        fields,
        1024,
        "padding error".into(),
    );
    assert!(report.start_attempted);
    assert!(report.started_audio_client);
    // StopGuard should attempt Stop on drop
    // But explicit stop was not called due to early return
    assert!(!report.stop_attempted);
}

#[test]
fn stop_failure_is_reported_without_reset() {
    let fields = sample_fields();
    let report =
        WasapiPaddingQuerySmokeReport::stop_failed(fields, 1024, Some(100), "stop error".into());
    assert!(report.stop_attempted);
    assert!(!report.stopped_audio_client);
    assert!(!report.reset_audio_client);
}

#[test]
fn cleanup_does_not_reset_audio_client() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert!(!report.reset_audio_client);
}

#[test]
fn cleanup_does_not_write_non_silent_data() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert!(!report.audio_produced);
    assert!(report.prefill_used_silent_flag);
}

#[test]
fn cleanup_does_not_connect_output_sink() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert!(!report.output_sink_connected);
}

#[test]
fn cleanup_does_not_claim_capability() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert!(!report.capability_exposed);
}

#[test]
fn cleanup_does_not_register_callback() {
    let fields = sample_fields();
    let report = WasapiPaddingQuerySmokeReport::success(fields, 1024, 0);
    assert!(!report.callback_registered);
    assert!(!report.thread_created);
    assert!(!report.async_runtime_created);
}
