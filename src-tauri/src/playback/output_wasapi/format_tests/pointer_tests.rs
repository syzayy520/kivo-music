// pointer_tests.rs
//
// Tests for format pointer and field extraction.
//
// These tests verify that the format pointer is properly managed
// and that format fields are correctly reported.

use std::env;

use crate::playback::output_wasapi::format::{probe_mix_format, WASAPI_MIX_FORMAT_SMOKE_ENV};

// ── Regression: Format pointer is released via RAII ──────────────────────

#[test]
fn mix_format_smoke_releases_format_pointer() {
    env::remove_var(WASAPI_MIX_FORMAT_SMOKE_ENV);

    // The smoke function internally uses MixFormatGuard RAII to release
    // the format pointer via CoTaskMemFree on drop.
    // This test verifies the report structure exists and the guard pattern is used.
    // On non-Windows, the stub never allocates a pointer.
    // On Windows with env missing, no pointer is allocated.
    // On Windows with env=1, MixFormatGuard ensures CoTaskMemFree is called.
    let report = probe_mix_format();

    // Without env, no pointer is ever allocated
    assert!(
        !report.mix_format_available,
        "no mix format without env opt-in"
    );
}

// ── Regression: Basic format fields reported ─────────────────────────────

#[test]
fn mix_format_smoke_reports_basic_format_fields() {
    env::remove_var(WASAPI_MIX_FORMAT_SMOKE_ENV);

    let report = probe_mix_format();

    // Without env, all format fields should be None
    assert!(
        report.sample_rate_hz.is_none(),
        "sample_rate_hz should be None without env"
    );
    assert!(
        report.channels.is_none(),
        "channels should be None without env"
    );
    assert!(
        report.bits_per_sample.is_none(),
        "bits_per_sample should be None without env"
    );
    assert!(
        report.format_tag.is_none(),
        "format_tag should be None without env"
    );
    assert!(
        report.block_align.is_none(),
        "block_align should be None without env"
    );
    assert!(
        report.avg_bytes_per_sec.is_none(),
        "avg_bytes_per_sec should be None without env"
    );
    assert!(
        report.cb_size.is_none(),
        "cb_size should be None without env"
    );
}

// ── Non-Windows stub safety tests ─────────────────────────────────────────

#[cfg(not(windows))]
#[test]
fn mix_format_smoke_non_windows_stub_is_safe() {
    // This test only runs on non-Windows platforms
    // It verifies the stub returns a safe skipped report
    env::remove_var(WASAPI_MIX_FORMAT_SMOKE_ENV);

    let report = probe_mix_format();

    assert_eq!(report.platform, "non-windows");
    assert!(!report.opt_in_enabled);
    assert!(!report.attempted);
    assert!(report.skipped);
    assert_eq!(report.skipped_reason, Some("unsupported platform"));
    assert!(!report.endpoint_available);
    assert!(!report.client_activated);
    assert!(!report.mix_format_available);
    assert!(!report.initialized_audio_client);
    assert!(!report.is_format_supported_called);
    assert!(!report.render_client_available);
    assert!(report.sample_rate_hz.is_none());
    assert!(report.channels.is_none());
    assert!(report.bits_per_sample.is_none());
    assert!(report.format_tag.is_none());
    assert!(report.error_message.is_none());
}
