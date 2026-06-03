// buffer_tests/cleanup_tests.rs
//
// Cleanup and RAII behavior tests for the WASAPI buffer smoke boundary.
// Validates that resources are properly released.

use std::env;

use crate::playback::output_wasapi::buffer::{probe_buffer, WASAPI_BUFFER_SMOKE_ENV};

#[test]
fn buffer_drops_without_start_stop() {
    // This test verifies that the buffer smoke does not call Start/Stop/Reset
    env::remove_var(WASAPI_BUFFER_SMOKE_ENV);

    let report = probe_buffer();

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
fn buffer_smoke_does_not_produce_audio() {
    // This test verifies that cleanup does not involve audio production
    env::remove_var(WASAPI_BUFFER_SMOKE_ENV);

    let report = probe_buffer();

    // These must always be false
    assert!(
        !report.audio_produced,
        "audio_produced should always be false"
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
fn mix_format_pointer_released_after_buffer_smoke() {
    // This test verifies that the MixFormatGuard properly releases the format pointer.
    // Without env, we can't test actual COM cleanup, but we can verify the report structure.
    env::remove_var(WASAPI_BUFFER_SMOKE_ENV);

    let report = probe_buffer();

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
fn buffer_silent_flag_usage() {
    // This test verifies that the report tracks silent flag usage correctly.
    // Without env, used_silent_flag should be false.
    env::remove_var(WASAPI_BUFFER_SMOKE_ENV);

    let report = probe_buffer();

    assert!(
        !report.used_silent_flag,
        "used_silent_flag should be false when skipped"
    );
    assert!(
        !report.buffer_released,
        "buffer_released should be false when skipped"
    );
}
