use tauri::State;

use super::backends::backend_types::PlaybackBackendDescriptor;
use super::manager::PlaybackManagerState;
use super::state::PlaybackState;

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
