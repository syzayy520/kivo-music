use crate::playback::activity_log::PlaybackActivityLogState;
use crate::playback::events::PlaybackEvent;

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
