use crate::playback::activity_log::PlaybackActivityLogState;
use crate::playback::errors::PlaybackResult;
use crate::playback::events::PlaybackEvent;
use crate::playback::state::PlaybackState;

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
