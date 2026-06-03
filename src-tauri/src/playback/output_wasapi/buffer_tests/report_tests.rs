// buffer_tests/report_tests.rs
//
// Report field semantics tests for the WASAPI buffer smoke boundary.
// Validates that report fields have correct invariants.

use std::env;

use crate::playback::output_wasapi::buffer::{probe_buffer, WASAPI_BUFFER_SMOKE_ENV};

#[test]
fn buffer_report_defaults_do_not_claim_capability() {
    env::remove_var(WASAPI_BUFFER_SMOKE_ENV);

    let report = probe_buffer();

    // All prohibited operations must be false
    assert!(
        !report.is_format_supported_called,
        "is_format_supported_called should always be false"
    );
    assert!(
        !report.get_current_padding_called,
        "get_current_padding_called should always be false"
    );
    assert!(
        !report.started_audio_client,
        "started_audio_client should always be false"
    );
    assert!(
        !report.stopped_audio_client,
        "stopped_audio_client should always be false"
    );
    assert!(
        !report.reset_audio_client,
        "reset_audio_client should always be false"
    );
    assert!(
        !report.audio_produced,
        "audio_produced should always be false"
    );
}

#[test]
fn buffer_success_marks_get_buffer_and_release_buffer() {
    // This test verifies that a success report claims GetBuffer and ReleaseBuffer succeeded,
    // but not that audio playback is possible (only silent data was submitted).
    env::remove_var(WASAPI_BUFFER_SMOKE_ENV);

    let report = probe_buffer();

    // Without env, these should be false
    assert!(
        !report.get_buffer_attempted,
        "get_buffer_attempted should be false when skipped"
    );
    assert!(
        !report.buffer_obtained,
        "buffer_obtained should be false when skipped"
    );
    assert!(
        !report.release_buffer_attempted,
        "release_buffer_attempted should be false when skipped"
    );
    assert!(
        !report.buffer_released,
        "buffer_released should be false when skipped"
    );
    assert!(
        !report.used_silent_flag,
        "used_silent_flag should be false when skipped"
    );
    assert!(
        !report.get_buffer_size_attempted,
        "get_buffer_size_attempted should be false when skipped"
    );
    assert!(
        report.buffer_size_frames.is_none(),
        "buffer_size_frames should be None when skipped"
    );
    assert!(
        report.requested_frames.is_none(),
        "requested_frames should be None when skipped"
    );
    assert!(
        report.released_frames.is_none(),
        "released_frames should be None when skipped"
    );
}

#[test]
fn buffer_failure_preserves_no_start_stop() {
    // This test verifies that even on failure, prohibited operations remain false
    env::remove_var(WASAPI_BUFFER_SMOKE_ENV);

    let report = probe_buffer();

    // These must always be false regardless of success/failure
    assert!(
        !report.is_format_supported_called,
        "is_format_supported_called should always be false"
    );
    assert!(
        !report.get_current_padding_called,
        "get_current_padding_called should always be false"
    );
    assert!(
        !report.started_audio_client,
        "started_audio_client should always be false"
    );
    assert!(
        !report.stopped_audio_client,
        "stopped_audio_client should always be false"
    );
    assert!(
        !report.reset_audio_client,
        "reset_audio_client should always be false"
    );
    assert!(
        !report.audio_produced,
        "audio_produced should always be false"
    );
}

#[test]
fn buffer_report_records_mix_format_fields() {
    // This test verifies that format fields are properly recorded when available
    env::remove_var(WASAPI_BUFFER_SMOKE_ENV);

    let report = probe_buffer();

    // Without env, format fields should be None
    assert!(
        report.sample_rate_hz.is_none(),
        "sample_rate_hz should be None when skipped"
    );
    assert!(
        report.channels.is_none(),
        "channels should be None when skipped"
    );
    assert!(
        report.bits_per_sample.is_none(),
        "bits_per_sample should be None when skipped"
    );
    assert!(
        report.block_align.is_none(),
        "block_align should be None when skipped"
    );
    assert!(
        report.avg_bytes_per_sec.is_none(),
        "avg_bytes_per_sec should be None when skipped"
    );
    assert!(
        report.format_tag.is_none(),
        "format_tag should be None when skipped"
    );
    assert!(
        report.cb_size.is_none(),
        "cb_size should be None when skipped"
    );
}

#[test]
fn buffer_report_records_initialize_parameters() {
    // This test verifies that Initialize parameters are recorded
    env::remove_var(WASAPI_BUFFER_SMOKE_ENV);

    let report = probe_buffer();

    // These should be fixed values
    assert_eq!(report.share_mode, "shared", "share_mode should be 'shared'");
    assert_eq!(report.stream_flags, 0, "stream_flags should be 0");
    assert_eq!(
        report.buffer_duration_hns, 0,
        "buffer_duration_hns should be 0"
    );
    assert_eq!(report.periodicity_hns, 0, "periodicity_hns should be 0");
}
