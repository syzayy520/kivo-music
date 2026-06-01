use super::activity_log::PlaybackActivityLogState;
use super::command_activity::{run_state_command, run_track_command};
use super::errors::PlaybackError;
use super::events::PlaybackEvent;
use super::manager::PlaybackManagerState;
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
fn run_state_command_returns_and_records_state_changed_payload() {
    let manager = PlaybackManagerState::default();
    let activity = PlaybackActivityLogState::default();

    let result = run_state_command(&manager, &activity, |_| Ok(state_with_error_marker("state-ok")));

    assert_eq!(result.unwrap().error.as_deref(), Some("state-ok"));
    let snapshot = activity.snapshot();
    assert_eq!(snapshot.entry_count, 1);
    assert_state_changed_event(&snapshot.entries[0].event, "state-ok");
}

#[test]
fn run_track_command_returns_and_records_error_payload() {
    let manager = PlaybackManagerState::default();
    let activity = PlaybackActivityLogState::default();

    let result = run_track_command(&manager, &activity, |_| {
        Err(PlaybackError::Playback("track failed".to_string()))
    });

    match &result {
        Err(PlaybackError::Playback(message)) => assert_eq!(message, "track failed"),
        other => panic!("expected track playback error, got {other:?}"),
    }
    let snapshot = activity.snapshot();
    assert_eq!(snapshot.entry_count, 1);
    assert_playback_error_event(&snapshot.entries[0].event, "track failed");
}

#[test]
fn run_state_command_returns_and_records_error_payload() {
    let manager = PlaybackManagerState::default();
    let activity = PlaybackActivityLogState::default();

    let result = run_state_command(&manager, &activity, |_| {
        Err(PlaybackError::Playback("state failed".to_string()))
    });

    match &result {
        Err(PlaybackError::Playback(message)) => assert_eq!(message, "state failed"),
        other => panic!("expected state playback error, got {other:?}"),
    }
    let snapshot = activity.snapshot();
    assert_eq!(snapshot.entry_count, 1);
    assert_playback_error_event(&snapshot.entries[0].event, "state failed");
}

#[test]
fn run_track_command_returns_and_records_track_changed_payload() {
    let manager = PlaybackManagerState::default();
    let activity = PlaybackActivityLogState::default();

    let result = run_track_command(&manager, &activity, |_| Ok(state_with_error_marker("track-ok")));

    assert_eq!(result.unwrap().error.as_deref(), Some("track-ok"));
    let snapshot = activity.snapshot();
    assert_eq!(snapshot.entry_count, 1);
    assert_track_changed_event(&snapshot.entries[0].event, "track-ok");
}
