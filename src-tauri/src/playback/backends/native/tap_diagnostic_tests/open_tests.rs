use crate::playback::engine::PlaybackEngine;

use super::super::KivoNativeEngine;
use super::fixtures::{
    assert_loaded, assert_unsupported, enable_diagnostic, remove_wav, signed16_wav_track,
    unsupported_track, wav_track,
};

#[test]
fn decoder_open_success_attaches_before_first_decode() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);
    let (track, path) = wav_track("open-success", 44_100);

    assert_loaded(engine.load(track));
    let report = engine
        .tap_diagnostic_current_report()
        .expect("diagnostic report after decoder open");

    assert_eq!(report.input_count, 1);
    assert_eq!(report.capacity_frames, 4);
    assert!(engine.tap_diagnostic_last_detached_report().is_none());
    remove_wav(path);
}

#[test]
fn same_stream_load_still_recreates_and_retains_old_report() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);
    let (track, path) = wav_track("same-stream", 44_100);

    let _ = engine.load(track.clone());
    let _ = engine.load(track);

    let detached = engine
        .tap_diagnostic_last_detached_report()
        .expect("old diagnostic report");
    let current = engine
        .tap_diagnostic_current_report()
        .expect("fresh diagnostic report");
    assert!(detached.closed);
    assert_eq!(detached.input_count, 1);
    assert!(!current.closed);
    assert_eq!(current.input_count, 1);
    remove_wav(path);
}

#[test]
fn different_stream_load_recreates_diagnostic_tap() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);
    let (first, first_path) = wav_track("stream-44100", 44_100);
    let (second, second_path) = wav_track("stream-48000", 48_000);

    let _ = engine.load(first);
    let _ = engine.load(second);

    assert!(engine
        .tap_diagnostic_last_detached_report()
        .is_some_and(|report| report.closed));
    assert_eq!(
        engine
            .tap_diagnostic_current_stream()
            .map(|stream| stream.sample_rate_hz),
        Some(48_000)
    );
    remove_wav(first_path);
    remove_wav(second_path);
}

#[test]
fn decoder_open_failure_does_not_create_diagnostic_tap() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);

    assert_unsupported(
        engine.load(unsupported_track()),
        "kivo core audio load is not implemented yet",
    );

    assert!(engine.tap_diagnostic_current_report().is_none());
    assert!(engine.tap_diagnostic_last_detached_report().is_none());
    assert_eq!(
        engine.current_state().error.as_deref(),
        Some("unsupported format: flac")
    );
}

#[test]
fn diagnostic_creation_failure_does_not_change_load_result() {
    let mut engine = KivoNativeEngine::new();
    enable_diagnostic(&mut engine, 4);
    let (track, path) = signed16_wav_track("diagnostic-create-failure");

    assert_loaded(engine.load(track));

    assert!(engine.tap_diagnostic_current_report().is_none());
    assert!(engine.current_state().error.is_none());
    remove_wav(path);
}
