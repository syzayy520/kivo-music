use super::activity_log::PlaybackActivityLogState;
use super::command_activity::{run_state_command, run_track_command};
use super::errors::PlaybackError;
use super::events::PlaybackEvent;
use super::manager::PlaybackManagerState;
use super::state::PlaybackState;

#[test]
fn run_state_command_records_state_changed_on_success() {
    let manager = PlaybackManagerState::default();
    let activity = PlaybackActivityLogState::default();

    let result = run_state_command(&manager, &activity, |_| Ok(PlaybackState::default()));

    assert!(result.is_ok());
    let snapshot = activity.snapshot();
    assert_eq!(snapshot.entry_count, 1);
    assert!(matches!(
        snapshot.entries[0].event,
        PlaybackEvent::StateChanged(_)
    ));
}

#[test]
fn run_track_command_records_error_on_failure() {
    let manager = PlaybackManagerState::default();
    let activity = PlaybackActivityLogState::default();

    let result = run_track_command(&manager, &activity, |_| {
        Err(PlaybackError::Playback("failed".to_string()))
    });

    assert!(result.is_err());
    let snapshot = activity.snapshot();
    assert_eq!(snapshot.entry_count, 1);
    assert!(matches!(snapshot.entries[0].event, PlaybackEvent::Error(_)));
}
