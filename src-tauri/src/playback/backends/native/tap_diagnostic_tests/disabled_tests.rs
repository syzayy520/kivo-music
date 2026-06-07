use crate::playback::engine::PlaybackEngine;

use super::super::KivoNativeEngine;
use super::fixtures::{assert_loaded, remove_wav, wav_track};

#[test]
fn new_engine_keeps_tap_diagnostic_disabled() {
    let engine = KivoNativeEngine::new();

    assert!(engine.tap_diagnostic.policy().is_none());
    assert!(engine.tap_diagnostic_current_report().is_none());
    assert!(engine.tap_diagnostic_last_detached_report().is_none());
}

#[test]
fn disabled_load_does_not_attach_hidden_diagnostic_tap() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track("disabled-load", 44_100);

    assert_loaded(engine.load(track));

    assert!(engine.tap_diagnostic.policy().is_none());
    assert!(engine.tap_diagnostic_current_report().is_none());
    assert!(engine.tap_diagnostic_last_detached_report().is_none());
    remove_wav(path);
}
