use std::fs;

use super::super::super::engine::PlaybackEngine;
use super::super::super::errors::PlaybackError;
use super::super::super::types::{PlaybackStatus, PlaybackTrack, TrackId};
use super::super::native::KivoNativeEngine;
use super::{multi_frame_wav_track, track, wav_track};

#[test]
fn native_playback_load_failure_does_not_commit_track_or_status_success() {
    let mut engine = KivoNativeEngine::new();

    let result = engine.load(track());

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, "kivo core audio load is not implemented yet");
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }

    let state = engine.current_state();
    assert!(state.current_track.is_none());
    assert!(matches!(state.status, PlaybackStatus::Idle));
    assert_eq!(state.error.as_deref(), Some("unsupported format: flac"));
}

#[test]
fn native_playback_load_sets_track_and_idle_after_existing_null_boundary_submit() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let state = engine
        .load(track)
        .expect("native load should succeed after pipeline boundary succeeds");

    assert!(matches!(state.status, PlaybackStatus::Idle));
    assert_eq!(
        state
            .current_track
            .as_ref()
            .map(|track| track.title.as_str()),
        Some("WAV Load")
    );
    assert!(state.error.is_none());

    let pipeline = engine.pipeline_state();
    let session = pipeline
        .decoder_session
        .expect("native engine load should open decoder session");
    let frame = pipeline
        .last_decoded_frame
        .expect("native engine load should decode one frame");

    assert_eq!(session.track_id, "track-wav-load");
    assert_eq!(session.duration_ms, Some(0));
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
fn native_playback_load_sets_known_duration_ms_from_decoder_session() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = multi_frame_wav_track();

    let state = engine
        .load(track)
        .expect("native load should succeed after pipeline boundary succeeds");

    assert!(matches!(state.status, PlaybackStatus::Idle));
    assert_eq!(state.timeline.position_ms, 0);
    assert_eq!(state.timeline.duration_ms, Some(46));
    assert_eq!(
        state
            .current_track
            .as_ref()
            .map(|track| track.title.as_str()),
        Some("WAV Play")
    );

    let pipeline = engine.pipeline_state();
    let session = pipeline
        .decoder_session
        .expect("native engine load should open decoder session");
    assert_eq!(session.duration_ms, Some(46));

    fs::remove_file(path).expect("remove wav file");
}

#[test]
fn native_playback_load_without_source_path_extension_returns_real_pipeline_error() {
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

    let state = engine.current_state();
    assert_eq!(
        state.error.as_deref(),
        Some("unsupported format: missing file extension")
    );
}

#[test]
fn native_playback_load_failure_keeps_existing_loaded_track() {
    let mut engine = KivoNativeEngine::new();
    let (loaded_track, path) = wav_track();
    engine
        .load(loaded_track)
        .expect("initial native load should succeed");

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
        Some("WAV Load")
    );
    assert_eq!(state.error.as_deref(), Some("unsupported format: flac"));

    fs::remove_file(path).expect("remove wav file");
}
