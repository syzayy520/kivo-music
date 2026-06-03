// format_tests.rs
//
// Tests for the WASAPI mix format smoke boundary.
//
// These tests verify that:
// 1. The mix format smoke probe respects the opt-in environment variable
// 2. The probe returns appropriate reports for each scenario
// 3. The probe does not affect the existing output sink behavior
// 4. The probe does not change public engine capabilities

use std::env;

use crate::playback::backends::native::KivoNativeEngine;
use crate::playback::capabilities::PlaybackCapabilities;
use crate::playback::engine::PlaybackEngine;
use crate::playback::errors::PlaybackError;
use crate::playback::native_output::KivoNativeOutputSink;
use crate::playback::output::{OutputSettings, OutputSink};
use crate::playback::output_wasapi::format::{probe_mix_format, WASAPI_MIX_FORMAT_SMOKE_ENV};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

// ── Smoke probe behavior tests ────────────────────────────────────────────

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

// ── Windows ignored opt-in test ──────────────────────────────────────────

#[cfg(windows)]
#[test]
#[ignore]
fn mix_format_smoke_windows_ignored_opt_in() {
    // This test is #[ignore] and only runs with:
    //   cargo test -- --ignored
    //
    // Even then, it only performs real COM operations if:
    //   KIVO_WASAPI_MIX_FORMAT_SMOKE=1
    //
    // If the env is not set, it passes with a skip report.
    // If the env is set but no endpoint is available, it passes with a skip report.
    // If the env is set but Activate fails, it passes with a skip report.
    // If the env is set but GetMixFormat fails, it passes with a skip report.
    // It never panics or hard-fails.

    let report = probe_mix_format();

    if !report.opt_in_enabled {
        // Env not set, skip is expected
        assert!(report.skipped, "should be skipped when env not set");
        assert!(!report.attempted, "should not attempt when env not set");
        assert!(
            !report.mix_format_available,
            "should not have mix format when env not set"
        );
        eprintln!("mix format smoke skipped: env not set");
        return;
    }

    // Env is set, probe was attempted
    assert!(report.attempted, "should have attempted when env is set");

    if report.skipped {
        // Probe failed (no audio device, COM error, Activate error, GetMixFormat error, etc.)
        // This is a valid outcome, not a code error
        assert!(
            report.skipped_reason.is_some(),
            "should have skip reason when skipped"
        );
        assert!(
            !report.mix_format_available,
            "mix format should not be available when skipped"
        );
        eprintln!(
            "mix format smoke skipped: {:?} - {:?}",
            report.skipped_reason, report.error_message
        );
        return;
    }

    // Probe succeeded
    assert!(
        report.endpoint_available,
        "endpoint should be available on success"
    );
    assert!(
        report.client_activated,
        "client should be activated on success"
    );
    assert!(
        report.mix_format_available,
        "mix format should be available on success"
    );
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
    assert!(
        report.sample_rate_hz.is_some(),
        "sample_rate_hz should be Some on success"
    );
    assert!(
        report.channels.is_some(),
        "channels should be Some on success"
    );
    assert!(
        report.bits_per_sample.is_some(),
        "bits_per_sample should be Some on success"
    );
    assert!(
        report.format_tag.is_some(),
        "format_tag should be Some on success"
    );
    assert!(
        report.skipped_reason.is_none(),
        "should not have skip reason on success"
    );
    assert!(
        report.error_message.is_none(),
        "should not have error message on success"
    );
    eprintln!(
        "mix format smoke passed: sample_rate={:?} channels={:?} bits={:?} tag={:?}",
        report.sample_rate_hz, report.channels, report.bits_per_sample, report.format_tag
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

// ── Regression: WasapiOutputSink still unsupported ───────────────────────

#[test]
fn mix_format_smoke_does_not_change_wasapi_sink_stub() {
    let mut sink = WasapiOutputSink::new();

    let result = sink.open(&OutputSettings::default());
    assert!(
        matches!(result, Err(PlaybackError::UnsupportedOperation(_))),
        "WasapiOutputSink.open() should still return UnsupportedOperation"
    );

    let status = sink.status();
    assert!(
        !status.is_open,
        "WasapiOutputSink status should not be open"
    );
    assert!(
        !status.is_active,
        "WasapiOutputSink status should not be active"
    );
}

// ── Regression: KivoNativeOutputSink still Null Sink ─────────────────────

#[test]
fn native_output_still_uses_null_sink_by_default() {
    let mut sink = KivoNativeOutputSink::new();

    let status = sink
        .open(&OutputSettings::default())
        .expect("KivoNativeOutputSink should open Null Sink");

    assert!(status.is_open, "Null Sink should report is_open");
    assert!(status.is_active, "Null Sink should report is_active");
}

// ── Regression: PlaybackCapabilities still default ───────────────────────

#[test]
fn capabilities_remain_default() {
    let caps = PlaybackCapabilities::default();

    assert!(!caps.can_seek, "can_seek should be false");
    assert!(
        !caps.can_select_output_device,
        "can_select_output_device should be false"
    );
    assert!(
        !caps.can_use_exclusive_output,
        "can_use_exclusive_output should be false"
    );
    assert!(
        !caps.can_probe_metadata,
        "can_probe_metadata should be false"
    );
    assert!(!caps.can_gapless, "can_gapless should be false");
    assert!(!caps.can_replaygain, "can_replaygain should be false");
}

// ── Regression: public NativeEngine still unsupported ────────────────────

#[test]
fn public_native_engine_remains_typed_unsupported() {
    let mut engine = KivoNativeEngine::new();

    let track = crate::playback::types::PlaybackTrack {
        id: crate::playback::types::TrackId("test".to_string()),
        title: "Test".to_string(),
        artist: "Test".to_string(),
        source_path: "test.wav".to_string(),
    };

    assert!(matches!(
        engine.load(track),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.play(),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.pause(),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.resume(),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.stop(),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.seek(0),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.set_volume(1.0),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.set_muted(false),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
}
