use crossbeam_channel::bounded;

use crate::audio::errors::AudioError;

use super::{
    lock_poison::lock_poison,
    service::AudioService,
    test_support::{insert_session, make_status},
    types::AudioSessionState,
};

#[test]
fn next_with_autoplay_false_updates_index_and_returns_active_status() {
    let svc = AudioService::default();
    let (cmd_tx, _cmd_rx) = bounded(1);
    insert_session(&svc, 11, cmd_tx, make_status(11, "a.mp3"));
    svc.queue_set(vec!["a.mp3".into(), "b.mp3".into()], Some(0), false)
        .expect("queue_set should succeed");
    lock_poison(&svc.queue).active_session_id = Some(11);

    let status = svc.next().expect("next should succeed");
    let snapshot = svc.queue_get().expect("queue_get should succeed");

    assert_eq!(status.session_id, 11);
    assert_eq!(snapshot.current_index, Some(1));
    assert_eq!(snapshot.current_path.as_deref(), Some("b.mp3"));
}

#[test]
fn next_with_autoplay_false_requires_active_session() {
    let svc = AudioService::default();
    svc.queue_set(vec!["a.mp3".into(), "b.mp3".into()], Some(0), false)
        .expect("queue_set should succeed");

    let err = svc
        .next()
        .expect_err("next should fail without active session");
    assert!(matches!(err, AudioError::InvalidInput(_)));
}

#[test]
fn next_with_autoplay_false_clears_stale_active_session() {
    let svc = AudioService::default();
    svc.queue_set(vec!["a.mp3".into(), "b.mp3".into()], Some(0), false)
        .expect("queue_set should succeed");
    lock_poison(&svc.queue).active_session_id = Some(404);

    let err = svc
        .next()
        .expect_err("next should fail and clear stale active session");

    assert!(matches!(err, AudioError::InvalidInput(_)));
    assert_eq!(lock_poison(&svc.queue).active_session_id, None);
}

#[test]
fn next_with_autoplay_true_writes_last_error_on_start_failure() {
    let svc = AudioService::default();
    let (cmd_tx, _cmd_rx) = bounded(1);
    let status = insert_session(&svc, 21, cmd_tx, make_status(21, "missing-a.mp3"));
    svc.queue_set(
        vec!["missing-a.mp3".into(), "missing-b.mp3".into()],
        Some(0),
        true,
    )
    .expect("queue_set should succeed");
    lock_poison(&svc.queue).active_session_id = Some(21);

    let err = svc.next().expect_err("next should fail for missing file");
    let last_error = lock_poison(&status).last_error.clone().unwrap_or_default();
    let snapshot = svc.queue_get().expect("queue_get should succeed");
    let active_session_id = lock_poison(&svc.queue).active_session_id;

    assert!(matches!(err, AudioError::InvalidInput(_)));
    assert!(last_error.contains("[AUDIO_INVALID_INPUT]"));
    assert!(last_error.contains("input_path not found"));
    assert_eq!(snapshot.current_index, Some(0));
    assert_eq!(snapshot.current_path.as_deref(), Some("missing-a.mp3"));
    assert_eq!(active_session_id, Some(21));
}

#[test]
fn prev_out_of_range_returns_invalid_input() {
    let svc = AudioService::default();
    let (cmd_tx, _cmd_rx) = bounded(1);
    let mut status = make_status(31, "a.mp3");
    status.state = AudioSessionState::Stopped;
    insert_session(&svc, 31, cmd_tx, status);
    svc.queue_set(vec!["a.mp3".into(), "b.mp3".into()], Some(0), false)
        .expect("queue_set should succeed");
    lock_poison(&svc.queue).active_session_id = Some(31);

    let err = svc.prev().expect_err("prev at index 0 should fail");
    assert!(matches!(err, AudioError::InvalidInput(_)));
}
