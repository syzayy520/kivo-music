use std::fs;

use super::super::super::engine::PlaybackEngine;
use super::super::super::errors::PlaybackError;
use super::super::super::types::PlaybackStatus;
use super::super::native::KivoNativeEngine;
use super::multi_frame_wav_track;

#[test]
fn set_volume_finite_updates_state_and_pipeline_status() {
    let mut engine = KivoNativeEngine::new();

    let state = engine.set_volume(0.25).expect("finite volume succeeds");
    let pipeline = engine.pipeline_state();

    assert_eq!(state.volume.level, 0.25);
    assert_eq!(pipeline.output_status.controls.volume_level, 0.25);
    assert!(state.error.is_none());
    assert!(pipeline.output_status.last_error.is_none());
}

#[test]
fn set_volume_clamps_below_zero_after_pipeline_success() {
    let mut engine = KivoNativeEngine::new();

    let state = engine.set_volume(-0.5).expect("finite volume succeeds");
    let pipeline = engine.pipeline_state();

    assert_eq!(state.volume.level, 0.0);
    assert_eq!(pipeline.output_status.controls.volume_level, 0.0);
}

#[test]
fn set_volume_clamps_above_one_after_pipeline_success() {
    let mut engine = KivoNativeEngine::new();

    let state = engine.set_volume(1.5).expect("finite volume succeeds");
    let pipeline = engine.pipeline_state();

    assert_eq!(state.volume.level, 1.0);
    assert_eq!(pipeline.output_status.controls.volume_level, 1.0);
}

#[test]
fn set_volume_preserves_muted_track_status_and_timeline_on_success() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = multi_frame_wav_track();

    engine.load(track).expect("load should succeed");
    engine.set_muted(true).expect("mute should succeed");
    engine.play().expect("play should succeed");
    let before = engine.current_state();

    let state = engine.set_volume(0.4).expect("finite volume succeeds");

    assert!(state.volume.muted);
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
fn set_volume_clears_previous_error_only_on_success() {
    let mut engine = KivoNativeEngine::new();

    let _ = engine.play();
    assert!(engine.current_state().error.is_some());

    let state = engine.set_volume(0.3).expect("finite volume succeeds");
    assert!(state.error.is_none());

    let _ = engine.play();
    let error_before_invalid = engine.current_state().error;
    let result = engine.set_volume(f32::NAN);

    assert!(matches!(result, Err(PlaybackError::InvalidControlInput(_))));
    assert_eq!(engine.current_state().error, error_before_invalid);
}

#[test]
fn set_volume_nan_returns_invalid_control_input_without_side_effects() {
    assert_invalid_volume_without_side_effects(f32::NAN);
}

#[test]
fn set_volume_positive_infinity_returns_invalid_control_input_without_side_effects() {
    assert_invalid_volume_without_side_effects(f32::INFINITY);
}

#[test]
fn set_volume_negative_infinity_returns_invalid_control_input_without_side_effects() {
    assert_invalid_volume_without_side_effects(f32::NEG_INFINITY);
}

fn assert_invalid_volume_without_side_effects(level: f32) {
    let mut engine = KivoNativeEngine::new();
    engine.set_volume(0.4).expect("initial volume succeeds");
    let _ = engine.play();

    let before_state = engine.current_state();
    let before_pipeline = engine.pipeline_state();
    let result = engine.set_volume(level);

    match result {
        Err(PlaybackError::InvalidControlInput(message)) => {
            assert_eq!(message, "volume level must be finite");
        }
        other => panic!("expected invalid control input, got {other:?}"),
    }

    let after_state = engine.current_state();
    let after_pipeline = engine.pipeline_state();

    assert_eq!(after_state.volume.level, before_state.volume.level);
    assert_eq!(after_state.error, before_state.error);
    assert_eq!(
        after_pipeline.output_status.controls.volume_level,
        before_pipeline.output_status.controls.volume_level
    );
    assert_eq!(
        after_pipeline.output_status.controls.muted,
        before_pipeline.output_status.controls.muted
    );
    assert_eq!(
        after_pipeline.output_status.last_error,
        before_pipeline.output_status.last_error
    );
}
