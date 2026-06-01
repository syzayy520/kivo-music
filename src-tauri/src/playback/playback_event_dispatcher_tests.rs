use super::errors::PlaybackError;
use super::events::PlaybackEvent;
use super::playback_event_dispatcher::PlaybackEventDispatcher;
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
fn immediate_events_preserve_type_payload_and_order() {
    let mut dispatcher = PlaybackEventDispatcher::new(500);
    let state_changed = state_with_error_marker("state-changed");
    let track_changed = state_with_error_marker("track-changed");

    dispatcher.emit_state_changed(state_changed);
    dispatcher.emit_track_changed(track_changed);
    dispatcher.emit_error(PlaybackError::Backend("backend failed".to_string()));

    let events = dispatcher.drain();

    assert_eq!(events.len(), 3);
    match &events[0] {
        PlaybackEvent::StateChanged(state) => {
            assert_eq!(state.error.as_deref(), Some("state-changed"));
        }
        other => panic!("expected state changed event, got {other:?}"),
    }
    match &events[1] {
        PlaybackEvent::TrackChanged(state) => {
            assert_eq!(state.error.as_deref(), Some("track-changed"));
        }
        other => panic!("expected track changed event, got {other:?}"),
    }
    match &events[2] {
        PlaybackEvent::Error(PlaybackError::Backend(message)) => {
            assert_eq!(message, "backend failed");
        }
        other => panic!("expected backend error event, got {other:?}"),
    }
}

#[test]
fn progress_events_are_throttled_by_interval() {
    let mut dispatcher = PlaybackEventDispatcher::new(500);

    dispatcher.emit_progress_at(100, Some(1_000), 1_000);
    dispatcher.emit_progress_at(200, Some(1_000), 1_200);
    dispatcher.emit_progress_at(300, Some(1_000), 1_500);

    let events = dispatcher.drain();

    assert_eq!(events.len(), 2);
    assert_progress_event(&events[0], 100);
    assert_progress_event(&events[1], 300);
}

#[test]
fn progress_event_emits_at_interval_boundary() {
    let mut dispatcher = PlaybackEventDispatcher::new(500);

    dispatcher.emit_progress_at(100, Some(1_000), 1_000);
    dispatcher.emit_progress_at(200, Some(1_000), 1_499);
    dispatcher.emit_progress_at(300, Some(1_000), 1_500);

    let events = dispatcher.drain();

    assert_eq!(events.len(), 2);
    assert_progress_event(&events[0], 100);
    assert_progress_event(&events[1], 300);
}

#[test]
fn drain_clears_pending_events() {
    let mut dispatcher = PlaybackEventDispatcher::default();

    dispatcher.emit_state_changed(PlaybackState::default());
    assert_eq!(dispatcher.drain().len(), 1);
    assert!(dispatcher.drain().is_empty());
}
