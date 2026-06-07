// reset_boundary_tests/regression_tests.rs
//
// Regression tests ensuring existing WASAPI boundaries are not broken
// by the reset_boundary module.

use crate::playback::backends::native::KivoNativeEngine;
use crate::playback::capabilities::PlaybackCapabilities;
use crate::playback::engine::PlaybackEngine;
use crate::playback::errors::PlaybackError;
use crate::playback::native_output::KivoNativeOutputSink;
use crate::playback::output::{OutputSettings, OutputSink};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

#[test]
fn wasapi_output_sink_scaffold_succeeds_after_reset_boundary() {
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

#[test]
fn native_output_still_uses_null_sink_after_reset_boundary() {
    let mut sink = KivoNativeOutputSink::new();

    let status = sink
        .open(&OutputSettings::default())
        .expect("KivoNativeOutputSink should open Null Sink");

    assert!(status.is_open, "Null Sink should report is_open");
    assert!(status.is_active, "Null Sink should report is_active");
}

#[test]
fn capabilities_remain_default_after_reset_boundary() {
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

#[test]
fn public_native_engine_keeps_native_control_contract_after_reset_boundary() {
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

#[test]
fn padding_query_boundary_unchanged() {
    // Verify padding_query module still exists and has the expected function
    let report = crate::playback::output_wasapi::padding_query::probe_padding_query();
    // Without env set, it should return a skipped report
    assert!(report.skipped, "padding_query should skip without env");
    assert_eq!(report.query_mode, "started");
}

#[test]
fn start_stop_boundary_unchanged() {
    // Verify start_stop module still exists and has the expected function
    let report = crate::playback::output_wasapi::start_stop::probe_start_stop();
    // Without env set, it should return a skipped report
    assert!(report.skipped, "start_stop should skip without env");
}

#[test]
fn buffer_boundary_unchanged() {
    // Verify buffer module still exists and has the expected function
    let report = crate::playback::output_wasapi::buffer::probe_buffer();
    // Without env set, it should return a skipped report
    assert!(report.skipped, "buffer should skip without env");
}

#[test]
fn silent_loop_boundary_unchanged() {
    // Verify silent_loop module still exists and has the expected function
    let report = crate::playback::output_wasapi::silent_loop::probe_silent_loop();
    // Without env set, it should return a skipped report
    assert!(report.skipped, "silent_loop should skip without env");
}

#[test]
fn reset_boundary_stub_returns_skipped_on_non_windows() {
    // On any platform (including Windows without env), probe_reset_boundary
    // should return a report. On non-Windows it's always skipped.
    #[cfg(not(windows))]
    {
        let report = crate::playback::output_wasapi::reset_boundary::probe_reset_boundary();
        assert!(report.skipped);
        assert_eq!(report.skipped_reason, Some("non-windows platform"));
    }
}
