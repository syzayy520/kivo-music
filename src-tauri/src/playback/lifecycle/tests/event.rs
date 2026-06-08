use crate::playback::lifecycle_event::{PlaybackLifecycleEvent, PlaybackLifecycleEventKind};

#[test]
fn initialized_event_has_timestamp_and_backend_name() {
    let event = PlaybackLifecycleEvent::initialized("kivo-core-audio");

    assert!(matches!(
        event.kind,
        PlaybackLifecycleEventKind::Initialized
    ));
    assert_eq!(event.backend_name, "kivo-core-audio");
    assert_eq!(event.message, None);
    assert!(event.timestamp_ms > 0);
}

#[test]
fn shutdown_failed_event_keeps_message_and_timestamp() {
    let event = PlaybackLifecycleEvent::shutdown_failed("kivo-core-audio", "shutdown failed");

    assert!(matches!(
        event.kind,
        PlaybackLifecycleEventKind::ShutdownFailed
    ));
    assert_eq!(event.backend_name, "kivo-core-audio");
    assert_eq!(event.message.as_deref(), Some("shutdown failed"));
    assert!(event.timestamp_ms > 0);
}
