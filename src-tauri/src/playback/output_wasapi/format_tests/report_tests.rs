// report_tests.rs
//
// Tests for report field defaults and constraints.
//
// These tests verify that report fields have the expected values
// in various scenarios, especially that prohibited operations
// are always reported as false.

use std::env;

use crate::playback::output_wasapi::format::{probe_mix_format, WASAPI_MIX_FORMAT_SMOKE_ENV};

// ── Smoke probe behavior tests ────────────────────────────────────────────

#[test]
fn mix_format_smoke_report_default_fields_when_skipped() {
    env::remove_var(WASAPI_MIX_FORMAT_SMOKE_ENV);

    let report = probe_mix_format();

    // All prohibited operations should be false
    assert!(
        !report.initialized_audio_client,
        "initialized_audio_client should always be false"
    );
    assert!(
        !report.is_format_supported_called,
        "is_format_supported_called should always be false"
    );
    assert!(
        !report.render_client_available,
        "render_client_available should always be false"
    );
}

// ── Regression: Smoke does not initialize audio client ────────────────────

#[test]
fn mix_format_smoke_does_not_initialize_audio_client() {
    env::remove_var(WASAPI_MIX_FORMAT_SMOKE_ENV);

    let report = probe_mix_format();

    assert!(
        !report.initialized_audio_client,
        "initialized_audio_client should always be false"
    );
}

// ── Regression: Smoke does not get render client ─────────────────────────

#[test]
fn mix_format_smoke_does_not_get_render_client() {
    env::remove_var(WASAPI_MIX_FORMAT_SMOKE_ENV);

    let report = probe_mix_format();

    assert!(
        !report.render_client_available,
        "render_client_available should always be false"
    );
}

// ── Regression: Smoke does not call IsFormatSupported ────────────────────

#[test]
fn mix_format_smoke_does_not_call_is_format_supported() {
    env::remove_var(WASAPI_MIX_FORMAT_SMOKE_ENV);

    let report = probe_mix_format();

    assert!(
        !report.is_format_supported_called,
        "is_format_supported_called should always be false"
    );
}
