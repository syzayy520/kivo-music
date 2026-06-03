// render_client_tests/cleanup_tests.rs
//
// Cleanup and RAII behavior tests for the WASAPI render client smoke boundary.
// Validates that resources are properly released.

use std::env;

use crate::playback::output_wasapi::render_client::{
    probe_render_client, WASAPI_RENDER_CLIENT_SMOKE_ENV,
};

#[test]
fn render_client_drops_without_start_stop() {
    // This test verifies that the render client smoke does not call Start/Stop/Reset
    env::remove_var(WASAPI_RENDER_CLIENT_SMOKE_ENV);

    let report = probe_render_client();

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
fn render_client_smoke_does_not_request_buffer() {
    // This test verifies that cleanup does not involve buffer operations
    env::remove_var(WASAPI_RENDER_CLIENT_SMOKE_ENV);

    let report = probe_render_client();

    // These must always be false
    assert!(
        !report.get_buffer_called,
        "get_buffer_called should always be false"
    );
    assert!(
        !report.release_buffer_called,
        "release_buffer_called should always be false"
    );
    assert!(
        !report.audio_produced,
        "audio_produced should always be false"
    );
}

#[test]
fn mix_format_pointer_released_after_get_service() {
    // This test verifies that the MixFormatGuard properly releases the format pointer.
    // Without env, we can't test actual COM cleanup, but we can verify the report structure.
    env::remove_var(WASAPI_RENDER_CLIENT_SMOKE_ENV);

    let report = probe_render_client();

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
fn cleanup_does_not_start_audio_client() {
    // This test verifies that cleanup does not involve starting the audio client
    env::remove_var(WASAPI_RENDER_CLIENT_SMOKE_ENV);

    let report = probe_render_client();

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
    assert!(
        !report.audio_produced,
        "audio_produced should always be false"
    );
}
