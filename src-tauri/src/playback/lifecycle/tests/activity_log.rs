use crate::playback::lifecycle_activity_log::PlaybackLifecycleActivityLog;
use crate::playback::lifecycle_event::PlaybackLifecycleEvent;

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
