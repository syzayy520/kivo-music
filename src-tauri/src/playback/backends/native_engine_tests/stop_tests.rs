use super::super::super::engine::PlaybackEngine;
use super::super::super::types::PlaybackStatus;
use super::super::native::KivoNativeEngine;
use super::{multi_frame_wav_track, wav_track};

#[test]
fn native_stop_without_loaded_track_is_idle_noop_success() {
    let mut engine = KivoNativeEngine::new();

    let state = engine.stop().expect("stop without track should succeed");

    assert!(matches!(state.status, PlaybackStatus::Idle));
    assert!(state.current_track.is_none());
    assert!(state.error.is_none());

    let current = engine.current_state();
    assert!(matches!(current.status, PlaybackStatus::Idle));
    assert!(current.current_track.is_none());
    assert!(current.error.is_none());

    let pipeline = engine.pipeline_state();
    assert_eq!(pipeline.output_status.pending_frames, 0);
    assert!(!pipeline.output_status.is_active);
}

#[test]
fn native_stop_after_load_preserves_track_sets_stopped_and_clears_pending_frames() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    engine.load(track).expect("load should succeed");

    let state = engine.stop().expect("stop after load should succeed");

    assert!(matches!(state.status, PlaybackStatus::Stopped));
    assert_eq!(
        state.current_track.as_ref().map(|t| t.title.as_str()),
        Some("WAV Load")
    );
    assert!(state.error.is_none());

    let current = engine.current_state();
    assert!(matches!(current.status, PlaybackStatus::Stopped));
    assert!(current.error.is_none());

    let pipeline = engine.pipeline_state();
    assert_eq!(pipeline.output_status.pending_frames, 0);
    assert!(!pipeline.output_status.is_active);

    std::fs::remove_file(path).expect("remove wav file");
}

#[test]
fn native_stop_after_play_sets_stopped_clears_error_and_pending_frames() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = multi_frame_wav_track();

    engine.load(track).expect("load should succeed");
    engine.play().expect("play should succeed");

    let state = engine.stop().expect("stop after play should succeed");

    assert!(matches!(state.status, PlaybackStatus::Stopped));
    assert_eq!(
        state.current_track.as_ref().map(|t| t.title.as_str()),
        Some("WAV Play")
    );
    assert!(state.error.is_none());

    let current = engine.current_state();
    assert!(matches!(current.status, PlaybackStatus::Stopped));
    assert!(current.error.is_none());

    let pipeline = engine.pipeline_state();
    assert_eq!(pipeline.output_status.pending_frames, 0);
    assert!(!pipeline.output_status.is_active);

    std::fs::remove_file(path).expect("remove wav file");
}

#[test]
fn native_stop_is_idempotent() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    engine.load(track).expect("load should succeed");

    let first = engine.stop().expect("first stop should succeed");
    let second = engine.stop().expect("second stop should succeed");

    assert!(matches!(first.status, PlaybackStatus::Stopped));
    assert!(matches!(second.status, PlaybackStatus::Stopped));
    assert_eq!(
        first.current_track.as_ref().map(|t| t.title.as_str()),
        second.current_track.as_ref().map(|t| t.title.as_str())
    );
    assert!(first.error.is_none());
    assert!(second.error.is_none());

    let pipeline = engine.pipeline_state();
    assert_eq!(pipeline.output_status.pending_frames, 0);
    assert!(!pipeline.output_status.is_active);

    std::fs::remove_file(path).expect("remove wav file");
}

#[test]
fn native_stop_after_eos_before_submit_failure_clears_error_and_pending_frames() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    engine.load(track).expect("load should succeed");

    let play_result = engine.play();
    assert!(play_result.is_err());

    let state = engine.current_state();
    assert!(state.error.is_some(), "play error should be recorded");

    let stop_state = engine
        .stop()
        .expect("stop after eos failure should succeed");

    assert!(matches!(stop_state.status, PlaybackStatus::Stopped));
    assert!(stop_state.current_track.is_some());
    assert!(stop_state.error.is_none(), "stop should clear error");

    let current = engine.current_state();
    assert!(current.error.is_none());

    let pipeline = engine.pipeline_state();
    assert_eq!(pipeline.output_status.pending_frames, 0);
    assert!(!pipeline.output_status.is_active);

    std::fs::remove_file(path).expect("remove wav file");
}
