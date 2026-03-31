use crossbeam_channel::bounded;

use crate::audio::errors::AudioError;

use super::{
    lock_poison::lock_poison,
    service::AudioService,
    test_support::{insert_session, make_status},
    types::PlayStartOptions,
};

#[test]
fn queue_set_normalizes_items_and_keeps_requested_index() {
    let svc = AudioService::default();
    let snapshot = svc
        .queue_set(
            vec![" a.mp3 ".into(), "".into(), " b.mp3 ".into()],
            Some(1),
            false,
        )
        .expect("queue_set should succeed");

    assert_eq!(snapshot.items, vec!["a.mp3", "b.mp3"]);
    assert_eq!(snapshot.current_index, Some(1));
    assert_eq!(snapshot.current_path.as_deref(), Some("b.mp3"));
    assert!(!snapshot.autoplay);
    assert_eq!(lock_poison(&svc.queue).active_session_id, None);
}

#[test]
fn queue_set_rejects_out_of_range_index() {
    let svc = AudioService::default();
    let err = svc
        .queue_set(vec!["a.mp3".into()], Some(1), false)
        .expect_err("queue_set should reject out-of-range index");

    assert!(matches!(err, AudioError::InvalidInput(_)));
}

#[test]
fn queue_add_normalizes_items_and_sets_first_index_when_empty() {
    let svc = AudioService::default();
    let snapshot = svc
        .queue_add(vec![" ".into(), " a.mp3 ".into(), "b.mp3".into()])
        .expect("queue_add should succeed");

    assert_eq!(snapshot.items, vec!["a.mp3", "b.mp3"]);
    assert_eq!(snapshot.current_index, Some(0));
    assert_eq!(snapshot.current_path.as_deref(), Some("a.mp3"));
}

#[test]
fn queue_add_preserves_existing_current_index() {
    let svc = AudioService::default();
    svc.queue_set(vec!["a.mp3".into(), "b.mp3".into()], Some(1), false)
        .expect("queue_set should succeed");

    let snapshot = svc
        .queue_add(vec!["c.mp3".into(), "d.mp3".into()])
        .expect("queue_add should succeed");

    assert_eq!(snapshot.items, vec!["a.mp3", "b.mp3", "c.mp3", "d.mp3"]);
    assert_eq!(snapshot.current_index, Some(1));
    assert_eq!(snapshot.current_path.as_deref(), Some("b.mp3"));
}

#[test]
fn note_play_started_clears_current_index_when_path_is_not_in_queue() {
    let svc = AudioService::default();
    svc.queue_set(vec!["a.mp3".into(), "b.mp3".into()], Some(1), true)
        .expect("queue_set should succeed");

    {
        let mut q = lock_poison(&svc.queue);
        q.note_play_started(
            91,
            "outside.mp3",
            &PlayStartOptions {
                start_seconds: Some(3.0),
                max_seconds: Some(4.0),
                ffmpeg_dir: Some("ffmpeg".into()),
                device_id: Some("device".into()),
            },
        );
        assert_eq!(q.active_session_id, Some(91));
        assert_eq!(q.current_index, None);
        assert_eq!(q.last_play_opt.start_seconds, Some(3.0));
        assert_eq!(q.last_play_opt.max_seconds, Some(4.0));
    }

    let snapshot = svc.queue_get().expect("queue_get should succeed");
    assert_eq!(snapshot.current_index, None);
    assert_eq!(snapshot.current_path, None);
}

#[test]
fn queue_clear_resets_index_and_current_path() {
    let svc = AudioService::default();
    svc.queue_set(vec!["a.mp3".into(), "b.mp3".into()], Some(1), true)
        .expect("queue_set should succeed");
    {
        let mut q = lock_poison(&svc.queue);
        q.active_session_id = Some(99);
        q.last_play_opt = PlayStartOptions {
            start_seconds: Some(12.0),
            max_seconds: Some(34.0),
            ffmpeg_dir: Some("ffmpeg".into()),
            device_id: Some("device".into()),
        };
    }

    let snapshot = svc.queue_clear().expect("queue_clear should succeed");

    assert!(snapshot.items.is_empty());
    assert_eq!(snapshot.current_index, None);
    assert_eq!(snapshot.current_path, None);
    let q = lock_poison(&svc.queue);
    assert_eq!(q.active_session_id, None);
    assert_eq!(q.last_play_opt.start_seconds, None);
    assert_eq!(q.last_play_opt.max_seconds, None);
    assert_eq!(q.last_play_opt.ffmpeg_dir, None);
    assert_eq!(q.last_play_opt.device_id, None);
}

#[test]
fn queue_set_clears_stale_active_session() {
    let svc = AudioService::default();
    let (cmd_tx, _cmd_rx) = bounded(1);
    insert_session(&svc, 81, cmd_tx, make_status(81, "old.mp3"));
    {
        let mut q = lock_poison(&svc.queue);
        q.active_session_id = Some(81);
        q.last_play_opt = PlayStartOptions {
            start_seconds: Some(56.0),
            max_seconds: Some(78.0),
            ffmpeg_dir: Some("old-ffmpeg".into()),
            device_id: Some("old-device".into()),
        };
    }

    let snapshot = svc
        .queue_set(vec!["new-a.mp3".into(), "new-b.mp3".into()], Some(0), true)
        .expect("queue_set should succeed");

    assert_eq!(snapshot.current_path.as_deref(), Some("new-a.mp3"));
    let q = lock_poison(&svc.queue);
    assert_eq!(q.active_session_id, None);
    assert_eq!(q.last_play_opt.start_seconds, None);
    assert_eq!(q.last_play_opt.max_seconds, None);
    assert_eq!(q.last_play_opt.ffmpeg_dir, None);
    assert_eq!(q.last_play_opt.device_id, None);
}
