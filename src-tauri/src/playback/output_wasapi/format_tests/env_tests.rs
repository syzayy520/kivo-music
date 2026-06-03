// env_tests.rs
//
// Tests for opt-in environment variable behavior.
//
// These tests verify that the smoke probe correctly respects the
// KIVO_WASAPI_MIX_FORMAT_SMOKE environment variable.

use std::env;

use crate::playback::output_wasapi::format::{probe_mix_format, WASAPI_MIX_FORMAT_SMOKE_ENV};

#[test]
fn mix_format_smoke_without_env_skips_without_attempting() {
    // Ensure env is not set
    env::remove_var(WASAPI_MIX_FORMAT_SMOKE_ENV);

    let report = probe_mix_format();

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
        !report.initialized_audio_client,
        "initialized_audio_client should be false"
    );
    assert!(
        !report.is_format_supported_called,
        "is_format_supported_called should be false"
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
fn mix_format_smoke_with_non_one_env_skips_without_attempting() {
    // Test various non-"1" values
    for value in &["0", "true", "false", "yes", "no", "on", "off", ""] {
        env::set_var(WASAPI_MIX_FORMAT_SMOKE_ENV, value);

        let report = probe_mix_format();

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
            !report.mix_format_available,
            "mix_format_available should be false for env value '{value}'"
        );
    }

    // Clean up
    env::remove_var(WASAPI_MIX_FORMAT_SMOKE_ENV);
}

#[test]
fn mix_format_smoke_report_uses_mix_format_env_name() {
    env::remove_var(WASAPI_MIX_FORMAT_SMOKE_ENV);

    let report = probe_mix_format();

    assert_eq!(
        report.opt_in_env, WASAPI_MIX_FORMAT_SMOKE_ENV,
        "report should reference the correct env var name"
    );
    assert_eq!(
        report.opt_in_env, "KIVO_WASAPI_MIX_FORMAT_SMOKE",
        "env var name should be KIVO_WASAPI_MIX_FORMAT_SMOKE"
    );
}
