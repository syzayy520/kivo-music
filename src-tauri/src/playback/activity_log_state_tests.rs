use std::sync::Arc;
use std::thread;

use super::activity_log::PlaybackActivityLogState;
use super::events::PlaybackEvent;

fn progress_event(position_ms: u64) -> PlaybackEvent {
    PlaybackEvent::Progress {
        position_ms,
        duration_ms: Some(1_000),
    }
}

#[test]
fn state_append_is_visible_in_snapshot() {
    let activity = PlaybackActivityLogState::default();

    activity.append(progress_event(250));

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 1);
    assert!(snapshot.limit > 0);
    assert!(matches!(
        &snapshot.entries[0].event,
        PlaybackEvent::Progress {
            position_ms: 250,
            duration_ms: Some(1_000),
        }
    ));
    assert!(snapshot.entries[0].timestamp_ms > 0);
}

#[test]
fn state_clear_removes_entries() {
    let activity = PlaybackActivityLogState::default();

    activity.append(progress_event(250));
    activity.clear();

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 0);
    assert!(snapshot.entries.is_empty());
}

#[test]
fn state_snapshot_returns_empty_on_lock_poison() {
    let activity = Arc::new(PlaybackActivityLogState::default());
    let activity_clone = Arc::clone(&activity);

    let handle = thread::spawn(move || {
        activity_clone.poison_for_test();
    });

    let _ = handle.join();

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 0);
    assert!(snapshot.entries.is_empty());
    assert_eq!(snapshot.limit, 0);
}
