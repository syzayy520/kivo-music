use crate::playback::activity_log::PlaybackActivityLogState;
use crate::playback::command_activity::{run_state_command, run_track_command};
use crate::playback::errors::PlaybackError;
use crate::playback::events::PlaybackEvent;
use crate::playback::manager::PlaybackManagerState;
use crate::playback::state::PlaybackState;

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
