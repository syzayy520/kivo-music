use super::super::engine::PlaybackEngine;
use super::super::errors::PlaybackError;
use super::super::types::{PlaybackTrack, TrackId};
use super::native::KivoNativeEngine;

fn track() -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId("track-1".to_string()),
        title: "Track 1".to_string(),
        artist: "Artist".to_string(),
        source_path: "C:/Music/track-1.flac".to_string(),
    }
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
    assert_eq!(
        state.error.as_deref(),
        Some("kivo core audio load is not implemented yet")
    );
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
