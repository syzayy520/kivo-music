use super::activity_log::PlaybackActivityLogState;
use super::errors::PlaybackError;
use super::events::PlaybackEvent;
use super::playback_event_bridge::PlaybackEventBridge;
use super::state::PlaybackState;

#[test]
fn state_result_records_state_changed_event() {
    let mut bridge = PlaybackEventBridge::default();
    let activity = PlaybackActivityLogState::default();
    let result = Ok(PlaybackState::default());

    bridge.record_state_result(&activity, &result);

    let snapshot = activity.snapshot();
    assert_eq!(snapshot.entry_count, 1);
    assert!(matches!(snapshot.entries[0].event, PlaybackEvent::StateChanged(_)));
}

#[test]
fn track_result_records_error_event() {
    let mut bridge = PlaybackEventBridge::default();
    let activity = PlaybackActivityLogState::default();
    let result = Err(PlaybackError::Playback("failed".to_string()));

    bridge.record_track_result(&activity, &result);

    let snapshot = activity.snapshot();
    assert_eq!(snapshot.entry_count, 1);
    assert!(matches!(snapshot.entries[0].event, PlaybackEvent::Error(_)));
}

#[test]
fn progress_from_state_is_throttled() {
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
}
