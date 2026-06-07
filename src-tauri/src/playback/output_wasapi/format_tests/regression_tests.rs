// regression_tests.rs
//
// Tests for existing behavior preservation.
//
// These tests verify that the mix format smoke boundary does not
// change existing output sink behavior or public engine capabilities.

use crate::playback::backends::native::KivoNativeEngine;
use crate::playback::capabilities::PlaybackCapabilities;
use crate::playback::engine::PlaybackEngine;
use crate::playback::errors::PlaybackError;
use crate::playback::native_output::KivoNativeOutputSink;
use crate::playback::output::{OutputSettings, OutputSink};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

// ── Regression: WasapiOutputSink still unsupported ───────────────────────

#[test]
fn mix_format_smoke_does_not_change_wasapi_sink_scaffold() {
    let mut sink = WasapiOutputSink::new();

    let result = sink.open(&OutputSettings::default());
    assert!(
        result.is_ok(),
        "WasapiOutputSink.open() scaffold should succeed"
    );

    let status = result.unwrap();
    assert!(
        status.is_open,
        "WasapiOutputSink scaffold should report is_open after open"
    );
    assert!(
        status.is_active,
        "WasapiOutputSink scaffold should report is_active after open"
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
fn public_native_engine_keeps_native_control_contract() {
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
    let pause_state = engine
        .pause()
        .expect("native pause without track should succeed after P0-147");
    assert!(matches!(
        pause_state.status,
        crate::playback::types::PlaybackStatus::Idle
    ));
    let resume_state = engine
        .resume()
        .expect("native resume without track should succeed after P0-147");
    assert!(matches!(
        resume_state.status,
        crate::playback::types::PlaybackStatus::Idle
    ));
    assert!(engine.stop().is_ok(), "stop should succeed");
    assert!(matches!(
        engine.seek(0),
        Err(PlaybackError::NoTrack(_))
    ));
    let volume_state = engine
        .set_volume(1.0)
        .expect("native set_volume should succeed after P0-145");
    assert_eq!(volume_state.volume.level, 1.0);

    let muted_state = engine
        .set_muted(false)
        .expect("native set_muted should succeed after P0-145");
    assert!(!muted_state.volume.muted);
}
