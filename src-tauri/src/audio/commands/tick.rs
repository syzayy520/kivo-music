use crate::audio::state::AudioService;
use tauri::State;

use super::types::AudioServiceTickResult;

#[tauri::command]
pub fn audio_service_tick(svc: State<'_, AudioService>) -> Result<AudioServiceTickResult, String> {
    let (queue, status) = svc.tick().map_err(|e| e.to_public_string())?;
    Ok(AudioServiceTickResult { queue, status })
}
