use super::activity_log::PlaybackActivityLogState;
use super::errors::PlaybackError;
use super::manager::PlaybackManagerState;
use super::playback_event_bridge::PlaybackEventBridge;
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
    let mut bridge = PlaybackEventBridge::default();
    bridge.record_state_result(activity, &result);
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
    let mut bridge = PlaybackEventBridge::default();
    bridge.record_track_result(activity, &result);
    result
}
