use std::fs;

use super::super::super::engine::PlaybackEngine;
use super::super::super::types::PlaybackStatus;
use super::super::native::KivoNativeEngine;
use super::multi_frame_wav_track;

#[test]
fn set_muted_true_updates_state_and_pipeline_status() {
    let mut engine = KivoNativeEngine::new();

    let state = engine.set_muted(true).expect("mute succeeds");
    let pipeline = engine.pipeline_state();

    assert!(state.volume.muted);
    assert!(pipeline.output_status.controls.muted);
    assert!(state.error.is_none());
    assert!(pipeline.output_status.last_error.is_none());
}

#[test]
fn set_muted_false_updates_state_and_pipeline_status() {
    let mut engine = KivoNativeEngine::new();

    engine.set_muted(true).expect("initial mute succeeds");
    let state = engine.set_muted(false).expect("unmute succeeds");
    let pipeline = engine.pipeline_state();

    assert!(!state.volume.muted);
    assert!(!pipeline.output_status.controls.muted);
}

#[test]
fn set_muted_preserves_volume_level_track_status_and_timeline_on_success() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = multi_frame_wav_track();

    engine.set_volume(0.4).expect("volume succeeds");
    engine.load(track).expect("load should succeed");
    engine.play().expect("play should succeed");
    let before = engine.current_state();

    let state = engine.set_muted(true).expect("mute succeeds");

    assert_eq!(state.volume.level, before.volume.level);
    assert!(matches!(state.status, PlaybackStatus::Playing));
    assert!(matches!(before.status, PlaybackStatus::Playing));
    assert_eq!(
        state
            .current_track
            .as_ref()
            .map(|track| track.title.as_str()),
        before
            .current_track
            .as_ref()
            .map(|track| track.title.as_str())
    );
    assert_eq!(state.timeline.position_ms, before.timeline.position_ms);
    assert_eq!(state.timeline.duration_ms, before.timeline.duration_ms);
    assert_eq!(
        state.timeline.progress_event_interval_ms,
        before.timeline.progress_event_interval_ms
    );

    fs::remove_file(path).expect("remove wav file");
}

#[test]
fn set_muted_clears_previous_error_only_on_success() {
    let mut engine = KivoNativeEngine::new();

    let _ = engine.play();
    assert!(engine.current_state().error.is_some());

    let state = engine.set_muted(true).expect("mute succeeds");

    assert!(state.error.is_none());
}
