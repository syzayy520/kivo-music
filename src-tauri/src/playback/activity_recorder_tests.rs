use super::activity_log::PlaybackActivityLogState;
use super::activity_recorder::{record_state_result, record_track_result};
use super::errors::{PlaybackError, PlaybackResult};
use super::events::PlaybackEvent;
use super::state::PlaybackState;

#[test]
fn record_state_result_appends_state_changed_event() {
    let activity = PlaybackActivityLogState::default();
    let result: PlaybackResult<PlaybackState> = Ok(PlaybackState::default());

    record_state_result(&activity, &result);

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 1);
    assert!(matches!(
        snapshot.entries[0].event,
        PlaybackEvent::StateChanged(_)
    ));
    assert!(snapshot.entries[0].timestamp_ms > 0);
}

#[test]
fn record_track_result_appends_track_changed_event() {
    let activity = PlaybackActivityLogState::default();
    let result: PlaybackResult<PlaybackState> = Ok(PlaybackState::default());

    record_track_result(&activity, &result);

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 1);
    assert!(matches!(
        snapshot.entries[0].event,
        PlaybackEvent::TrackChanged(_)
    ));
    assert!(snapshot.entries[0].timestamp_ms > 0);
}

#[test]
fn recorder_appends_error_event_on_failure() {
    let activity = PlaybackActivityLogState::default();
    let result: PlaybackResult<PlaybackState> = Err(PlaybackError::Playback("failed".to_string()));

    record_state_result(&activity, &result);

    let snapshot = activity.snapshot();

    assert_eq!(snapshot.entry_count, 1);
    assert!(matches!(
        snapshot.entries[0].event,
        PlaybackEvent::Error(PlaybackError::Playback(_))
    ));
}
