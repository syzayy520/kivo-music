// start_stop_tests/env_tests.rs
//
// Environment variable and skip behavior tests for the WASAPI start/stop smoke boundary.

use std::env;

use crate::playback::output_wasapi::start_stop::{probe_start_stop, WASAPI_START_STOP_SMOKE_ENV};

#[test]
fn start_stop_smoke_without_env_skips_without_attempting() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert!(!report.opt_in_enabled, "opt_in_enabled should be false");
    assert!(!report.attempted, "attempted should be false");
    assert!(report.skipped, "skipped should be true");
    assert!(!report.endpoint_available, "endpoint_available should be false");
    assert!(!report.client_activated, "client_activated should be false");
    assert!(!report.mix_format_available, "mix_format_available should be false");
    assert!(!report.initialize_attempted, "initialize_attempted should be false");
    assert!(!report.initialized_audio_client, "initialized_audio_client should be false");
    assert!(!report.get_service_attempted, "get_service_attempted should be false");
    assert!(!report.render_client_obtained, "render_client_obtained should be false");
    assert!(!report.get_buffer_size_attempted, "get_buffer_size_attempted should be false");
    assert!(!report.get_buffer_attempted, "get_buffer_attempted should be false");
    assert!(!report.buffer_obtained, "buffer_obtained should be false");
    assert!(!report.release_buffer_attempted, "release_buffer_attempted should be false");
    assert!(!report.buffer_released, "buffer_released should be false");
    assert!(!report.start_attempted, "start_attempted should be false");
    assert!(!report.started_audio_client, "started_audio_client should be false");
    assert!(!report.stop_attempted, "stop_attempted should be false");
    assert!(!report.stopped_audio_client, "stopped_audio_client should be false");
    assert!(report.skipped_reason.is_some(), "skipped_reason should be set");
    assert!(report.error_message.is_none(), "error_message should be None for env missing");
}

#[test]
fn start_stop_smoke_with_non_one_env_skips_without_attempting() {
    for value in &["0", "true", "false", "yes", "no", "on", "off", ""] {
        env::set_var(WASAPI_START_STOP_SMOKE_ENV, value);

        let report = probe_start_stop();

        assert!(!report.opt_in_enabled, "opt_in_enabled should be false for '{value}'");
        assert!(!report.attempted, "attempted should be false for '{value}'");
        assert!(report.skipped, "skipped should be true for '{value}'");
        assert!(!report.start_attempted, "start_attempted should be false for '{value}'");
        assert!(!report.stop_attempted, "stop_attempted should be false for '{value}'");
    }

    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);
}

#[test]
fn start_stop_smoke_report_uses_start_stop_env_name() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert_eq!(report.opt_in_env, WASAPI_START_STOP_SMOKE_ENV);
    assert_eq!(report.opt_in_env, "KIVO_WASAPI_START_STOP_SMOKE");
}

#[cfg(not(windows))]
#[test]
fn start_stop_non_windows_stub_is_safe() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert_eq!(report.platform, "non-windows");
    assert!(!report.opt_in_enabled);
    assert!(!report.attempted);
    assert!(report.skipped);
    assert_eq!(report.skipped_reason, Some("unsupported platform"));
    assert!(!report.endpoint_available);
    assert!(!report.start_attempted);
    assert!(!report.stop_attempted);
    assert!(report.error_message.is_none());
}
