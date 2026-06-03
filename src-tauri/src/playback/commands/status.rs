use tauri::State;

use crate::playback::backend_status::BackendStatus;
use crate::playback::backends::backend_types::PlaybackBackendDescriptor;
use crate::playback::core_profile::KivoCoreAudioProfile;
use crate::playback::manager::PlaybackManagerState;
use crate::playback::state::PlaybackState;

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
