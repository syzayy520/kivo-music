use super::super::super::decoder_runtime_state::DecoderRuntimePhase;
use super::super::super::engine::PlaybackEngine;
use super::super::native::KivoNativeEngine;
use super::wav_track;
use crate::playback::errors::PlaybackError;
use crate::playback::types::PlaybackStatus;

#[test]
fn shutdown_after_wav_load_closes_pipeline_children() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let _ = engine.load(track);
    engine.shutdown().expect("shutdown native engine");

    let pipeline = engine.pipeline_state();
    assert_eq!(pipeline.decoder_state.phase, DecoderRuntimePhase::Closed);
    assert!(pipeline.decoder_session.is_none());
    assert!(pipeline.last_decoded_frame.is_none());

    std::fs::remove_file(path).expect("remove wav file");
}

#[test]
fn shutdown_after_wav_load_syncs_public_state_to_stopped() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let _ = engine.load(track);
    engine.shutdown().expect("shutdown native engine");

    let state = engine.current_state();
    assert!(matches!(state.status, PlaybackStatus::Stopped));
    assert!(state.current_track.is_some());
    assert!(state.error.is_none());

    std::fs::remove_file(path).expect("remove wav file");
}

#[test]
fn seek_after_shutdown_is_rejected_as_stopped() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let _ = engine.load(track);
    engine.shutdown().expect("shutdown native engine");

    match engine.seek(0) {
        Err(PlaybackError::InvalidControlState(message)) => {
            assert!(
                message.contains("stopped"),
                "expected stopped invalid-control-state message, got {message}"
            );
        }
        Err(PlaybackError::SeekTransactionFailed(_)) => {
            panic!("shutdown seek must not reach pipeline transaction");
        }
        other => panic!("unexpected seek-after-shutdown result: {other:?}"),
    }

    std::fs::remove_file(path).expect("remove wav file");
}
