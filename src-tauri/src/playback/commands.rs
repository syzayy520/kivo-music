use super::state::PlaybackState;

#[tauri::command]
pub fn playback_get_state() -> PlaybackState {
    PlaybackState::default()
}
