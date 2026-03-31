use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use crossbeam_channel::bounded;

use crate::audio::errors::AudioError;

use super::{
    service::AudioService,
    test_support::{insert_session, make_status},
    types::{PlayStartOptions, ServiceCmd},
};

#[test]
fn dispatch_cmd_unknown_session_returns_invalid_input() {
    let svc = AudioService::default();
    let err = svc
        .dispatch_cmd(42, "stop", ServiceCmd::Stop, Duration::from_millis(0))
        .expect_err("unknown session should fail");
    assert!(matches!(err, AudioError::InvalidInput(_)));
}

#[test]
fn dispatch_cmd_timeout_returns_unsupported() {
    let svc = AudioService::default();
    let (cmd_tx, _cmd_rx) = bounded::<ServiceCmd>(0);
    insert_session(&svc, 7, cmd_tx, make_status(7, "a.mp3"));

    let err = svc
        .dispatch_cmd(
            7,
            "pause",
            ServiceCmd::Pause(true),
            Duration::from_millis(0),
        )
        .expect_err("full zero-capacity queue should time out");

    let public = err.to_public_string();
    assert!(matches!(err, AudioError::Unsupported(_)));
    assert!(public.contains("command queue busy"));
    assert!(public.contains("session_id=7"));
    assert!(public.contains("action=pause"));
}

#[test]
fn dispatch_cmd_disconnected_returns_invalid_input() {
    let svc = AudioService::default();
    let (cmd_tx, cmd_rx) = bounded::<ServiceCmd>(1);
    drop(cmd_rx);
    insert_session(&svc, 9, cmd_tx, make_status(9, "b.mp3"));

    let err = svc
        .dispatch_cmd(9, "stop", ServiceCmd::Stop, Duration::from_millis(0))
        .expect_err("disconnected queue should fail");
    assert!(matches!(err, AudioError::InvalidInput(_)));
}

#[test]
fn play_start_rejects_empty_input_path() {
    let svc = AudioService::default();
    let err = svc
        .play_start("   ".into(), PlayStartOptions::default())
        .expect_err("empty input path should fail");
    assert!(matches!(err, AudioError::InvalidInput(_)));
}

#[test]
fn play_start_rejects_missing_input_path() {
    let svc = AudioService::default();
    let err = svc
        .play_start(
            "Z:\\definitely-missing\\nope.mp3".into(),
            PlayStartOptions::default(),
        )
        .expect_err("missing input path should fail");
    assert!(matches!(err, AudioError::InvalidInput(_)));
}

#[test]
fn play_start_rejects_invalid_start_seconds() {
    let svc = AudioService::default();
    let temp = TempAudioFile::new("invalid_start_seconds");
    let err = svc
        .play_start(
            temp.path.to_string_lossy().to_string(),
            PlayStartOptions {
                start_seconds: Some(f64::NAN),
                ..PlayStartOptions::default()
            },
        )
        .expect_err("invalid start_seconds should fail");
    assert!(matches!(err, AudioError::InvalidInput(_)));
}

#[test]
fn play_start_rejects_invalid_max_seconds() {
    let svc = AudioService::default();
    let temp = TempAudioFile::new("invalid_max_seconds");
    let err = svc
        .play_start(
            temp.path.to_string_lossy().to_string(),
            PlayStartOptions {
                max_seconds: Some(0.0),
                ..PlayStartOptions::default()
            },
        )
        .expect_err("invalid max_seconds should fail");
    assert!(matches!(err, AudioError::InvalidInput(_)));
}

struct TempAudioFile {
    path: std::path::PathBuf,
}

impl TempAudioFile {
    fn new(label: &str) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "kivo_shell_{label}_{}_{}.tmp",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::write(&path, b"not-a-real-audio-file").expect("temp file should be writable");
        Self { path }
    }
}

impl Drop for TempAudioFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
