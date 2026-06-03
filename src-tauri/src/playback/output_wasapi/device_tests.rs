// device_tests.rs
//
// Tests for the WASAPI endpoint smoke boundary.
//
// These tests verify that:
// 1. The endpoint smoke probe respects the opt-in environment variable
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
use crate::playback::output_wasapi::device::{probe_default_endpoint, WASAPI_ENDPOINT_SMOKE_ENV};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

// ── Smoke probe behavior tests ────────────────────────────────────────────

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

// ── Non-Windows stub safety tests ─────────────────────────────────────────

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

// ── Windows ignored opt-in test ──────────────────────────────────────────

#[cfg(windows)]
#[test]
#[ignore]
fn endpoint_smoke_windows_ignored_opt_in() {
    // This test is #[ignore] and only runs with:
    //   cargo test -- --ignored
    //
    // Even then, it only performs real COM operations if:
    //   KIVO_WASAPI_ENDPOINT_SMOKE=1
    //
    // If the env is not set, it passes with a skip report.
    // If the env is set but no endpoint is available, it passes with a skip report.
    // It never panics or hard-fails.

    let report = probe_default_endpoint();

    if !report.opt_in_enabled {
        // Env not set, skip is expected
        assert!(report.skipped, "should be skipped when env not set");
        assert!(!report.attempted, "should not attempt when env not set");
        eprintln!("endpoint smoke skipped: env not set");
        return;
    }

    // Env is set, probe was attempted
    assert!(report.attempted, "should have attempted when env is set");

    if report.skipped {
        // Probe failed (no audio device, COM error, etc.)
        // This is a valid outcome, not a code error
        assert!(
            report.skipped_reason.is_some(),
            "should have skip reason when skipped"
        );
        assert!(
            !report.endpoint_available,
            "endpoint should not be available when skipped"
        );
        eprintln!(
            "endpoint smoke skipped: {:?} - {:?}",
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
        report.skipped_reason.is_none(),
        "should not have skip reason on success"
    );
    assert!(
        report.error_message.is_none(),
        "should not have error message on success"
    );
    eprintln!("endpoint smoke passed: default endpoint available");
}

// ── Regression: WasapiOutputSink still unsupported ───────────────────────

#[test]
fn endpoint_smoke_does_not_change_wasapi_sink_stub() {
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
