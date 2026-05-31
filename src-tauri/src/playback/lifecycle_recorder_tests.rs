use super::lifecycle_activity_log::PlaybackLifecycleActivityLogState;
use super::lifecycle_event::PlaybackLifecycleEventKind;
use super::lifecycle_recorder::{
    record_lifecycle_initialized, record_lifecycle_shutdown_failed,
    record_lifecycle_shutdown_started, record_lifecycle_shutdown_succeeded,
};

#[test]
fn recorder_appends_lifecycle_events_in_order() {
    let activity = PlaybackLifecycleActivityLogState::default();

    record_lifecycle_initialized(&activity, "kivo-core-audio");
    record_lifecycle_shutdown_started(&activity, "kivo-core-audio");
    record_lifecycle_shutdown_succeeded(&activity, "kivo-core-audio");

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 3);
    assert!(matches!(
        snapshot.entries[0].kind,
        PlaybackLifecycleEventKind::Initialized
    ));
    assert!(matches!(
        snapshot.entries[1].kind,
        PlaybackLifecycleEventKind::ShutdownStarted
    ));
    assert!(matches!(
        snapshot.entries[2].kind,
        PlaybackLifecycleEventKind::ShutdownSucceeded
    ));
    assert!(
        snapshot
            .entries
            .iter()
            .all(|entry| entry.backend_name == "kivo-core-audio")
    );
}

#[test]
fn recorder_keeps_shutdown_failed_message() {
    let activity = PlaybackLifecycleActivityLogState::default();

    record_lifecycle_shutdown_failed(&activity, "kivo-core-audio", "native shutdown failed");

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 1);
    assert!(matches!(
        snapshot.entries[0].kind,
        PlaybackLifecycleEventKind::ShutdownFailed
    ));
    assert_eq!(
        snapshot.entries[0].message.as_deref(),
        Some("native shutdown failed")
    );
    assert!(snapshot.entries[0].timestamp_ms > 0);
}
