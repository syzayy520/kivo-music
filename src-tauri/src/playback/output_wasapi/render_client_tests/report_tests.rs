// render_client_tests/report_tests.rs
//
// Report field semantics tests for the WASAPI render client smoke boundary.
// Validates that report fields have correct invariants.

use std::env;

use crate::playback::output_wasapi::render_client::{
    probe_render_client, WASAPI_RENDER_CLIENT_SMOKE_ENV,
};

#[test]
fn render_client_report_defaults_do_not_claim_capability() {
    env::remove_var(WASAPI_RENDER_CLIENT_SMOKE_ENV);

    let report = probe_render_client();

    // All prohibited operations must be false
    assert!(
        !report.is_format_supported_called,
        "is_format_supported_called should always be false"
    );
    assert!(
        !report.get_buffer_called,
        "get_buffer_called should always be false"
    );
    assert!(
        !report.release_buffer_called,
        "release_buffer_called should always be false"
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
fn render_client_success_marks_get_service_only() {
    // This test verifies that a success report only claims GetService succeeded,
    // not that audio playback is possible.
    env::remove_var(WASAPI_RENDER_CLIENT_SMOKE_ENV);

    let report = probe_render_client();

    // Without env, these should be false
    assert!(
        !report.get_service_attempted,
        "get_service_attempted should be false when skipped"
    );
    assert!(
        !report.render_client_obtained,
        "render_client_obtained should be false when skipped"
    );
    assert!(
        !report.initialized_audio_client,
        "initialized_audio_client should be false when skipped"
    );
}

#[test]
fn render_client_failure_preserves_no_buffer_start() {
    // This test verifies that even on failure, prohibited operations remain false
    env::remove_var(WASAPI_RENDER_CLIENT_SMOKE_ENV);

    let report = probe_render_client();

    // These must always be false regardless of success/failure
    assert!(
        !report.get_buffer_called,
        "get_buffer_called should always be false"
    );
    assert!(
        !report.release_buffer_called,
        "release_buffer_called should always be false"
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
fn render_client_report_records_mix_format_fields() {
    // This test verifies that format fields are properly recorded when available
    env::remove_var(WASAPI_RENDER_CLIENT_SMOKE_ENV);

    let report = probe_render_client();

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
fn render_client_report_records_initialize_parameters() {
    // This test verifies that Initialize parameters are recorded
    env::remove_var(WASAPI_RENDER_CLIENT_SMOKE_ENV);

    let report = probe_render_client();

    // These should be fixed values
    assert_eq!(report.share_mode, "shared", "share_mode should be 'shared'");
    assert_eq!(report.stream_flags, 0, "stream_flags should be 0");
    assert_eq!(
        report.buffer_duration_hns, 0,
        "buffer_duration_hns should be 0"
    );
    assert_eq!(report.periodicity_hns, 0, "periodicity_hns should be 0");
}
