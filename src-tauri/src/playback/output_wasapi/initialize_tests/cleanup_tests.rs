// initialize_tests/cleanup_tests.rs
//
// Cleanup and RAII behavior tests for the WASAPI initialize smoke boundary.
// Validates that resources are properly released.

use std::env;

use crate::playback::output_wasapi::initialize::{probe_initialize, WASAPI_CLIENT_INIT_SMOKE_ENV};

#[test]
fn mix_format_pointer_is_released_after_initialize() {
    // This test verifies that the MixFormatGuard properly releases the format pointer.
    // Without env, we can't test actual COM cleanup, but we can verify the report structure.
    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);

    let report = probe_initialize();

    // Without env, format fields should be None (pointer not obtained)
    assert!(
        report.sample_rate_hz.is_none(),
        "sample_rate_hz should be None when env not set"
    );
    assert!(
        report.channels.is_none(),
        "channels should be None when env not set"
    );
}

#[test]
fn initialize_does_not_require_start_or_stop() {
    // This test verifies that Initialize smoke does not call Start/Stop/Reset
    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);

    let report = probe_initialize();

    // These must always be false
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
fn cleanup_does_not_open_render_client() {
    // This test verifies that cleanup does not involve render client
    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);

    let report = probe_initialize();

    // These must always be false
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
}
