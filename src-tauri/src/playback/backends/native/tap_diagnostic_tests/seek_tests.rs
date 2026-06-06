use crate::playback::engine::PlaybackEngine;

use super::super::KivoNativeEngine;
use super::fixtures::{assert_unsupported, enable_diagnostic, remove_wav, wav_track};

#[test]
fn successful_pipeline_seek_resets_diagnostic_tap() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);
    let (track, path) = wav_track("seek-success", 44_100);
    let _ = engine.load(track);

    assert_unsupported(
        engine.seek(0),
        "kivo core audio native playback seek is not implemented yet",
    );

    let report = engine
        .tap_diagnostic_current_report()
        .expect("diagnostic report after seek");
    assert_eq!(report.input_count, 0);
    assert_eq!(report.pending_frames, 0);
    assert!(report.last_error.is_none());
    assert!(engine.tap_diagnostic_last_detached_report().is_none());
    remove_wav(path);
}

#[test]
fn failed_pipeline_seek_does_not_reset_diagnostic_tap() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);
    let (track, path) = wav_track("seek-failure", 44_100);
    let _ = engine.load(track);
    engine
        .pipeline
        .close_decoder()
        .expect("close decoder before failed seek");
    let before = engine
        .tap_diagnostic_current_report()
        .expect("diagnostic report before failed seek");

    assert_unsupported(
        engine.seek(10),
        "kivo core audio native playback seek is not implemented yet",
    );

    let after = engine
        .tap_diagnostic_current_report()
        .expect("diagnostic report after failed seek");
    assert_eq!(after, before);
    assert!(engine.tap_diagnostic_last_detached_report().is_none());
    remove_wav(path);
}
