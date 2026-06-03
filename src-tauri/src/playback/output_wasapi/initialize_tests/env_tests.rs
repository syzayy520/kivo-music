// initialize_tests/env_tests.rs
//
// Environment variable and skip behavior tests for the WASAPI initialize smoke boundary.

use std::env;

use crate::playback::output_wasapi::initialize::{probe_initialize, WASAPI_CLIENT_INIT_SMOKE_ENV};

#[test]
fn init_smoke_without_env_skips_without_attempting() {
    // Ensure env is not set
    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);

    let report = probe_initialize();

    assert!(!report.opt_in_enabled, "opt_in_enabled should be false");
    assert!(!report.attempted, "attempted should be false");
    assert!(report.skipped, "skipped should be true");
    assert!(
        !report.endpoint_available,
        "endpoint_available should be false"
    );
    assert!(!report.client_activated, "client_activated should be false");
    assert!(
        !report.mix_format_available,
        "mix_format_available should be false"
    );
    assert!(
        !report.initialize_attempted,
        "initialize_attempted should be false"
    );
    assert!(
        !report.initialized_audio_client,
        "initialized_audio_client should be false"
    );
    assert!(
        report.skipped_reason.is_some(),
        "skipped_reason should be set"
    );
    assert!(
        report.error_message.is_none(),
        "error_message should be None for env missing"
    );
}

#[test]
fn init_smoke_with_non_one_env_skips_without_attempting() {
    // Test various non-"1" values
    for value in &["0", "true", "false", "yes", "no", "on", "off", ""] {
        env::set_var(WASAPI_CLIENT_INIT_SMOKE_ENV, value);

        let report = probe_initialize();

        assert!(
            !report.opt_in_enabled,
            "opt_in_enabled should be false for env value '{value}'"
        );
        assert!(
            !report.attempted,
            "attempted should be false for env value '{value}'"
        );
        assert!(
            report.skipped,
            "skipped should be true for env value '{value}'"
        );
        assert!(
            !report.endpoint_available,
            "endpoint_available should be false for env value '{value}'"
        );
        assert!(
            !report.client_activated,
            "client_activated should be false for env value '{value}'"
        );
        assert!(
            !report.mix_format_available,
            "mix_format_available should be false for env value '{value}'"
        );
        assert!(
            !report.initialize_attempted,
            "initialize_attempted should be false for env value '{value}'"
        );
        assert!(
            !report.initialized_audio_client,
            "initialized_audio_client should be false for env value '{value}'"
        );
    }

    // Clean up
    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);
}

#[test]
fn init_smoke_report_uses_init_env_name() {
    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);

    let report = probe_initialize();

    assert_eq!(
        report.opt_in_env, WASAPI_CLIENT_INIT_SMOKE_ENV,
        "report should reference the correct env var name"
    );
    assert_eq!(
        report.opt_in_env, "KIVO_WASAPI_CLIENT_INIT_SMOKE",
        "env var name should be KIVO_WASAPI_CLIENT_INIT_SMOKE"
    );
}

#[cfg(not(windows))]
#[test]
fn init_smoke_non_windows_stub_is_safe() {
    // This test only runs on non-Windows platforms
    // It verifies the stub returns a safe skipped report
    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);

    let report = probe_initialize();

    assert_eq!(report.platform, "non-windows");
    assert!(!report.opt_in_enabled);
    assert!(!report.attempted);
    assert!(report.skipped);
    assert_eq!(report.skipped_reason, Some("unsupported platform"));
    assert!(!report.endpoint_available);
    assert!(!report.client_activated);
    assert!(!report.mix_format_available);
    assert!(!report.initialize_attempted);
    assert!(!report.initialized_audio_client);
    assert!(report.error_message.is_none());
}
