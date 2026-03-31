use crate::audio::state::{AudioService, AudioSessionStatus, QueueSnapshot};
use tauri::State;

#[tauri::command]
pub fn audio_service_queue_set(
    svc: State<'_, AudioService>,
    items: Vec<String>,
    start_index: Option<u32>,
    autoplay: bool,
) -> Result<QueueSnapshot, String> {
    svc.queue_set(items, start_index, autoplay)
        .map_err(|e| e.to_public_string())
}

#[tauri::command]
pub fn audio_service_queue_add(
    svc: State<'_, AudioService>,
    items: Vec<String>,
) -> Result<QueueSnapshot, String> {
    svc.queue_add(items).map_err(|e| e.to_public_string())
}

#[tauri::command]
pub fn audio_service_queue_get(svc: State<'_, AudioService>) -> Result<QueueSnapshot, String> {
    svc.queue_get().map_err(|e| e.to_public_string())
}

#[tauri::command]
pub fn audio_service_queue_clear(svc: State<'_, AudioService>) -> Result<QueueSnapshot, String> {
    svc.queue_clear().map_err(|e| e.to_public_string())
}

#[tauri::command]
pub fn audio_service_next(svc: State<'_, AudioService>) -> Result<AudioSessionStatus, String> {
    svc.next().map_err(|e| e.to_public_string())
}

#[tauri::command]
pub fn audio_service_prev(svc: State<'_, AudioService>) -> Result<AudioSessionStatus, String> {
    svc.prev().map_err(|e| e.to_public_string())
}
