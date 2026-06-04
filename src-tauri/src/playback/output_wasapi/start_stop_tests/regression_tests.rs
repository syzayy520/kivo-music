// start_stop_tests/regression_tests.rs
//
// Regression tests verifying that start/stop smoke does not change
// existing output sink behavior or public engine capabilities.

use crate::playback::backends::native::KivoNativeEngine;
use crate::playback::capabilities::PlaybackCapabilities;
use crate::playback::engine::PlaybackEngine;
use crate::playback::errors::PlaybackError;
use crate::playback::native_output::KivoNativeOutputSink;
use crate::playback::output::{OutputSettings, OutputSink};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

#[test]
fn wasapi_output_sink_scaffold_succeeds_after_start_stop_boundary() {
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
fn native_output_still_uses_null_sink_after_start_stop_boundary() {
    let mut sink = KivoNativeOutputSink::new();

    let status = sink
        .open(&OutputSettings::default())
        .expect("KivoNativeOutputSink should open Null Sink");

    assert!(status.is_open, "Null Sink should report is_open");
    assert!(status.is_active, "Null Sink should report is_active");
}

#[test]
fn capabilities_remain_default_after_start_stop_boundary() {
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
fn public_native_engine_remains_unsupported_after_start_stop_boundary() {
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

#[test]
fn buffer_smoke_boundary_still_does_not_start_by_default() {
    use crate::playback::output_wasapi::buffer::{probe_buffer, WASAPI_BUFFER_SMOKE_ENV};
    use std::env;

    env::remove_var(WASAPI_BUFFER_SMOKE_ENV);

    let report = probe_buffer();

    assert!(
        !report.started_audio_client,
        "buffer smoke should never call Start"
    );
    assert!(
        !report.stopped_audio_client,
        "buffer smoke should never call Stop"
    );
    assert!(
        !report.reset_audio_client,
        "buffer smoke should never call Reset"
    );
}
