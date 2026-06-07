use crate::playback::engine::PlaybackEngine;
use crate::playback::errors::PlaybackError;

use super::super::KivoNativeEngine;
use super::fixtures::{enable_diagnostic, remove_wav, wav_track};

#[test]
fn seek_does_not_reset_diagnostic_tap() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);
    let (track, path) = wav_track("seek-no-reset", 44_100);
    let _ = engine.load(track);

    let before = engine
        .tap_diagnostic_current_report()
        .expect("diagnostic report before seek");

    // Loaded/Idle seek succeeds — tap diagnostic must not reset
    let result = engine.seek(0);
    assert!(result.is_ok(), "seek on loaded idle should succeed, got {result:?}");

    let after = engine
        .tap_diagnostic_current_report()
        .expect("diagnostic report after seek");
    assert_eq!(after.input_count, before.input_count);
    assert_eq!(after.pending_frames, before.pending_frames);
    assert_eq!(after.last_error, before.last_error);
    assert!(engine.tap_diagnostic_last_detached_report().is_none());
    remove_wav(path);
}

#[test]
fn seek_does_not_reset_diagnostic_tap_when_decoder_closed() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);
    let (track, path) = wav_track("seek-failure", 44_100);
    let _ = engine.load(track);
    engine
        .pipeline
        .close_decoder()
        .expect("close decoder before seek");
    let before = engine
        .tap_diagnostic_current_report()
        .expect("diagnostic report before seek");

    // Decoder closed → seek transaction fails — tap diagnostic must not reset
    let result = engine.seek(10);
    match result {
        Err(PlaybackError::SeekTransactionFailed(_)) => {}
        other => panic!("expected seek transaction failed, got {other:?}"),
    }

    let after = engine
        .tap_diagnostic_current_report()
        .expect("diagnostic report after seek");
    assert_eq!(after, before);
    assert!(engine.tap_diagnostic_last_detached_report().is_none());
    remove_wav(path);
}
