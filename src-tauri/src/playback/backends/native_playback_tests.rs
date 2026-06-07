use super::super::errors::PlaybackError;
use super::super::types::PlaybackStatus;
use super::native_playback::KivoNativePlayback;

fn assert_unsupported<T>(result: Result<T, PlaybackError>, operation: &str) {
    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(
                message,
                format!("kivo core audio native playback {operation} is not implemented yet")
            );
        }
        Err(error) => panic!("expected unsupported operation, got {error:?}"),
        Ok(_) => panic!("expected unsupported operation, got success"),
    }
}

#[test]
fn play_is_typed_unsupported() {
    let playback = KivoNativePlayback::new();

    assert_unsupported(playback.play(), "play");
}

#[test]
fn pause_is_typed_unsupported() {
    let playback = KivoNativePlayback::new();

    assert_unsupported(playback.pause(), "pause");
}

#[test]
fn resume_is_typed_unsupported() {
    let playback = KivoNativePlayback::new();

    assert_unsupported(playback.resume(), "resume");
}

#[test]
fn stop_without_track_returns_idle() {
    let mut playback = KivoNativePlayback::new();

    let status = playback.stop().expect("stop should succeed");

    assert!(matches!(status, PlaybackStatus::Idle));
}

#[test]
fn stop_with_track_returns_stopped() {
    let mut playback = KivoNativePlayback::new();
    let track = crate::playback::types::PlaybackTrack {
        id: crate::playback::types::TrackId("test".to_string()),
        title: "Test".to_string(),
        artist: "Artist".to_string(),
        source_path: "/test.wav".to_string(),
    };
    playback.load_track(track);

    let status = playback.stop().expect("stop should succeed");

    assert!(matches!(status, PlaybackStatus::Stopped));
}

#[test]
fn seek_is_typed_unsupported() {
    let playback = KivoNativePlayback::new();

    assert_unsupported(playback.seek(1_000), "seek");
}
