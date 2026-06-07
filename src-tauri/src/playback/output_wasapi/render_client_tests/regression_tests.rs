// render_client_tests/regression_tests.rs
//
// Regression tests verifying that render client smoke does not change
// existing output sink behavior or public engine capabilities.

use crate::playback::backends::native::KivoNativeEngine;
use crate::playback::capabilities::PlaybackCapabilities;
use crate::playback::engine::PlaybackEngine;
use crate::playback::errors::PlaybackError;
use crate::playback::native_output::KivoNativeOutputSink;
use crate::playback::output::{OutputSettings, OutputSink};
use crate::playback::output_wasapi::initialize::probe_initialize;
use crate::playback::output_wasapi::sink::WasapiOutputSink;

#[test]
fn wasapi_output_sink_scaffold_succeeds() {
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
fn native_output_still_uses_null_sink_by_default() {
    let mut sink = KivoNativeOutputSink::new();

    let status = sink
        .open(&OutputSettings::default())
        .expect("KivoNativeOutputSink should open Null Sink");

    assert!(status.is_open, "Null Sink should report is_open");
    assert!(status.is_active, "Null Sink should report is_active");
}

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
        Err(PlaybackError::UnsupportedOperation(_))
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
fn initialize_smoke_boundary_still_does_not_get_render_client() {
    // Verify that the initialize smoke still does NOT call GetService
    // This is a regression test to ensure render_client boundary is independent
    use crate::playback::output_wasapi::initialize::WASAPI_CLIENT_INIT_SMOKE_ENV;
    use std::env;

    env::remove_var(WASAPI_CLIENT_INIT_SMOKE_ENV);

    let report = probe_initialize();

    // Initialize smoke must never call GetService
    assert!(
        !report.service_requested,
        "initialize smoke should never call GetService"
    );
    assert!(
        !report.render_client_available,
        "initialize smoke should never obtain render client"
    );
    assert!(
        !report.buffer_requested,
        "initialize smoke should never call GetBuffer"
    );
    assert!(
        !report.started_audio_client,
        "initialize smoke should never call Start"
    );
    assert!(
        !report.stopped_audio_client,
        "initialize smoke should never call Stop"
    );
    assert!(
        !report.reset_audio_client,
        "initialize smoke should never call Reset"
    );
}
