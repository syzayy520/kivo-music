use crate::playback::errors::PlaybackError;
use crate::playback::manager::PlaybackManager;
use crate::playback::types::{PlaybackTrack, RepeatMode, TrackId};

fn track(index: usize) -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId(format!("track-{index}")),
        title: format!("Track {index}"),
        artist: "Artist".to_string(),
        source_path: format!("C:/Music/track-{index}.flac"),
    }
}

#[test]
fn queue_append_sets_first_current_index() {
    let mut manager = PlaybackManager::new();

    let queue = manager.queue_append(track(1));

    assert_eq!(queue.items.len(), 1);
    assert_eq!(queue.current_index, Some(0));
}

#[test]
fn queue_remove_updates_current_index() {
    let mut manager = PlaybackManager::new();
    manager.queue_append(track(1));
    manager.queue_append(track(2));
    manager.queue_append(track(3));

    let _ = manager.queue_set_current(2);
    let queue = manager
        .queue_remove(1)
        .expect("queue remove should succeed");

    assert_eq!(queue.items.len(), 2);
    assert_eq!(queue.current_index, Some(1));
}

#[test]
fn queue_next_moves_to_next_track() {
    let mut manager = PlaybackManager::new();
    manager.queue_append(track(1));
    manager.queue_append(track(2));

    let result = manager.queue_next();

    match result {
        Err(PlaybackError::UnsupportedOperation(_)) => {}
        other => panic!("expected unsupported operation from native load, got {other:?}"),
    }

    assert_eq!(manager.queue().current_index, Some(1));
}

#[test]
fn queue_previous_from_start_stays_on_first_track() {
    let mut manager = PlaybackManager::new();
    manager.queue_append(track(1));
    manager.queue_append(track(2));

    let result = manager.queue_previous();

    match result {
        Err(PlaybackError::UnsupportedOperation(_)) => {}
        other => panic!("expected unsupported operation from native load, got {other:?}"),
    }

    assert_eq!(manager.queue().current_index, Some(0));
}

#[test]
fn queue_set_repeat_mode_updates_queue_state() {
    let mut manager = PlaybackManager::new();

    let queue = manager.queue_set_repeat_mode(RepeatMode::All);

    assert!(matches!(queue.repeat_mode, RepeatMode::All));
}
