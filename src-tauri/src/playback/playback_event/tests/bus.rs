use crate::playback::errors::PlaybackError;
use crate::playback::playback_event_bus::PlaybackEventBus;
use crate::playback::state::PlaybackState;

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

#[test]
fn state_result_maps_success_to_state_changed() {
    let mut bus = PlaybackEventBus::default();
    let result = Ok(PlaybackState::default());

    bus.emit_state_result(&result);
    let emitted = bus.flush_to_activity();

    assert_eq!(emitted.len(), 1);
    assert!(matches!(
        emitted[0],
        crate::playback::events::PlaybackEvent::StateChanged(_)
    ));
}

#[test]
fn state_result_maps_error_to_error_event() {
    let mut bus = PlaybackEventBus::default();
    let result = Err(PlaybackError::Playback("state failed".to_string()));

    bus.emit_state_result(&result);
    let emitted = bus.flush_to_activity();

    assert_eq!(emitted.len(), 1);
    assert!(matches!(
        emitted[0],
        crate::playback::events::PlaybackEvent::Error(_)
    ));
}

#[test]
fn track_result_maps_error_to_error_event() {
    let mut bus = PlaybackEventBus::default();
    let result = Err(PlaybackError::Playback("track failed".to_string()));

    bus.emit_track_result(&result);
    let emitted = bus.flush_to_activity();

    assert_eq!(emitted.len(), 1);
    assert!(matches!(
        emitted[0],
        crate::playback::events::PlaybackEvent::Error(_)
    ));
}

#[test]
fn track_result_maps_success_to_track_changed() {
    let mut bus = PlaybackEventBus::default();
    let result = Ok(PlaybackState::default());

    bus.emit_track_result(&result);
    let emitted = bus.flush_to_activity();

    assert_eq!(emitted.len(), 1);
    assert!(matches!(
        emitted[0],
        crate::playback::events::PlaybackEvent::TrackChanged(_)
    ));
}

#[test]
fn progress_can_be_emitted_from_state() {
    let mut bus = PlaybackEventBus::default();
    let mut state = PlaybackState::default();
    state.timeline.position_ms = 640;
    state.timeline.duration_ms = Some(2_400);

    bus.emit_progress_from_state(&state, 1_000);
    let emitted = bus.flush_to_activity();

    assert_eq!(emitted.len(), 1);
    assert!(matches!(
        emitted[0],
        crate::playback::events::PlaybackEvent::Progress {
            position_ms: 640,
            duration_ms: Some(2_400)
        }
    ));
}
