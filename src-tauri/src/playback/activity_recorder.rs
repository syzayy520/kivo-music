use super::activity_log::PlaybackActivityLogState;
use super::errors::PlaybackResult;
use super::events::PlaybackEvent;
use super::state::PlaybackState;

pub fn record_state_result(
    activity: &PlaybackActivityLogState,
    result: &PlaybackResult<PlaybackState>,
) {
    match result {
        Ok(state) => activity.append(PlaybackEvent::StateChanged(state.clone())),
        Err(error) => activity.append(PlaybackEvent::Error(error.clone())),
    }
}

pub fn record_track_result(
    activity: &PlaybackActivityLogState,
    result: &PlaybackResult<PlaybackState>,
) {
    match result {
        Ok(state) => activity.append(PlaybackEvent::TrackChanged(state.clone())),
        Err(error) => activity.append(PlaybackEvent::Error(error.clone())),
    }
}
