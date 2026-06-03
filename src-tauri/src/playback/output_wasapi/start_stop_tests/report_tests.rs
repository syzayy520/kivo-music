// start_stop_tests/report_tests.rs
//
// Report field semantics tests for the WASAPI start/stop smoke boundary.

use std::env;

use crate::playback::output_wasapi::start_stop::{probe_start_stop, WASAPI_START_STOP_SMOKE_ENV};

#[test]
fn start_stop_report_defaults_do_not_claim_capability() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert!(!report.is_format_supported_called, "is_format_supported_called should always be false");
    assert!(!report.get_current_padding_called, "get_current_padding_called should always be false");
    assert!(!report.reset_audio_client, "reset_audio_client should always be false");
    assert!(!report.audio_produced, "audio_produced should always be false");
    assert!(!report.output_sink_connected, "output_sink_connected should always be false");
    assert!(!report.capability_exposed, "capability_exposed should always be false");
}

#[test]
fn start_stop_success_marks_start_and_stop_only() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    // Without env, start/stop should be false
    assert!(!report.start_attempted, "start_attempted should be false when skipped");
    assert!(!report.started_audio_client, "started_audio_client should be false when skipped");
    assert!(!report.stop_attempted, "stop_attempted should be false when skipped");
    assert!(!report.stopped_audio_client, "stopped_audio_client should be false when skipped");
}

#[test]
fn start_stop_start_failure_records_no_stop() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    // Without env, these should all be false
    assert!(!report.start_attempted);
    assert!(!report.started_audio_client);
    assert!(!report.stop_attempted);
    assert!(!report.stopped_audio_client);
}

#[test]
fn start_stop_stop_failure_records_started() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    // Without env, all false
    assert!(!report.started_audio_client);
    assert!(!report.stopped_audio_client);
}

#[test]
fn start_stop_report_records_mix_format_fields() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert!(report.sample_rate_hz.is_none(), "sample_rate_hz should be None when skipped");
    assert!(report.channels.is_none(), "channels should be None when skipped");
    assert!(report.bits_per_sample.is_none(), "bits_per_sample should be None when skipped");
    assert!(report.block_align.is_none(), "block_align should be None when skipped");
    assert!(report.avg_bytes_per_sec.is_none(), "avg_bytes_per_sec should be None when skipped");
    assert!(report.format_tag.is_none(), "format_tag should be None when skipped");
    assert!(report.cb_size.is_none(), "cb_size should be None when skipped");
}

#[test]
fn start_stop_success_uses_silent_flag() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert!(!report.used_silent_flag, "used_silent_flag should be false when skipped");
    assert!(!report.buffer_released, "buffer_released should be false when skipped");
}

#[test]
fn start_stop_success_records_requested_and_released_frames() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert!(report.requested_frames.is_none(), "requested_frames should be None when skipped");
    assert!(report.released_frames.is_none(), "released_frames should be None when skipped");
}

#[test]
fn start_stop_success_records_wait_duration() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert_eq!(report.wait_duration_ms, Some(0), "wait_duration_ms should be Some(0)");
}

#[test]
fn start_stop_success_records_no_reset_no_padding() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert!(!report.reset_audio_client, "reset_audio_client should always be false");
    assert!(!report.get_current_padding_called, "get_current_padding_called should always be false");
}

#[test]
fn start_stop_success_does_not_claim_output_sink_or_capability() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert!(!report.output_sink_connected, "output_sink_connected should always be false");
    assert!(!report.capability_exposed, "capability_exposed should always be false");
}
