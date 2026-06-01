use super::lifecycle_event::{PlaybackLifecycleEvent, PlaybackLifecycleEventKind};

fn assert_lifecycle_event(
    event: PlaybackLifecycleEvent,
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
fn initialized_event_has_timestamp_and_backend_name() {
    let event = PlaybackLifecycleEvent::initialized("kivo-core-audio");

    assert_lifecycle_event(
        event,
        PlaybackLifecycleEventKind::Initialized,
        "kivo-core-audio",
        None,
    );
}

#[test]
fn shutdown_started_event_has_backend_name_without_message() {
    let event = PlaybackLifecycleEvent::shutdown_started("kivo-core-audio");

    assert_lifecycle_event(
        event,
        PlaybackLifecycleEventKind::ShutdownStarted,
        "kivo-core-audio",
        None,
    );
}

#[test]
fn shutdown_succeeded_event_has_backend_name_without_message() {
    let event = PlaybackLifecycleEvent::shutdown_succeeded("kivo-core-audio");

    assert_lifecycle_event(
        event,
        PlaybackLifecycleEventKind::ShutdownSucceeded,
        "kivo-core-audio",
        None,
    );
}

#[test]
fn shutdown_failed_event_keeps_message_and_timestamp() {
    let event = PlaybackLifecycleEvent::shutdown_failed("kivo-core-audio", "shutdown failed");

    assert_lifecycle_event(
        event,
        PlaybackLifecycleEventKind::ShutdownFailed,
        "kivo-core-audio",
        Some("shutdown failed"),
    );
}
