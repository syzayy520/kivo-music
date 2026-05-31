use super::activity_log::PlaybackActivityLogState;
use super::activity_recorder::{record_state_result, record_track_result};
use super::errors::PlaybackError;
use super::manager::PlaybackManagerState;
use super::state::PlaybackState;

pub fn run_state_command<F>(
    manager: &PlaybackManagerState,
    activity: &PlaybackActivityLogState,
    run: F,
) -> Result<PlaybackState, PlaybackError>
where
    F: FnOnce(&PlaybackManagerState) -> Result<PlaybackState, PlaybackError>,
{
    let result = run(manager);
    record_state_result(activity, &result);
    result
}

pub fn run_track_command<F>(
    manager: &PlaybackManagerState,
    activity: &PlaybackActivityLogState,
    run: F,
) -> Result<PlaybackState, PlaybackError>
where
    F: FnOnce(&PlaybackManagerState) -> Result<PlaybackState, PlaybackError>,
{
    let result = run(manager);
    record_track_result(activity, &result);
    result
}
