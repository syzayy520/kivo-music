// device_tests/env_tests.rs
//
// Environment variable and skip behavior tests for the WASAPI endpoint smoke boundary.

use std::env;

use crate::playback::output_wasapi::device::{probe_default_endpoint, WASAPI_ENDPOINT_SMOKE_ENV};

#[test]
fn endpoint_smoke_without_env_skips_without_attempting_device() {
    // Ensure env is not set
    env::remove_var(WASAPI_ENDPOINT_SMOKE_ENV);

    let report = probe_default_endpoint();

    assert!(!report.opt_in_enabled, "opt_in_enabled should be false");
    assert!(!report.attempted, "attempted should be false");
    assert!(report.skipped, "skipped should be true");
    assert!(
        !report.endpoint_available,
        "endpoint_available should be false"
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
fn endpoint_smoke_with_non_one_env_skips_without_attempting_device() {
    // Test various non-"1" values
    for value in &["0", "true", "false", "yes", "no", "on", "off", ""] {
        env::set_var(WASAPI_ENDPOINT_SMOKE_ENV, value);

        let report = probe_default_endpoint();

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
    }

    // Clean up
    env::remove_var(WASAPI_ENDPOINT_SMOKE_ENV);
}

#[test]
fn endpoint_smoke_report_uses_endpoint_env_name() {
    env::remove_var(WASAPI_ENDPOINT_SMOKE_ENV);

    let report = probe_default_endpoint();

    assert_eq!(
        report.opt_in_env, WASAPI_ENDPOINT_SMOKE_ENV,
        "report should reference the correct env var name"
    );
    assert_eq!(
        report.opt_in_env, "KIVO_WASAPI_ENDPOINT_SMOKE",
        "env var name should be KIVO_WASAPI_ENDPOINT_SMOKE"
    );
}

#[test]
fn endpoint_smoke_report_platform_field() {
    env::remove_var(WASAPI_ENDPOINT_SMOKE_ENV);

    let report = probe_default_endpoint();

    #[cfg(windows)]
    {
        // On Windows with env missing, platform is still "windows"
        // but the report is skipped before reaching COM
        assert_eq!(report.platform, "windows", "platform should be windows");
    }

    #[cfg(not(windows))]
    {
        assert_eq!(
            report.platform, "non-windows",
            "platform should be non-windows"
        );
    }
}

#[cfg(not(windows))]
#[test]
fn endpoint_smoke_non_windows_stub_is_safe() {
    // This test only runs on non-Windows platforms
    // It verifies the stub returns a safe skipped report
    env::remove_var(WASAPI_ENDPOINT_SMOKE_ENV);

    let report = probe_default_endpoint();

    assert_eq!(report.platform, "non-windows");
    assert!(!report.opt_in_enabled);
    assert!(!report.attempted);
    assert!(report.skipped);
    assert_eq!(report.skipped_reason, Some("unsupported platform"));
    assert!(!report.endpoint_available);
    assert!(report.error_message.is_none());
}
