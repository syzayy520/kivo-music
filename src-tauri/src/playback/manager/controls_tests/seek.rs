use super::super::PlaybackManager;
use crate::playback::errors::PlaybackError;
use crate::playback::types::{PlaybackTrack, TrackId};

fn wav_track() -> (PlaybackTrack, String) {
    let path = std::env::temp_dir().join(format!(
        "kivo-seek-gate-{}-{}.wav",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time")
            .as_nanos()
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
            id: TrackId("seek-gate-track".to_string()),
            title: "Seek Gate Track".to_string(),
            artist: "Test".to_string(),
            source_path: source_path.clone(),
        },
        source_path,
    )
}

#[test]
fn manager_seek_returns_unsupported_operation() {
    let mut manager = PlaybackManager::new();
    let (track, path) = wav_track();

    let _ = manager.load(track);
    let result = manager.seek(0);

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, "native playback seek");
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }

    std::fs::remove_file(&path).expect("remove wav file");
}

#[test]
fn manager_seek_does_not_call_primary_engine_seek() {
    let mut manager = PlaybackManager::new();
    let (track, path) = wav_track();

    let _ = manager.load(track);
    let before = manager.current_state();

    let _ = manager.seek(500);

    let after = manager.current_state();
    assert_eq!(
        before.timeline.position_ms, after.timeline.position_ms,
        "timeline.position_ms must not change when gate blocks seek"
    );

    std::fs::remove_file(&path).expect("remove wav file");
}

#[test]
fn manager_seek_blocked_preserves_status() {
    let mut manager = PlaybackManager::new();
    let (track, path) = wav_track();

    let _ = manager.load(track);
    let before_status = format!("{:?}", manager.current_state().status);

    let _ = manager.seek(0);

    let after_status = format!("{:?}", manager.current_state().status);
    assert_eq!(
        before_status, after_status,
        "status must not change when seek is gated"
    );

    std::fs::remove_file(&path).expect("remove wav file");
}

#[test]
fn manager_seek_blocked_preserves_current_track() {
    let mut manager = PlaybackManager::new();
    let (track, path) = wav_track();

    let _ = manager.load(track);
    let before_track = manager.current_state().current_track.clone();

    let _ = manager.seek(100);

    let after_track = manager.current_state().current_track.clone();
    assert_eq!(
        before_track.as_ref().map(|t| t.id.0.as_str()),
        after_track.as_ref().map(|t| t.id.0.as_str()),
        "current_track must not change when seek is gated"
    );

    std::fs::remove_file(&path).expect("remove wav file");
}
