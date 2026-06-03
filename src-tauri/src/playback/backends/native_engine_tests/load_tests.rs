use std::fs;

use super::super::super::engine::PlaybackEngine;
use super::super::super::errors::PlaybackError;
use super::super::super::types::{PlaybackTrack, TrackId};
use super::super::native::KivoNativeEngine;
use super::{track, wav_track};

#[test]
fn load_keeps_track_and_returns_typed_unsupported() {
    let mut engine = KivoNativeEngine::new();

    let result = engine.load(track());

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, "kivo core audio load is not implemented yet");
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }

    let state = engine.current_state();
    assert_eq!(
        state
            .current_track
            .as_ref()
            .map(|track| track.title.as_str()),
        Some("Track 1")
    );
    assert_eq!(state.error.as_deref(), Some("unsupported format: flac"));
}

#[test]
fn load_opens_wav_decoder_session_and_decodes_one_frame_submitted_to_null_sink() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let result = engine.load(track);

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, "kivo core audio load is not implemented yet");
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }

    let pipeline = engine.pipeline_state();
    let session = pipeline
        .decoder_session
        .expect("native engine load should open decoder session");
    let frame = pipeline
        .last_decoded_frame
        .expect("native engine load should decode one frame");

    assert_eq!(session.track_id, "track-wav-load");
    assert_eq!(session.stream_info.sample_rate_hz, 44_100);
    assert_eq!(session.stream_info.channels, 2);
    assert_eq!(session.decoded_frame_count, 1);
    assert_eq!(frame.stream.sample_rate_hz, 44_100);
    assert_eq!(frame.stream.channels, 2);
    assert_eq!(frame.samples.len(), 4);
    assert!(pipeline.output_status.last_error.is_none());
    assert_eq!(pipeline.output_status.pending_frames, 1);

    fs::remove_file(path).expect("remove wav file");
}

#[test]
fn load_with_track_without_source_path_extension_is_still_typed_unsupported() {
    let mut engine = KivoNativeEngine::new();
    let track = PlaybackTrack {
        id: TrackId("track-no-ext".to_string()),
        title: "No Ext".to_string(),
        artist: "Artist".to_string(),
        source_path: "C:/Music/no_extension".to_string(),
    };

    let result = engine.load(track);

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, "kivo core audio load is not implemented yet");
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }
}

#[test]
fn load_records_pipeline_error_when_decoder_open_fails_but_keeps_public_load_unsupported() {
    let mut engine = KivoNativeEngine::new();

    let result = engine.load(track());

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, "kivo core audio load is not implemented yet");
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }

    let state = engine.current_state();
    assert_eq!(
        state
            .current_track
            .as_ref()
            .map(|track| track.title.as_str()),
        Some("Track 1")
    );
    assert_eq!(state.error.as_deref(), Some("unsupported format: flac"));
}
