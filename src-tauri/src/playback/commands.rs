use tauri::State;

use super::activity_log::PlaybackActivityLogState;
use super::activity_recorder::{record_state_result, record_track_result};
use super::activity_snapshot::PlaybackActivityLogSnapshot;
use super::backend_status::BackendStatus;
use super::backends::backend_types::PlaybackBackendDescriptor;
use super::core_profile::KivoCoreAudioProfile;
use super::errors::PlaybackError;
use super::manager::PlaybackManagerState;
use super::queue::PlaybackQueue;
use super::state::PlaybackState;
use super::types::{PlaybackTrack, RepeatMode};

#[tauri::command]
pub fn playback_get_state(manager: State<'_, PlaybackManagerState>) -> PlaybackState {
    manager.current_state()
}

#[tauri::command]
pub fn playback_get_primary_backend(
    manager: State<'_, PlaybackManagerState>,
) -> PlaybackBackendDescriptor {
    manager.primary_backend()
}

#[tauri::command]
pub fn playback_get_compatibility_backends(
    manager: State<'_, PlaybackManagerState>,
) -> Vec<PlaybackBackendDescriptor> {
    manager.compatibility_backends()
}

#[tauri::command]
pub fn playback_get_core_profile() -> KivoCoreAudioProfile {
    KivoCoreAudioProfile::default()
}

#[tauri::command]
pub fn playback_get_activity_log(
    activity: State<'_, PlaybackActivityLogState>,
) -> PlaybackActivityLogSnapshot {
    activity.snapshot()
}

#[tauri::command]
pub fn playback_clear_activity_log(activity: State<'_, PlaybackActivityLogState>) {
    activity.clear();
}

#[tauri::command]
pub fn playback_get_backend_status(manager: State<'_, PlaybackManagerState>) -> BackendStatus {
    let backend = manager.primary_backend();

    BackendStatus {
        backend_name: backend.name,
        available: false,
        note: Some("Kivo Core Audio is scaffolded; real output is not wired yet".to_string()),
    }
}

#[tauri::command]
pub fn playback_load(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
    track: PlaybackTrack,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.load(track);
    record_track_result(&activity, &result);
    result
}

#[tauri::command]
pub fn playback_play(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.play();
    record_state_result(&activity, &result);
    result
}

#[tauri::command]
pub fn playback_pause(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.pause();
    record_state_result(&activity, &result);
    result
}

#[tauri::command]
pub fn playback_resume(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.resume();
    record_state_result(&activity, &result);
    result
}

#[tauri::command]
pub fn playback_stop(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.stop();
    record_state_result(&activity, &result);
    result
}

#[tauri::command]
pub fn playback_seek(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
    position_ms: u64,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.seek(position_ms);
    record_state_result(&activity, &result);
    result
}

#[tauri::command]
pub fn playback_set_volume(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
    level: f32,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.set_volume(level);
    record_state_result(&activity, &result);
    result
}

#[tauri::command]
pub fn playback_set_muted(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
    muted: bool,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.set_muted(muted);
    record_state_result(&activity, &result);
    result
}

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
    let result = manager.queue_set_current(index);
    record_state_result(&activity, &result);
    result
}

#[tauri::command]
pub fn playback_queue_next(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.queue_next();
    record_state_result(&activity, &result);
    result
}

#[tauri::command]
pub fn playback_queue_previous(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.queue_previous();
    record_state_result(&activity, &result);
    result
}
