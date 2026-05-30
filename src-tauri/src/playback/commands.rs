use tauri::State;

use super::activity_log::PlaybackActivityLogState;
use super::backend_status::BackendStatus;
use super::backends::backend_types::PlaybackBackendDescriptor;
use super::core_profile::KivoCoreAudioProfile;
use super::errors::PlaybackError;
use super::events::PlaybackEvent;
use super::manager::PlaybackManagerState;
use super::state::PlaybackState;
use super::types::PlaybackTrack;

fn record_event(activity: &PlaybackActivityLogState, event: PlaybackEvent) {
    activity.append(event);
}

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
) -> Vec<PlaybackEvent> {
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

    match &result {
        Ok(state) => record_event(&activity, PlaybackEvent::TrackChanged(state.clone())),
        Err(error) => record_event(&activity, PlaybackEvent::Error(error.clone())),
    }

    result
}

#[tauri::command]
pub fn playback_play(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.play();

    match &result {
        Ok(state) => record_event(&activity, PlaybackEvent::StateChanged(state.clone())),
        Err(error) => record_event(&activity, PlaybackEvent::Error(error.clone())),
    }

    result
}

#[tauri::command]
pub fn playback_pause(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.pause();

    match &result {
        Ok(state) => record_event(&activity, PlaybackEvent::StateChanged(state.clone())),
        Err(error) => record_event(&activity, PlaybackEvent::Error(error.clone())),
    }

    result
}

#[tauri::command]
pub fn playback_resume(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.resume();

    match &result {
        Ok(state) => record_event(&activity, PlaybackEvent::StateChanged(state.clone())),
        Err(error) => record_event(&activity, PlaybackEvent::Error(error.clone())),
    }

    result
}

#[tauri::command]
pub fn playback_stop(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.stop();

    match &result {
        Ok(state) => record_event(&activity, PlaybackEvent::StateChanged(state.clone())),
        Err(error) => record_event(&activity, PlaybackEvent::Error(error.clone())),
    }

    result
}

#[tauri::command]
pub fn playback_seek(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
    position_ms: u64,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.seek(position_ms);

    match &result {
        Ok(state) => record_event(&activity, PlaybackEvent::StateChanged(state.clone())),
        Err(error) => record_event(&activity, PlaybackEvent::Error(error.clone())),
    }

    result
}

#[tauri::command]
pub fn playback_set_volume(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
    level: f32,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.set_volume(level);

    match &result {
        Ok(state) => record_event(&activity, PlaybackEvent::StateChanged(state.clone())),
        Err(error) => record_event(&activity, PlaybackEvent::Error(error.clone())),
    }

    result
}

#[tauri::command]
pub fn playback_set_muted(
    manager: State<'_, PlaybackManagerState>,
    activity: State<'_, PlaybackActivityLogState>,
    muted: bool,
) -> Result<PlaybackState, PlaybackError> {
    let result = manager.set_muted(muted);

    match &result {
        Ok(state) => record_event(&activity, PlaybackEvent::StateChanged(state.clone())),
        Err(error) => record_event(&activity, PlaybackEvent::Error(error.clone())),
    }

    result
}
