use super::errors::PlaybackError;
use super::events::PlaybackEvent;
use super::playback_event_bus::PlaybackEventBus;
use super::state::PlaybackState;

fn state_with_error_marker(marker: &str) -> PlaybackState {
    PlaybackState {
        error: Some(marker.to_string()),
        ..PlaybackState::default()
    }
}

fn assert_progress_event(event: &PlaybackEvent, expected_position_ms: u64) {
    match event {
        PlaybackEvent::Progress {
            position_ms,
            duration_ms,
        } => {
            assert_eq!(*position_ms, expected_position_ms);
            assert_eq!(*duration_ms, Some(1_000));
        }
        other => panic!("expected progress event, got {other:?}"),
    }
}

#[test]
fn flush_returns_events_and_writes_activity_log_in_order() {
    let mut bus = PlaybackEventBus::default();

    bus.emit_state_changed(state_with_error_marker("state-changed"));
    bus.emit_track_changed(state_with_error_marker("track-changed"));
    bus.emit_error(PlaybackError::Playback("failed".to_string()));

    let emitted = bus.flush_to_activity();

    assert_eq!(emitted.len(), 3);
    match &emitted[0] {
        PlaybackEvent::StateChanged(state) => {
            assert_eq!(state.error.as_deref(), Some("state-changed"));
        }
        other => panic!("expected state changed event, got {other:?}"),
    }
    match &emitted[1] {
        PlaybackEvent::TrackChanged(state) => {
            assert_eq!(state.error.as_deref(), Some("track-changed"));
        }
        other => panic!("expected track changed event, got {other:?}"),
    }
    match &emitted[2] {
        PlaybackEvent::Error(PlaybackError::Playback(message)) => {
            assert_eq!(message, "failed");
        }
        other => panic!("expected playback error event, got {other:?}"),
    }

    let snapshot = bus.activity_snapshot();
    assert_eq!(snapshot.entry_count, 3);
    assert!(matches!(
        &snapshot.entries[0].event,
        PlaybackEvent::StateChanged(_)
    ));
    assert!(matches!(
        &snapshot.entries[1].event,
        PlaybackEvent::TrackChanged(_)
    ));
    assert!(matches!(
        &snapshot.entries[2].event,
        PlaybackEvent::Error(PlaybackError::Playback(_))
    ));
}

#[test]
fn progress_throttle_is_kept_when_flushing() {
    let mut bus = PlaybackEventBus::default();

    bus.emit_progress_at(100, Some(1_000), 1_000);
    bus.emit_progress_at(200, Some(1_000), 1_100);
    bus.emit_progress_at(300, Some(1_000), 1_500);

    let emitted = bus.flush_to_activity();

    assert_eq!(emitted.len(), 2);
    assert_progress_event(&emitted[0], 100);
    assert_progress_event(&emitted[1], 300);

    let snapshot = bus.activity_snapshot();
    assert_eq!(snapshot.entry_count, 2);
    assert_progress_event(&snapshot.entries[0].event, 100);
    assert_progress_event(&snapshot.entries[1].event, 300);
}

#[test]
fn drain_events_returns_pending_events_without_writing_activity_log() {
    let mut bus = PlaybackEventBus::default();

    bus.emit_state_changed(state_with_error_marker("state-only"));

    let drained = bus.drain_events();

    assert_eq!(drained.len(), 1);
    assert!(matches!(&drained[0], PlaybackEvent::StateChanged(_)));
    assert!(bus.drain_events().is_empty());
    assert_eq!(bus.activity_snapshot().entry_count, 0);
}

#[test]
fn state_result_maps_success_to_state_changed() {
    let mut bus = PlaybackEventBus::default();
    let result = Ok(PlaybackState::default());

    bus.emit_state_result(&result);
    let emitted = bus.flush_to_activity();

    assert_eq!(emitted.len(), 1);
    assert!(matches!(emitted[0], PlaybackEvent::StateChanged(_)));
}

#[test]
fn state_result_maps_error_to_error_event() {
    let mut bus = PlaybackEventBus::default();
    let result = Err(PlaybackError::Playback("state failed".to_string()));

    bus.emit_state_result(&result);
    let emitted = bus.flush_to_activity();

    assert_eq!(emitted.len(), 1);
    assert!(matches!(emitted[0], PlaybackEvent::Error(_)));
}

#[test]
fn track_result_maps_error_to_error_event() {
    let mut bus = PlaybackEventBus::default();
    let result = Err(PlaybackError::Playback("track failed".to_string()));

    bus.emit_track_result(&result);
    let emitted = bus.flush_to_activity();

    assert_eq!(emitted.len(), 1);
    assert!(matches!(emitted[0], PlaybackEvent::Error(_)));
}

#[test]
fn track_result_maps_success_to_track_changed() {
    let mut bus = PlaybackEventBus::default();
    let result = Ok(PlaybackState::default());

    bus.emit_track_result(&result);
    let emitted = bus.flush_to_activity();

    assert_eq!(emitted.len(), 1);
    assert!(matches!(emitted[0], PlaybackEvent::TrackChanged(_)));
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
        PlaybackEvent::Progress {
            position_ms: 640,
            duration_ms: Some(2_400)
        }
    ));
}
