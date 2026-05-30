use tauri::State;

use super::backend_status::BackendStatus;
use super::backends::backend_types::PlaybackBackendDescriptor;
use super::core_profile::KivoCoreAudioProfile;
use super::errors::PlaybackError;
use super::manager::PlaybackManagerState;
use super::state::PlaybackState;
use super::types::PlaybackTrack;

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
    track: PlaybackTrack,
) -> Result<PlaybackState, PlaybackError> {
    manager.load(track)
}

#[tauri::command]
pub fn playback_play(
    manager: State<'_, PlaybackManagerState>,
) -> Result<PlaybackState, PlaybackError> {
    manager.play()
}

#[tauri::command]
pub fn playback_pause(
    manager: State<'_, PlaybackManagerState>,
) -> Result<PlaybackState, PlaybackError> {
    manager.pause()
}

#[tauri::command]
pub fn playback_resume(
    manager: State<'_, PlaybackManagerState>,
) -> Result<PlaybackState, PlaybackError> {
    manager.resume()
}

#[tauri::command]
pub fn playback_stop(
    manager: State<'_, PlaybackManagerState>,
) -> Result<PlaybackState, PlaybackError> {
    manager.stop()
}

#[tauri::command]
pub fn playback_seek(
    manager: State<'_, PlaybackManagerState>,
    position_ms: u64,
) -> Result<PlaybackState, PlaybackError> {
    manager.seek(position_ms)
}

#[tauri::command]
pub fn playback_set_volume(
    manager: State<'_, PlaybackManagerState>,
    level: f32,
) -> Result<PlaybackState, PlaybackError> {
    manager.set_volume(level)
}

#[tauri::command]
pub fn playback_set_muted(
    manager: State<'_, PlaybackManagerState>,
    muted: bool,
) -> Result<PlaybackState, PlaybackError> {
    manager.set_muted(muted)
}
