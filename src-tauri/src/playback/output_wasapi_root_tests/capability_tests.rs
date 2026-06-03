// output_wasapi_root_tests/capability_tests.rs
//
// Tests for WASAPI capability and native output behavior.

use super::support::test_frame;
use crate::playback::backends::native::KivoNativeEngine;
use crate::playback::capabilities::PlaybackCapabilities;
use crate::playback::engine::PlaybackEngine;
use crate::playback::errors::PlaybackError;
use crate::playback::native_output::KivoNativeOutputSink;
use crate::playback::output::{OutputSettings, OutputSink};

#[test]
fn native_output_still_uses_null_sink_by_default() {
    let mut sink = KivoNativeOutputSink::new();

    // Should open Null Sink successfully
    let status = sink
        .open(&OutputSettings::default())
        .expect("KivoNativeOutputSink should open Null Sink");

    assert!(status.is_open, "Null Sink should report is_open");
    assert!(status.is_active, "Null Sink should report is_active");

    // Submit frame should succeed
    let status = sink
        .submit_frame(test_frame())
        .expect("Null Sink should accept frames");
    assert_eq!(
        status.pending_frames, 1,
        "Null Sink should track pending frames"
    );
}

#[test]
fn null_sink_regression_still_passes() {
    // This is a duplicate of the existing null_sink_open_succeeds_and_sets_boundary_active test
    // to ensure Null Sink behavior is unchanged.
    let mut sink = KivoNativeOutputSink::new();

    let status = sink
        .open(&OutputSettings::default())
        .expect("null sink open should succeed");

    assert!(status.is_open);
    assert!(status.is_active);
    assert!(status.last_error.is_none());
}

#[test]
fn public_native_engine_remains_typed_unsupported() {
    let mut engine = KivoNativeEngine::new();

    // All public methods should return UnsupportedOperation
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
