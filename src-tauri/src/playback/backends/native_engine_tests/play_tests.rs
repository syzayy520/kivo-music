use std::fs;

use super::super::super::engine::PlaybackEngine;
use super::super::super::errors::PlaybackError;
use super::super::super::types::PlaybackStatus;
use super::super::native::KivoNativeEngine;
use super::{multi_frame_wav_track, wav_track};

#[test]
fn native_playback_after_load_pumps_pipeline_to_native_null_output_and_returns_playing() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = multi_frame_wav_track();

    let load_state = engine
        .load(track)
        .expect("native load should prepare the pipeline");
    assert!(matches!(load_state.status, PlaybackStatus::Idle));

    let before_play = engine.pipeline_state();
    assert_eq!(before_play.output_status.pending_frames, 1);
    assert_eq!(
        before_play
            .decoder_session
            .as_ref()
            .expect("decoder session should be open after load")
            .decoded_frame_count,
        1
    );

    let play_state = engine
        .play()
        .expect("native play should submit one bounded frame to NullOutput");

    assert!(matches!(play_state.status, PlaybackStatus::Playing));
    assert!(play_state.error.is_none());
    assert_eq!(
        play_state
            .current_track
            .as_ref()
            .map(|track| track.title.as_str()),
        Some("WAV Play")
    );

    let after_play = engine.pipeline_state();
    assert_eq!(after_play.output_status.pending_frames, 2);
    assert!(after_play.output_status.last_error.is_none());
    assert_eq!(
        after_play
            .decoder_session
            .as_ref()
            .expect("decoder session should remain open after bounded play pump")
            .decoded_frame_count,
        2
    );

    fs::remove_file(path).expect("remove wav file");
}

#[test]
fn native_playback_does_not_return_playing_when_end_of_stream_precedes_native_null_submit() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    engine
        .load(track)
        .expect("native load should submit the only frame at the existing load boundary");

    let result = engine.play();

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert_eq!(
                message,
                "native pipeline reached end-of-stream before NullOutput submit"
            );
        }
        other => panic!("expected end-of-stream backend error, got {other:?}"),
    }

    let state = engine.current_state();
    assert!(matches!(state.status, PlaybackStatus::Idle));
    assert_eq!(
        state.error.as_deref(),
        Some("backend error: native pipeline reached end-of-stream before NullOutput submit")
    );

    let pipeline = engine.pipeline_state();
    assert_eq!(pipeline.output_status.pending_frames, 1);

    fs::remove_file(path).expect("remove wav file");
}
