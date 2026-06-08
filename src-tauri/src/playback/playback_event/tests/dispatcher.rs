use crate::playback::errors::PlaybackError;
use crate::playback::playback_event_dispatcher::PlaybackEventDispatcher;
use crate::playback::state::PlaybackState;

#[test]
fn state_track_and_error_events_are_immediate() {
    let mut dispatcher = PlaybackEventDispatcher::new(500);
    let state = PlaybackState::default();

    dispatcher.emit_state_changed(state.clone());
    dispatcher.emit_track_changed(state);
    dispatcher.emit_error(PlaybackError::Backend("test".to_string()));

    let events = dispatcher.drain();
    assert_eq!(events.len(), 3);
}

#[test]
fn progress_events_are_throttled_by_interval() {
    let mut dispatcher = PlaybackEventDispatcher::new(500);

    dispatcher.emit_progress_at(100, Some(1_000), 1_000);
    dispatcher.emit_progress_at(200, Some(1_000), 1_200);
    dispatcher.emit_progress_at(300, Some(1_000), 1_500);

    let events = dispatcher.drain();
    assert_eq!(events.len(), 2);
}

#[test]
fn drain_clears_pending_events() {
    let mut dispatcher = PlaybackEventDispatcher::default();

    dispatcher.emit_state_changed(PlaybackState::default());
    assert_eq!(dispatcher.drain().len(), 1);
    assert!(dispatcher.drain().is_empty());
}
