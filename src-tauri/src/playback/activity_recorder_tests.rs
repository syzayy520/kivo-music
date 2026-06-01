use super::activity_log::PlaybackActivityLogState;
use super::activity_recorder::{record_state_result, record_track_result};
use super::errors::{PlaybackError, PlaybackResult};
use super::events::PlaybackEvent;
use super::state::PlaybackState;

fn state_with_error_marker(marker: &str) -> PlaybackState {
    PlaybackState {
        error: Some(marker.to_string()),
        ..PlaybackState::default()
    }
}

fn assert_state_changed_event(event: &PlaybackEvent, expected_marker: &str) {
    match event {
        PlaybackEvent::StateChanged(state) => {
            assert_eq!(state.error.as_deref(), Some(expected_marker));
        }
        other => panic!("expected state changed event, got {other:?}"),
    }
}

fn assert_track_changed_event(event: &PlaybackEvent, expected_marker: &str) {
    match event {
        PlaybackEvent::TrackChanged(state) => {
            assert_eq!(state.error.as_deref(), Some(expected_marker));
        }
        other => panic!("expected track changed event, got {other:?}"),
    }
}

fn assert_playback_error_event(event: &PlaybackEvent, expected_message: &str) {
    match event {
        PlaybackEvent::Error(PlaybackError::Playback(message)) => {
            assert_eq!(message, expected_message);
        }
        other => panic!("expected playback error event, got {other:?}"),
    }
}

#[test]
fn record_state_result_appends_state_changed_payload() {
    let activity = PlaybackActivityLogState::default();
    let result: PlaybackResult<PlaybackState> = Ok(state_with_error_marker("state-ok"));

    record_state_result(&activity, &result);

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 1);
    assert_state_changed_event(&snapshot.entries[0].event, "state-ok");
    assert!(snapshot.entries[0].timestamp_ms > 0);
}

#[test]
fn record_track_result_appends_track_changed_payload() {
    let activity = PlaybackActivityLogState::default();
    let result: PlaybackResult<PlaybackState> = Ok(state_with_error_marker("track-ok"));

    record_track_result(&activity, &result);

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 1);
    assert_track_changed_event(&snapshot.entries[0].event, "track-ok");
    assert!(snapshot.entries[0].timestamp_ms > 0);
}

#[test]
fn record_state_result_appends_error_payload_on_failure() {
    let activity = PlaybackActivityLogState::default();
    let result: PlaybackResult<PlaybackState> = Err(PlaybackError::Playback("state failed".to_string()));

    record_state_result(&activity, &result);

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 1);
    assert_playback_error_event(&snapshot.entries[0].event, "state failed");
}

#[test]
fn record_track_result_appends_error_payload_on_failure() {
    let activity = PlaybackActivityLogState::default();
    let result: PlaybackResult<PlaybackState> = Err(PlaybackError::Playback("track failed".to_string()));

    record_track_result(&activity, &result);

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 1);
    assert_playback_error_event(&snapshot.entries[0].event, "track failed");
}

#[test]
fn recorder_appends_multiple_results_in_call_order() {
    let activity = PlaybackActivityLogState::default();
    let state_result: PlaybackResult<PlaybackState> = Ok(state_with_error_marker("state-ok"));
    let track_result: PlaybackResult<PlaybackState> = Ok(state_with_error_marker("track-ok"));
    let error_result: PlaybackResult<PlaybackState> = Err(PlaybackError::Playback("failed".to_string()));

    record_state_result(&activity, &state_result);
    record_track_result(&activity, &track_result);
    record_state_result(&activity, &error_result);

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 3);
    assert_state_changed_event(&snapshot.entries[0].event, "state-ok");
    assert_track_changed_event(&snapshot.entries[1].event, "track-ok");
    assert_playback_error_event(&snapshot.entries[2].event, "failed");
}
