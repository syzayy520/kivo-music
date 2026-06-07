use super::super::PlaybackManager;
use crate::playback::errors::PlaybackError;
use crate::playback::types::{PlaybackStatus, PlaybackTrack, TrackId};

/// Creates a 1-second WAV file (88200 stereo samples at 44100Hz) for position-change tests.
fn long_wav_track() -> (PlaybackTrack, String) {
    let path = std::env::temp_dir().join(format!(
        "kivo-seek-long-{}-{}.wav",
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
    // 44100 frames * 2 channels = 88200 samples → 1 second
    for i in 0..88200 {
        let sample = ((i as i16) % 1000) - 500;
        writer.write_sample(sample).expect("write wav sample");
    }
    writer.finalize().expect("finalize wav file");

    let source_path = path.to_string_lossy().into_owned();
    (
        PlaybackTrack {
            id: TrackId("seek-long-track".to_string()),
            title: "Seek Long Track".to_string(),
            artist: "Test".to_string(),
            source_path: source_path.clone(),
        },
        source_path,
    )
}

// ─── Loaded/Idle seek success tests ────────────────────────────────────────────

#[test]
fn manager_seek_loaded_idle_returns_ok() {
    let mut manager = PlaybackManager::new();
    let (track, path) = long_wav_track();

    let load_result = manager.load(track);
    assert!(load_result.is_ok(), "load must succeed: {load_result:?}");

    let result = manager.seek(500);
    assert!(
        result.is_ok(),
        "seek in Loaded/Idle must return Ok, got: {result:?}"
    );

    let state = result.unwrap();
    assert_eq!(
        state.timeline.position_ms, 500,
        "position_ms must reflect seek target"
    );

    let _ = std::fs::remove_file(&path);
}

#[test]
fn manager_seek_loaded_idle_updates_position() {
    let mut manager = PlaybackManager::new();
    let (track, path) = long_wav_track();

    let _ = manager.load(track);
    let before = manager.current_state();
    assert_eq!(before.timeline.position_ms, 0, "initial position must be 0");

    let result = manager.seek(500).expect("seek must succeed");
    assert_eq!(
        result.timeline.position_ms, 500,
        "returned state position_ms must be 500"
    );

    let after = manager.current_state();
    assert_eq!(
        after.timeline.position_ms, 500,
        "current_state position_ms must be 500 after seek"
    );

    let _ = std::fs::remove_file(&path);
}

#[test]
fn manager_seek_loaded_idle_preserves_status() {
    let mut manager = PlaybackManager::new();
    let (track, path) = long_wav_track();

    let _ = manager.load(track);
    let before_status = manager.current_state().status;

    let result = manager.seek(500).expect("seek must succeed");

    assert!(
        matches!(result.status, PlaybackStatus::Idle),
        "returned status must be Idle, got {:?}",
        result.status
    );
    assert!(
        matches!(before_status, PlaybackStatus::Idle),
        "status before seek must be Idle, got {before_status:?}"
    );

    let after_status = manager.current_state().status;
    assert!(
        matches!(after_status, PlaybackStatus::Idle),
        "status after seek must remain Idle, got {after_status:?}"
    );

    let _ = std::fs::remove_file(&path);
}

#[test]
fn manager_seek_loaded_idle_preserves_current_track() {
    let mut manager = PlaybackManager::new();
    let (track, path) = long_wav_track();

    let _ = manager.load(track);
    let before_track = manager.current_state().current_track;

    let result = manager.seek(500).expect("seek must succeed");

    assert!(
        result.current_track.is_some(),
        "returned state must have current_track"
    );
    assert_eq!(
        before_track.as_ref().map(|t| t.id.0.as_str()),
        result.current_track.as_ref().map(|t| t.id.0.as_str()),
        "current_track id must not change after seek"
    );

    let after_track = manager.current_state().current_track;
    assert_eq!(
        before_track.as_ref().map(|t| t.id.0.as_str()),
        after_track.as_ref().map(|t| t.id.0.as_str()),
        "current_state track must not change after seek"
    );

    let _ = std::fs::remove_file(&path);
}

// ─── Command-visible error shape tests ─────────────────────────────────────────

/// NoTrack: seek with no loaded track.
/// External JSON shape: {"NoTrack": "seek requires a loaded track"}
#[test]
fn manager_seek_no_track_returns_no_track() {
    let mut manager = PlaybackManager::new();

    let result = manager.seek(0);

    match result {
        Err(PlaybackError::NoTrack(message)) => {
            assert_eq!(
                message, "seek requires a loaded track",
                "NoTrack message must match backend contract"
            );
        }
        other => panic!("expected NoTrack, got {other:?}"),
    }
}

/// SeekOutOfRange: seek beyond known duration after loading a 1-second WAV.
/// External JSON shape: {"SeekOutOfRange": {"position_ms": N, "duration_ms": M}}
#[test]
fn manager_seek_out_of_range_returns_seek_out_of_range() {
    let mut manager = PlaybackManager::new();
    let (track, path) = long_wav_track();

    manager.load(track).expect("load must succeed");
    let duration_ms = manager
        .current_state()
        .timeline
        .duration_ms
        .expect("load should expose known duration");
    assert_eq!(duration_ms, 1_000);
    let position_ms = duration_ms + 1;

    let result = manager.seek(position_ms);

    match result {
        Err(PlaybackError::SeekOutOfRange {
            position_ms: actual_position_ms,
            duration_ms: actual_duration_ms,
        }) => {
            assert_eq!(actual_position_ms, position_ms);
            assert_eq!(actual_duration_ms, duration_ms);
        }
        other => panic!("expected SeekOutOfRange, got {other:?}"),
    }

    let state = manager.current_state();
    assert_eq!(state.timeline.position_ms, 0);

    let _ = std::fs::remove_file(&path);
}

/// Known-duration boundary: seeking exactly at duration is allowed.
#[test]
fn manager_seek_at_known_duration_boundary_succeeds() {
    let mut manager = PlaybackManager::new();
    let (track, path) = long_wav_track();

    manager.load(track).expect("load must succeed");
    let duration_ms = manager
        .current_state()
        .timeline
        .duration_ms
        .expect("load should expose known duration");
    assert_eq!(duration_ms, 1_000);

    let state = manager
        .seek(duration_ms)
        .expect("seek at known duration boundary should succeed");
    assert_eq!(state.timeline.position_ms, duration_ms);

    let _ = std::fs::remove_file(&path);
}
/// Paused InvalidControlState: seek while paused.
/// External JSON shape: {"InvalidControlState": "seek while paused requires output flush contract"}
/// Natural seam: requires engine in Paused state. load -> Idle, play -> Playing, pause -> Paused.
#[test]
fn manager_seek_paused_returns_invalid_control_state() {
    let mut manager = PlaybackManager::new();
    let (track, path) = long_wav_track();

    let _ = manager.load(track);

    // Transition to Paused state; test environment must support WASAPI
    manager
        .play()
        .expect("test environment must enter Playing for paused seek proof");
    manager
        .pause()
        .expect("test environment must enter Paused for paused seek proof");

    let result = manager.seek(500);
    match result {
        Err(PlaybackError::InvalidControlState(message)) => {
            assert_eq!(
                message, "seek while paused requires output flush contract",
                "Paused error message must match exact contract"
            );
        }
        other => panic!("expected InvalidControlState for paused seek, got {other:?}"),
    }

    let _ = std::fs::remove_file(&path);
}

/// Playing InvalidControlState: seek while playing.
/// External JSON shape: {"InvalidControlState": "seek while playing requires output flush contract"}
/// Natural seam: requires engine in Playing state. load -> Idle, play -> Playing.
#[test]
fn manager_seek_playing_returns_invalid_control_state() {
    let mut manager = PlaybackManager::new();
    let (track, path) = long_wav_track();

    let _ = manager.load(track);

    // Transition to Playing state; test environment must support WASAPI
    manager
        .play()
        .expect("test environment must enter Playing for playing seek proof");

    let result = manager.seek(500);
    match result {
        Err(PlaybackError::InvalidControlState(message)) => {
            assert_eq!(
                message, "seek while playing requires output flush contract",
                "Playing error message must match exact contract"
            );
        }
        other => panic!("expected InvalidControlState for playing seek, got {other:?}"),
    }

    let _ = std::fs::remove_file(&path);
}
