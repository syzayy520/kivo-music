use super::super::errors::PlaybackError;
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
fn stop_is_typed_unsupported() {
    let playback = KivoNativePlayback::new();

    assert_unsupported(playback.stop(), "stop");
}

#[test]
fn seek_is_typed_unsupported() {
    let playback = KivoNativePlayback::new();

    assert_unsupported(playback.seek(1_000), "seek");
}
