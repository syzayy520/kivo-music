use tauri::State;

use crate::playback::activity_log::PlaybackActivityLogState;
use crate::playback::command_activity::run_state_command;
use crate::playback::errors::PlaybackError;
use crate::playback::manager::PlaybackManagerState;
use crate::playback::queue::PlaybackQueue;
use crate::playback::state::PlaybackState;
use crate::playback::types::{PlaybackTrack, RepeatMode};

#[tauri::command]
pub fn playback_queue_get(manager: State<'_, PlaybackManagerState>) -> PlaybackQueue {
    manager.queue()
}

#[tauri::command]
pub fn playback_queue_append(
    manager: State<'_, PlaybackManagerState>,
    track: PlaybackTrack,
) -> PlaybackQueue {
    manager.queue_append(track)
}

#[tauri::command]
pub fn playback_queue_remove(
    manager: State<'_, PlaybackManagerState>,
    index: usize,
) -> Result<PlaybackQueue, PlaybackError> {
    manager.queue_remove(index)
}

#[tauri::command]
pub fn playback_queue_set_repeat_mode(
    manager: State<'_, PlaybackManagerState>,
    repeat_mode: RepeatMode,
) -> PlaybackQueue {
    manager.queue_set_repeat_mode(repeat_mode)
}

#[tauri::command]
pub fn playback_queue_set_shuffle(
    manager: State<'_, PlaybackManagerState>,
    shuffle: bool,
) -> PlaybackQueue {
    manager.queue_set_shuffle(shuffle)
}

#[tauri::command]
pub fn playback_queue_set_current(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
    index: usize,
) -> Result<PlaybackState, PlaybackError> {
    run_state_command(&manager, &activity, move |service| {
        service.queue_set_current(index)
    })
}

#[tauri::command]
pub fn playback_queue_next(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    run_state_command(&manager, &activity, PlaybackManagerState::queue_next)
}

#[tauri::command]
pub fn playback_queue_previous(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    run_state_command(&manager, &activity, PlaybackManagerState::queue_previous)
}
