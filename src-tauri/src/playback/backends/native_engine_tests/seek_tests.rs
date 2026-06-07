use super::super::super::engine::PlaybackEngine;
use super::super::super::errors::PlaybackError;
use super::super::super::types::PlaybackStatus;
use super::super::native::KivoNativeEngine;
use super::wav_track;

#[test]
fn seek_without_track_returns_unsupported() {
    let mut engine = KivoNativeEngine::new();

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
}

#[test]
fn seek_after_load_returns_unsupported() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let _ = engine.load(track);
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

    std::fs::remove_file(path).expect("remove wav file");
}

#[test]
fn seek_does_not_change_status() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let _ = engine.load(track);
    let before = engine.current_state();

    let _ = engine.seek(0);

    let after = engine.current_state();
    assert!(
        matches!(after.status, PlaybackStatus::Idle),
        "status should remain Idle after seek"
    );
    assert_eq!(
        format!("{:?}", before.status),
        format!("{:?}", after.status),
        "status must not change"
    );

    std::fs::remove_file(path).expect("remove wav file");
}

#[test]
fn seek_does_not_change_current_track() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let _ = engine.load(track);
    let before = engine.current_state();

    let _ = engine.seek(500);

    let after = engine.current_state();
    assert_eq!(
        before.current_track.as_ref().map(|t| t.title.as_str()),
        after.current_track.as_ref().map(|t| t.title.as_str()),
        "current_track must not change"
    );

    std::fs::remove_file(path).expect("remove wav file");
}

#[test]
fn seek_does_not_change_timeline() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let _ = engine.load(track);
    let before = engine.current_state();

    let _ = engine.seek(999);

    let after = engine.current_state();
    assert_eq!(
        before.timeline.position_ms, after.timeline.position_ms,
        "timeline.position_ms must not change"
    );
    assert_eq!(
        before.timeline.duration_ms, after.timeline.duration_ms,
        "timeline.duration_ms must not change"
    );

    std::fs::remove_file(path).expect("remove wav file");
}

#[test]
fn seek_does_not_change_volume_and_mute() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    engine.set_volume(0.5).expect("set volume");
    engine.set_muted(true).expect("set muted");
    let _ = engine.load(track);
    let before = engine.current_state();

    let _ = engine.seek(100);

    let after = engine.current_state();
    assert_eq!(
        before.volume.level, after.volume.level,
        "volume.level must not change"
    );
    assert_eq!(
        before.volume.muted, after.volume.muted,
        "volume.muted must not change"
    );

    std::fs::remove_file(path).expect("remove wav file");
}

#[test]
fn seek_does_not_record_error_in_state() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let _ = engine.load(track);
    let _ = engine.seek(0);

    let state = engine.current_state();
    assert!(
        state.error.is_none(),
        "seek must not record error in engine state"
    );

    std::fs::remove_file(path).expect("remove wav file");
}
