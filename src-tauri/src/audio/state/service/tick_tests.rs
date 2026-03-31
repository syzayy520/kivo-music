use crossbeam_channel::bounded;

use crate::audio::errors::AudioError;

use super::{
    lock_poison::lock_poison,
    service::AudioService,
    test_support::{insert_session, make_status},
    types::AudioSessionState,
};

#[test]
fn tick_without_active_session_returns_invalid_input() {
    let svc = AudioService::default();
    let err = svc
        .tick()
        .expect_err("tick should fail without active session");
    assert!(matches!(err, AudioError::InvalidInput(_)));
}

#[test]
fn tick_after_queue_set_does_not_use_stale_active_session() {
    let svc = AudioService::default();
    let (cmd_tx, _cmd_rx) = bounded(1);
    insert_session(&svc, 40, cmd_tx, make_status(40, "old.mp3"));
    lock_poison(&svc.queue).active_session_id = Some(40);
    svc.queue_set(vec!["new-a.mp3".into(), "new-b.mp3".into()], Some(0), true)
        .expect("queue_set should succeed");

    let err = svc
        .tick()
        .expect_err("tick should fail after queue_set clears stale active session");

    assert!(matches!(err, AudioError::InvalidInput(_)));
}

#[test]
fn tick_clears_stale_active_session_when_status_is_missing() {
    let svc = AudioService::default();
    svc.queue_set(vec!["a.mp3".into(), "b.mp3".into()], Some(0), true)
        .expect("queue_set should succeed");
    lock_poison(&svc.queue).active_session_id = Some(405);

    let err = svc
        .tick()
        .expect_err("tick should fail and clear stale active session");

    assert!(matches!(err, AudioError::InvalidInput(_)));
    assert_eq!(lock_poison(&svc.queue).active_session_id, None);
}

#[test]
fn tick_without_autoplay_returns_current_snapshot_and_status() {
    let svc = AudioService::default();
    let (cmd_tx, _cmd_rx) = bounded(1);
    let mut status = make_status(41, "a.mp3");
    status.state = AudioSessionState::Stopped;
    status.ended = true;
    insert_session(&svc, 41, cmd_tx, status);
    svc.queue_set(vec!["a.mp3".into(), "b.mp3".into()], Some(0), false)
        .expect("queue_set should succeed");
    lock_poison(&svc.queue).active_session_id = Some(41);

    let (snapshot, status) = svc.tick().expect("tick should succeed");

    assert_eq!(snapshot.current_index, Some(0));
    assert_eq!(snapshot.current_path.as_deref(), Some("a.mp3"));
    assert_eq!(status.session_id, 41);
    assert_eq!(status.state, AudioSessionState::Stopped);
}

#[test]
fn tick_with_autoplay_but_not_ended_does_not_advance() {
    let svc = AudioService::default();
    let (cmd_tx, _cmd_rx) = bounded(1);
    let mut status = make_status(45, "a.mp3");
    status.state = AudioSessionState::Playing;
    status.ended = false;
    insert_session(&svc, 45, cmd_tx, status);
    svc.queue_set(vec!["a.mp3".into(), "b.mp3".into()], Some(0), true)
        .expect("queue_set should succeed");
    lock_poison(&svc.queue).active_session_id = Some(45);

    let (snapshot, status) = svc.tick().expect("tick should succeed");

    assert_eq!(snapshot.current_index, Some(0));
    assert_eq!(snapshot.current_path.as_deref(), Some("a.mp3"));
    assert_eq!(status.session_id, 45);
    assert_eq!(status.state, AudioSessionState::Playing);
}

#[test]
fn tick_with_autoplay_at_queue_end_does_not_advance() {
    let svc = AudioService::default();
    let (cmd_tx, _cmd_rx) = bounded(1);
    let mut status = make_status(51, "b.mp3");
    status.state = AudioSessionState::Stopped;
    status.ended = true;
    insert_session(&svc, 51, cmd_tx, status);
    svc.queue_set(vec!["a.mp3".into(), "b.mp3".into()], Some(1), true)
        .expect("queue_set should succeed");
    lock_poison(&svc.queue).active_session_id = Some(51);

    let (snapshot, status) = svc.tick().expect("tick should succeed");

    assert_eq!(snapshot.current_index, Some(1));
    assert_eq!(snapshot.current_path.as_deref(), Some("b.mp3"));
    assert_eq!(status.session_id, 51);
}

#[test]
fn tick_with_autoplay_propagates_start_failure_and_sets_last_error() {
    let svc = AudioService::default();
    let (cmd_tx, _cmd_rx) = bounded(1);
    let mut status = make_status(61, "missing-a.mp3");
    status.state = AudioSessionState::Stopped;
    status.ended = true;
    let status = insert_session(&svc, 61, cmd_tx, status);
    svc.queue_set(
        vec!["missing-a.mp3".into(), "missing-b.mp3".into()],
        Some(0),
        true,
    )
    .expect("queue_set should succeed");
    lock_poison(&svc.queue).active_session_id = Some(61);

    let err = svc
        .tick()
        .expect_err("tick autoplay should fail for missing file");
    let last_error = lock_poison(&status).last_error.clone().unwrap_or_default();
    let snapshot = svc.queue_get().expect("queue_get should succeed");
    let active_session_id = lock_poison(&svc.queue).active_session_id;

    assert!(matches!(err, AudioError::InvalidInput(_)));
    assert!(last_error.contains("[AUDIO_INVALID_INPUT]"));
    assert!(last_error.contains("input_path not found"));
    assert_eq!(snapshot.current_index, Some(0));
    assert_eq!(snapshot.current_path.as_deref(), Some("missing-a.mp3"));
    assert_eq!(active_session_id, Some(61));
}
