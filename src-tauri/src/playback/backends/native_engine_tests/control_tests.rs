use super::super::super::engine::PlaybackEngine;
use super::super::super::errors::PlaybackError;
use super::super::native::KivoNativeEngine;
use super::wav_track;

#[test]
fn play_records_playback_error_as_state_error() {
    let mut engine = KivoNativeEngine::new();

    let result = engine.play();

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(
                message,
                "kivo core audio native playback play is not implemented yet"
            );
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }

    let state = engine.current_state();
    assert_eq!(
        state.error.as_deref(),
        Some("unsupported operation: kivo core audio native playback play is not implemented yet")
    );
}

#[test]
fn seek_returns_unsupported_without_recording_state_error() {
    let mut engine = KivoNativeEngine::new();

    let result = engine.seek(1_000);

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(
                message,
                "kivo core audio native playback seek is not implemented yet"
            );
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }

    let state = engine.current_state();
    assert!(state.error.is_none(), "seek must not record error in state");
}

#[test]
fn seek_after_wav_load_does_not_mutate_pipeline() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let _ = engine.load(track);
    let pipeline_before = engine.pipeline_state();

    let result = engine.seek(0);

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(
                message,
                "kivo core audio native playback seek is not implemented yet"
            );
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }

    let pipeline_after = engine.pipeline_state();
    assert_eq!(
        pipeline_before.decoder_session.is_some(),
        pipeline_after.decoder_session.is_some(),
        "seek must not change decoder session presence"
    );
    assert_eq!(
        pipeline_before.last_decoded_frame.is_some(),
        pipeline_after.last_decoded_frame.is_some(),
        "seek must not change last_decoded_frame presence"
    );

    std::fs::remove_file(path).expect("remove wav file");
}

#[test]
fn set_volume_updates_state_and_returns_ok() {
    let mut engine = KivoNativeEngine::new();

    let state = engine.set_volume(1.5).expect("volume should succeed");
    let pipeline = engine.pipeline_state();

    assert_eq!(state.volume.level, 1.0);
    assert_eq!(pipeline.output_status.controls.volume_level, 1.0);
    assert!(pipeline.output_status.last_error.is_none());
    assert!(state.error.is_none());
}

#[test]
fn set_muted_updates_state_and_returns_ok() {
    let mut engine = KivoNativeEngine::new();

    let state = engine.set_muted(true).expect("mute should succeed");
    let pipeline = engine.pipeline_state();

    assert!(state.volume.muted);
    assert!(pipeline.output_status.controls.muted);
    assert!(pipeline.output_status.last_error.is_none());
    assert!(state.error.is_none());
}
