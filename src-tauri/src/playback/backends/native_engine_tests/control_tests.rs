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
fn seek_records_playback_error_as_state_error() {
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
    assert_eq!(
        state.error.as_deref(),
        Some("unsupported operation: kivo core audio native playback seek is not implemented yet")
    );
}

#[test]
fn seek_after_wav_load_moves_pipeline_decoder_but_keeps_public_seek_unsupported() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let _ = engine.load(track);
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

    let pipeline = engine.pipeline_state();
    let session = pipeline
        .decoder_session
        .expect("decoder session should stay open");

    assert_eq!(session.last_position_ms, 0);
    assert!(pipeline.last_decoded_frame.is_none());

    std::fs::remove_file(path).expect("remove wav file");
}

#[test]
fn set_volume_updates_state_and_returns_typed_unsupported() {
    let mut engine = KivoNativeEngine::new();

    let result = engine.set_volume(1.5);

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, "kivo core audio set volume is not implemented yet");
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }

    let state = engine.current_state();
    let pipeline = engine.pipeline_state();

    assert_eq!(state.volume.level, 1.0);
    assert_eq!(pipeline.output_status.controls.volume_level, 1.0);
    assert!(pipeline.output_status.last_error.is_none());
    assert_eq!(
        state.error.as_deref(),
        Some("kivo core audio set volume is not implemented yet")
    );
}

#[test]
fn set_muted_updates_state_and_returns_typed_unsupported() {
    let mut engine = KivoNativeEngine::new();

    let result = engine.set_muted(true);

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, "kivo core audio set muted is not implemented yet");
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }

    let state = engine.current_state();
    let pipeline = engine.pipeline_state();

    assert!(state.volume.muted);
    assert!(pipeline.output_status.controls.muted);
    assert!(pipeline.output_status.last_error.is_none());
    assert_eq!(
        state.error.as_deref(),
        Some("kivo core audio set muted is not implemented yet")
    );
}
