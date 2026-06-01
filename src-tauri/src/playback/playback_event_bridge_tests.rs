use super::activity_log::PlaybackActivityLogState;
use super::errors::PlaybackError;
use super::events::PlaybackEvent;
use super::playback_event_bridge::PlaybackEventBridge;
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
fn state_result_records_state_changed_payload() {
    let mut bridge = PlaybackEventBridge::default();
    let activity = PlaybackActivityLogState::default();
    let result = Ok(state_with_error_marker("state-ok"));

    bridge.record_state_result(&activity, &result);

    let snapshot = activity.snapshot();
    assert_eq!(snapshot.entry_count, 1);
    match &snapshot.entries[0].event {
        PlaybackEvent::StateChanged(state) => {
            assert_eq!(state.error.as_deref(), Some("state-ok"));
        }
        other => panic!("expected state changed event, got {other:?}"),
    }
}

#[test]
fn state_result_records_error_payload() {
    let mut bridge = PlaybackEventBridge::default();
    let activity = PlaybackActivityLogState::default();
    let result = Err(PlaybackError::Playback("state failed".to_string()));

    bridge.record_state_result(&activity, &result);

    let snapshot = activity.snapshot();
    assert_eq!(snapshot.entry_count, 1);
    match &snapshot.entries[0].event {
        PlaybackEvent::Error(PlaybackError::Playback(message)) => {
            assert_eq!(message, "state failed");
        }
        other => panic!("expected playback error event, got {other:?}"),
    }
}

#[test]
fn track_result_records_error_payload() {
    let mut bridge = PlaybackEventBridge::default();
    let activity = PlaybackActivityLogState::default();
    let result = Err(PlaybackError::Playback("track failed".to_string()));

    bridge.record_track_result(&activity, &result);

    let snapshot = activity.snapshot();
    assert_eq!(snapshot.entry_count, 1);
    match &snapshot.entries[0].event {
        PlaybackEvent::Error(PlaybackError::Playback(message)) => {
            assert_eq!(message, "track failed");
        }
        other => panic!("expected playback error event, got {other:?}"),
    }
}

#[test]
fn track_result_records_track_changed_payload() {
    let mut bridge = PlaybackEventBridge::default();
    let activity = PlaybackActivityLogState::default();
    let result = Ok(state_with_error_marker("track-ok"));

    bridge.record_track_result(&activity, &result);

    let snapshot = activity.snapshot();
    assert_eq!(snapshot.entry_count, 1);
    match &snapshot.entries[0].event {
        PlaybackEvent::TrackChanged(state) => {
            assert_eq!(state.error.as_deref(), Some("track-ok"));
        }
        other => panic!("expected track changed event, got {other:?}"),
    }
}

#[test]
fn bridge_records_multiple_calls_in_order() {
    let mut bridge = PlaybackEventBridge::default();
    let activity = PlaybackActivityLogState::default();
    let state_result = Ok(state_with_error_marker("state-ok"));
    let track_result = Ok(state_with_error_marker("track-ok"));
    let error_result = Err(PlaybackError::Playback("state failed".to_string()));

    bridge.record_state_result(&activity, &state_result);
    bridge.record_track_result(&activity, &track_result);
    bridge.record_state_result(&activity, &error_result);

    let snapshot = activity.snapshot();

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
fn progress_from_state_is_throttled_and_preserves_payload() {
    let mut bridge = PlaybackEventBridge::default();
    let activity = PlaybackActivityLogState::default();
    let mut state = PlaybackState::default();
    state.timeline.position_ms = 100;
    state.timeline.duration_ms = Some(1_000);

    bridge.record_progress_from_state(&activity, &state, 1_000);
    state.timeline.position_ms = 200;
    bridge.record_progress_from_state(&activity, &state, 1_200);
    state.timeline.position_ms = 300;
    bridge.record_progress_from_state(&activity, &state, 1_500);

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 2);
    assert_progress_event(&snapshot.entries[0].event, 100);
    assert_progress_event(&snapshot.entries[1].event, 300);
}
