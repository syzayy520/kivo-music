use super::errors::PlaybackError;
use super::playback_event_bus::PlaybackEventBus;
use super::state::PlaybackState;

#[test]
fn flush_returns_events_and_writes_activity_log() {
    let mut bus = PlaybackEventBus::default();

    bus.emit_state_changed(PlaybackState::default());
    bus.emit_error(PlaybackError::Playback("failed".to_string()));

    let emitted = bus.flush_to_activity();
    assert_eq!(emitted.len(), 2);

    let snapshot = bus.activity_snapshot();
    assert_eq!(snapshot.entry_count, 2);
}

#[test]
fn progress_throttle_is_kept_when_flushing() {
    let mut bus = PlaybackEventBus::default();

    bus.emit_progress_at(100, Some(1_000), 1_000);
    bus.emit_progress_at(200, Some(1_000), 1_100);
    bus.emit_progress_at(300, Some(1_000), 1_500);

    let emitted = bus.flush_to_activity();
    assert_eq!(emitted.len(), 2);

    let snapshot = bus.activity_snapshot();
    assert_eq!(snapshot.entry_count, 2);
}
