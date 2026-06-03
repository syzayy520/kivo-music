use tauri::State;

use crate::playback::activity_log::PlaybackActivityLogState;
use crate::playback::command_activity::{run_state_command, run_track_command};
use crate::playback::errors::PlaybackError;
use crate::playback::manager::PlaybackManagerState;
use crate::playback::state::PlaybackState;
use crate::playback::types::PlaybackTrack;

#[tauri::command]
pub fn playback_load(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
    track: PlaybackTrack,
) -> Result<PlaybackState, PlaybackError> {
    run_track_command(&manager, &activity, move |service| service.load(track))
}

#[tauri::command]
pub fn playback_play(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    run_state_command(&manager, &activity, PlaybackManagerState::play)
}

#[tauri::command]
pub fn playback_pause(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    run_state_command(&manager, &activity, PlaybackManagerState::pause)
}

#[tauri::command]
pub fn playback_resume(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    run_state_command(&manager, &activity, PlaybackManagerState::resume)
}

#[tauri::command]
pub fn playback_stop(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    run_state_command(&manager, &activity, PlaybackManagerState::stop)
}

#[tauri::command]
pub fn playback_seek(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
    position_ms: u64,
) -> Result<PlaybackState, PlaybackError> {
    run_state_command(&manager, &activity, move |service| {
        service.seek(position_ms)
    })
}

#[tauri::command]
pub fn playback_set_volume(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
    level: f32,
) -> Result<PlaybackState, PlaybackError> {
    run_state_command(&manager, &activity, move |service| {
        service.set_volume(level)
    })
}

#[tauri::command]
pub fn playback_set_muted(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
    muted: bool,
) -> Result<PlaybackState, PlaybackError> {
    run_state_command(&manager, &activity, move |service| service.set_muted(muted))
}
