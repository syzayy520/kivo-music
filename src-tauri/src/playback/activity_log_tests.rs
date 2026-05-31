use super::activity_log::PlaybackActivityLog;
use super::events::PlaybackEvent;

fn progress_event(position_ms: u64) -> PlaybackEvent {
    PlaybackEvent::Progress {
        position_ms,
        duration_ms: Some(1_000),
    }
}

#[test]
fn snapshot_reports_entry_count_and_limit() {
    let mut log = PlaybackActivityLog::new(4);

    log.append(progress_event(100));
    log.append(progress_event(200));

    let snapshot = log.snapshot();

    assert_eq!(snapshot.entry_count, 2);
    assert_eq!(snapshot.limit, 4);
    assert_eq!(snapshot.entries.len(), 2);
    assert!(snapshot.entries.iter().all(|entry| entry.timestamp_ms > 0));
}

#[test]
fn log_keeps_latest_entries_within_limit() {
    let mut log = PlaybackActivityLog::new(2);

    log.append(progress_event(100));
    log.append(progress_event(200));
    log.append(progress_event(300));

    let snapshot = log.snapshot();

    assert_eq!(snapshot.entry_count, 2);
    assert_eq!(snapshot.entries.len(), 2);
}

#[test]
fn zero_limit_drops_new_entries() {
    let mut log = PlaybackActivityLog::new(0);

    log.append(progress_event(100));

    let snapshot = log.snapshot();

    assert_eq!(snapshot.entry_count, 0);
    assert_eq!(snapshot.limit, 0);
    assert!(snapshot.entries.is_empty());
}
