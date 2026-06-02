use super::super::decoder_runtime_state::DecoderRuntimePhase;
use super::super::engine::PlaybackEngine;
use super::super::errors::PlaybackError;
use super::super::types::{PlaybackTrack, TrackId};
use super::native::KivoNativeEngine;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn track() -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId("track-1".to_string()),
        title: "Track 1".to_string(),
        artist: "Artist".to_string(),
        source_path: "C:/Music/track-1.flac".to_string(),
    }
}

fn wav_track() -> (PlaybackTrack, String) {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "kivo-native-engine-load-{}-{suffix}.wav",
        std::process::id()
    ));
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44_100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&path, spec).expect("create wav file");

    for sample in [0_i16, 1000, -1000, 2000] {
        writer.write_sample(sample).expect("write wav sample");
    }
    writer.finalize().expect("finalize wav file");

    let source_path = path.to_string_lossy().into_owned();
    (
        PlaybackTrack {
            id: TrackId("track-wav-load".to_string()),
            title: "WAV Load".to_string(),
            artist: "Artist".to_string(),
            source_path: source_path.clone(),
        },
        source_path,
    )
}

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

    fs::remove_file(path).expect("remove wav file");
}

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

    fs::remove_file(path).expect("remove wav file");
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
    assert_eq!(state.volume.level, 1.0);
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
    assert!(state.volume.muted);
    assert_eq!(
        state.error.as_deref(),
        Some("kivo core audio set muted is not implemented yet")
    );
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
