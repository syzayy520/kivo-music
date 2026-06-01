use std::sync::Arc;
use std::thread;

use super::lifecycle_activity_log::{
    PlaybackLifecycleActivityLog, PlaybackLifecycleActivityLogState,
};
use super::lifecycle_event::PlaybackLifecycleEvent;

fn initialized_event(backend_name: &str) -> PlaybackLifecycleEvent {
    PlaybackLifecycleEvent::initialized(backend_name)
}

#[test]
fn snapshot_reports_entry_count_and_limit() {
    let mut log = PlaybackLifecycleActivityLog::new(4);

    log.append(initialized_event("native"));
    log.append(PlaybackLifecycleEvent::shutdown_started("native"));

    let snapshot = log.snapshot();

    assert_eq!(snapshot.entry_count, 2);
    assert_eq!(snapshot.limit, 4);
    assert_eq!(snapshot.entries.len(), 2);
    assert!(snapshot.entries.iter().all(|entry| entry.timestamp_ms > 0));
}

#[test]
fn log_keeps_latest_entries_within_limit() {
    let mut log = PlaybackLifecycleActivityLog::new(2);

    log.append(initialized_event("first"));
    log.append(initialized_event("second"));
    log.append(initialized_event("third"));

    let snapshot = log.snapshot();

    assert_eq!(snapshot.entry_count, 2);
    assert_eq!(snapshot.entries[0].backend_name, "second");
    assert_eq!(snapshot.entries[1].backend_name, "third");
}

#[test]
fn zero_limit_is_normalized_to_one() {
    let mut log = PlaybackLifecycleActivityLog::new(0);

    log.append(initialized_event("first"));
    log.append(initialized_event("second"));

    let snapshot = log.snapshot();

    assert_eq!(snapshot.entry_count, 1);
    assert_eq!(snapshot.limit, 1);
    assert_eq!(snapshot.entries[0].backend_name, "second");
}

#[test]
fn clear_after_append_returns_empty_snapshot_with_limit() {
    let mut log = PlaybackLifecycleActivityLog::new(4);

    log.append(initialized_event("native"));
    log.append(PlaybackLifecycleEvent::shutdown_started("native"));
    log.clear();

    let snapshot = log.snapshot();

    assert_eq!(snapshot.entry_count, 0);
    assert!(snapshot.entries.is_empty());
    assert_eq!(snapshot.limit, 4);
}

#[test]
fn append_maintains_insertion_order() {
    let mut log = PlaybackLifecycleActivityLog::new(4);

    log.append(initialized_event("first"));
    log.append(initialized_event("second"));
    log.append(initialized_event("third"));

    let snapshot = log.snapshot();

    assert_eq!(snapshot.entry_count, 3);
    assert_eq!(snapshot.entries[0].backend_name, "first");
    assert_eq!(snapshot.entries[1].backend_name, "second");
    assert_eq!(snapshot.entries[2].backend_name, "third");
}

#[test]
fn state_snapshot_returns_empty_on_lock_poison() {
    let activity = Arc::new(PlaybackLifecycleActivityLogState::default());
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
