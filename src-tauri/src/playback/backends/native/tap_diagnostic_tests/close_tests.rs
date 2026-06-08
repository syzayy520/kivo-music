use crate::playback::engine::PlaybackEngine;

use super::super::KivoNativeEngine;
use super::fixtures::{enable_diagnostic, remove_wav, wav_track};

#[test]
fn stop_best_effort_closes_and_detaches_diagnostic_tap() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);
    let (track, path) = wav_track("stop", 44_100);
    let _ = engine.load(track);

    let state = engine.stop().expect("stop should succeed");

    assert!(matches!(
        state.status,
        crate::playback::types::PlaybackStatus::Stopped
    ));
    assert!(state.error.is_none());

    assert!(engine.tap_diagnostic_current_report().is_none());
    assert!(engine
        .tap_diagnostic_last_detached_report()
        .is_some_and(|report| report.closed));
    remove_wav(path);
}

#[test]
fn shutdown_closes_diagnostic_before_pipeline_children() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);
    let (track, path) = wav_track("shutdown", 44_100);
    let _ = engine.load(track);

    engine.shutdown().expect("shutdown native engine");

    assert!(engine.tap_diagnostic_current_report().is_none());
    let detached = engine
        .tap_diagnostic_last_detached_report()
        .expect("final detached diagnostic report");
    assert!(detached.closed);
    assert_eq!(detached.input_count, 1);
    remove_wav(path);
}
