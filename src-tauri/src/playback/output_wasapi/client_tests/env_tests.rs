// client_tests/env_tests.rs
//
// Environment variable and skip behavior tests for the WASAPI client activate smoke boundary.

use std::env;

use crate::playback::output_wasapi::client::{
    probe_client_activate, WASAPI_CLIENT_ACTIVATE_SMOKE_ENV,
};

#[test]
fn client_activate_smoke_without_env_skips_without_attempting() {
    // Ensure env is not set
    env::remove_var(WASAPI_CLIENT_ACTIVATE_SMOKE_ENV);

    let report = probe_client_activate();

    assert!(!report.opt_in_enabled, "opt_in_enabled should be false");
    assert!(!report.attempted, "attempted should be false");
    assert!(report.skipped, "skipped should be true");
    assert!(
        !report.endpoint_available,
        "endpoint_available should be false"
    );
    assert!(!report.client_activated, "client_activated should be false");
    assert!(
        !report.initialized_audio_client,
        "initialized_audio_client should be false"
    );
    assert!(
        !report.render_client_available,
        "render_client_available should be false"
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
fn client_activate_smoke_with_non_one_env_skips_without_attempting() {
    // Test various non-"1" values
    for value in &["0", "true", "false", "yes", "no", "on", "off", ""] {
        env::set_var(WASAPI_CLIENT_ACTIVATE_SMOKE_ENV, value);

        let report = probe_client_activate();

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
            !report.client_activated,
            "client_activated should be false for env value '{value}'"
        );
    }

    // Clean up
    env::remove_var(WASAPI_CLIENT_ACTIVATE_SMOKE_ENV);
}

#[test]
fn client_activate_smoke_report_uses_client_activate_env_name() {
    env::remove_var(WASAPI_CLIENT_ACTIVATE_SMOKE_ENV);

    let report = probe_client_activate();

    assert_eq!(
        report.opt_in_env, WASAPI_CLIENT_ACTIVATE_SMOKE_ENV,
        "report should reference the correct env var name"
    );
    assert_eq!(
        report.opt_in_env, "KIVO_WASAPI_CLIENT_ACTIVATE_SMOKE",
        "env var name should be KIVO_WASAPI_CLIENT_ACTIVATE_SMOKE"
    );
}

#[cfg(not(windows))]
#[test]
fn client_activate_smoke_non_windows_stub_is_safe() {
    // This test only runs on non-Windows platforms
    // It verifies the stub returns a safe skipped report
    env::remove_var(WASAPI_CLIENT_ACTIVATE_SMOKE_ENV);

    let report = probe_client_activate();

    assert_eq!(report.platform, "non-windows");
    assert!(!report.opt_in_enabled);
    assert!(!report.attempted);
    assert!(report.skipped);
    assert_eq!(report.skipped_reason, Some("unsupported platform"));
    assert!(!report.endpoint_available);
    assert!(!report.client_activated);
    assert!(!report.initialized_audio_client);
    assert!(!report.render_client_available);
    assert!(report.error_message.is_none());
}
