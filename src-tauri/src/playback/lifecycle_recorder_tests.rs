use super::lifecycle_activity_log::PlaybackLifecycleActivityLogState;
use super::lifecycle_event::{PlaybackLifecycleEvent, PlaybackLifecycleEventKind};
use super::lifecycle_recorder::{
    record_lifecycle_initialized, record_lifecycle_shutdown_failed,
    record_lifecycle_shutdown_started, record_lifecycle_shutdown_succeeded,
};

fn assert_lifecycle_event(
    event: &PlaybackLifecycleEvent,
    expected_kind: PlaybackLifecycleEventKind,
    expected_backend_name: &str,
    expected_message: Option<&str>,
) {
    assert!(matches!(
        (&event.kind, expected_kind),
        (
            PlaybackLifecycleEventKind::Initialized,
            PlaybackLifecycleEventKind::Initialized
        ) | (
            PlaybackLifecycleEventKind::ShutdownStarted,
            PlaybackLifecycleEventKind::ShutdownStarted
        ) | (
            PlaybackLifecycleEventKind::ShutdownSucceeded,
            PlaybackLifecycleEventKind::ShutdownSucceeded
        ) | (
            PlaybackLifecycleEventKind::ShutdownFailed,
            PlaybackLifecycleEventKind::ShutdownFailed
        )
    ));
    assert_eq!(event.backend_name, expected_backend_name);
    assert_eq!(event.message.as_deref(), expected_message);
    assert!(event.timestamp_ms > 0);
}

#[test]
fn recorder_appends_lifecycle_events_in_order() {
    let activity = PlaybackLifecycleActivityLogState::default();

    record_lifecycle_initialized(&activity, "kivo-core-audio");
    record_lifecycle_shutdown_started(&activity, "kivo-core-audio");
    record_lifecycle_shutdown_succeeded(&activity, "kivo-core-audio");

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 3);
    assert_lifecycle_event(
        &snapshot.entries[0],
        PlaybackLifecycleEventKind::Initialized,
        "kivo-core-audio",
        None,
    );
    assert_lifecycle_event(
        &snapshot.entries[1],
        PlaybackLifecycleEventKind::ShutdownStarted,
        "kivo-core-audio",
        None,
    );
    assert_lifecycle_event(
        &snapshot.entries[2],
        PlaybackLifecycleEventKind::ShutdownSucceeded,
        "kivo-core-audio",
        None,
    );
}

#[test]
fn recorder_keeps_shutdown_failed_payload() {
    let activity = PlaybackLifecycleActivityLogState::default();

    record_lifecycle_shutdown_failed(&activity, "kivo-core-audio", "native shutdown failed");

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 1);
    assert_lifecycle_event(
        &snapshot.entries[0],
        PlaybackLifecycleEventKind::ShutdownFailed,
        "kivo-core-audio",
        Some("native shutdown failed"),
    );
}

#[test]
fn recorder_keeps_backend_names_per_event() {
    let activity = PlaybackLifecycleActivityLogState::default();

    record_lifecycle_initialized(&activity, "native");
    record_lifecycle_shutdown_started(&activity, "wasapi");
    record_lifecycle_shutdown_failed(&activity, "fallback", "unsupported output");

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 3);
    assert_lifecycle_event(
        &snapshot.entries[0],
        PlaybackLifecycleEventKind::Initialized,
        "native",
        None,
    );
    assert_lifecycle_event(
        &snapshot.entries[1],
        PlaybackLifecycleEventKind::ShutdownStarted,
        "wasapi",
        None,
    );
    assert_lifecycle_event(
        &snapshot.entries[2],
        PlaybackLifecycleEventKind::ShutdownFailed,
        "fallback",
        Some("unsupported output"),
    );
}
