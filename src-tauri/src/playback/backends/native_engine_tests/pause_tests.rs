use std::fs;

use super::super::super::engine::PlaybackEngine;
use super::super::super::errors::PlaybackError;
use super::super::super::types::PlaybackStatus;
use super::super::native::KivoNativeEngine;
use super::{multi_frame_wav_track, wav_track};

#[test]
fn pause_without_track_is_idle_noop_success() {
    let mut engine = KivoNativeEngine::new();

    let state = engine.pause().expect("pause without track should succeed");

    assert!(matches!(state.status, PlaybackStatus::Idle));
    assert!(state.current_track.is_none());
    assert!(state.error.is_none());
    assert!(matches!(
        engine.current_state().status,
        PlaybackStatus::Idle
    ));
}

#[test]
fn pause_loaded_idle_is_idle_noop_success() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    engine.load(track).expect("load should succeed");
    let before = engine.current_state();
    let state = engine.pause().expect("pause loaded idle should succeed");

    assert!(matches!(state.status, PlaybackStatus::Idle));
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
    assert_eq!(state.volume.level, before.volume.level);
    assert_eq!(state.volume.muted, before.volume.muted);
    assert_eq!(state.metadata.is_some(), before.metadata.is_some());

    let _ = fs::remove_file(path);
}

#[test]
fn pause_after_play_transitions_playing_to_paused() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = multi_frame_wav_track();

    engine.load(track).expect("load should succeed");
    engine.play().expect("play should succeed");

    let state = engine.pause().expect("pause after play should succeed");
    let current = engine.current_state();

    assert!(matches!(state.status, PlaybackStatus::Paused));
    assert!(matches!(current.status, PlaybackStatus::Paused));
    assert!(state.error.is_none());

    let _ = fs::remove_file(path);
}

#[test]
fn pause_when_already_paused_is_idempotent() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = multi_frame_wav_track();

    engine.load(track).expect("load should succeed");
    engine.play().expect("play should succeed");
    engine.pause().expect("first pause should succeed");
    let before_pipeline = engine.pipeline_state();

    let state = engine.pause().expect("second pause should succeed");
    let after_pipeline = engine.pipeline_state();

    assert!(matches!(state.status, PlaybackStatus::Paused));
    assert_eq!(
        after_pipeline.output_status.pending_frames,
        before_pipeline.output_status.pending_frames
    );
    assert!(state.error.is_none());

    let _ = fs::remove_file(path);
}

#[test]
fn pause_when_stopped_is_stopped_noop_success() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    engine.load(track).expect("load should succeed");
    engine.stop().expect("stop should succeed");
    let before = engine.current_state();

    let state = engine.pause().expect("pause stopped should succeed");

    assert!(matches!(state.status, PlaybackStatus::Stopped));
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
    assert_eq!(state.error, before.error);

    let _ = fs::remove_file(path);
}

#[test]
fn pause_preserves_current_track_volume_timeline_metadata() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = multi_frame_wav_track();

    engine.set_volume(0.4).expect("volume should succeed");
    engine.set_muted(true).expect("mute should succeed");
    engine.load(track).expect("load should succeed");
    engine.play().expect("play should succeed");
    let before = engine.current_state();

    let state = engine.pause().expect("pause should succeed");

    assert!(matches!(state.status, PlaybackStatus::Paused));
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
    assert_eq!(state.volume.level, before.volume.level);
    assert_eq!(state.volume.muted, before.volume.muted);
    assert_eq!(state.timeline.position_ms, before.timeline.position_ms);
    assert_eq!(state.timeline.duration_ms, before.timeline.duration_ms);
    assert_eq!(
        state.timeline.progress_event_interval_ms,
        before.timeline.progress_event_interval_ms
    );
    assert_eq!(state.metadata.is_some(), before.metadata.is_some());

    let _ = fs::remove_file(path);
}

#[test]
fn pause_does_not_change_seek_contract() {
    let mut engine = KivoNativeEngine::new();

    engine.pause().expect("pause should succeed");
    let result = engine.seek(0);

    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));
}

#[test]
fn pause_noop_states_preserve_error_policy_where_observable() {
    let mut engine = KivoNativeEngine::new();

    let _ = engine.play();
    let before = engine.current_state();
    let state = engine.pause().expect("pause noop should succeed");

    assert!(matches!(state.status, PlaybackStatus::Idle));
    assert_eq!(state.error, before.error);
    assert!(state.error.is_some());
}
