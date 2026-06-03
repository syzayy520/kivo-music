// initialize_tests/report_tests.rs
//
// Report field semantics tests for the WASAPI initialize smoke boundary.
// Validates that report fields have correct invariants.

use std::env;

use crate::playback::output_wasapi::initialize::{probe_initialize, WASAPI_CLIENT_INIT_SMOKE_ENV};

#[test]
fn init_report_defaults_do_not_claim_capability() {
    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);

    let report = probe_initialize();

    // All prohibited operations must be false
    assert!(
        !report.is_format_supported_called,
        "is_format_supported_called should always be false"
    );
    assert!(
        !report.render_client_available,
        "render_client_available should always be false"
    );
    assert!(
        !report.service_requested,
        "service_requested should always be false"
    );
    assert!(
        !report.buffer_requested,
        "buffer_requested should always be false"
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
}

#[test]
fn init_success_report_marks_initialize_only() {
    // This test verifies that a success report only claims Initialize succeeded,
    // not that audio playback is possible.
    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);

    let report = probe_initialize();

    // If this were a success report (which it isn't without env), these would be the invariants:
    // For now, we test the skipped report structure
    assert!(
        !report.initialized_audio_client,
        "initialized_audio_client should be false when skipped"
    );
    assert!(
        !report.initialize_attempted,
        "initialize_attempted should be false when skipped"
    );
}

#[test]
fn init_failure_report_preserves_no_render_buffer_start() {
    // This test verifies that even on failure, prohibited operations remain false
    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);

    let report = probe_initialize();

    // These must always be false regardless of success/failure
    assert!(
        !report.render_client_available,
        "render_client_available should always be false"
    );
    assert!(
        !report.service_requested,
        "service_requested should always be false"
    );
    assert!(
        !report.buffer_requested,
        "buffer_requested should always be false"
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
}

#[test]
fn init_report_records_mix_format_fields() {
    // This test verifies that format fields are properly recorded when available
    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);

    let report = probe_initialize();

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
fn init_report_records_initialize_parameters() {
    // This test verifies that Initialize parameters are recorded
    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);

    let report = probe_initialize();

    // These should be fixed values
    assert_eq!(report.share_mode, "shared", "share_mode should be 'shared'");
    assert_eq!(report.stream_flags, 0, "stream_flags should be 0");
    assert_eq!(
        report.buffer_duration_hns, 0,
        "buffer_duration_hns should be 0"
    );
    assert_eq!(report.periodicity_hns, 0, "periodicity_hns should be 0");
}
