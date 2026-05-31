use tauri::State;

use super::errors::MediaProbeError;
use super::service::MediaProbeServiceState;
use super::status::MediaProbeBackendStatus;
use super::types::MediaProbeResult;

#[tauri::command]
pub fn media_probe_get_backend_name(service: State<'_, MediaProbeServiceState>) -> String {
    service.backend_name().to_string()
}

#[tauri::command]
pub fn media_probe_get_backend_status(
    service: State<'_, MediaProbeServiceState>,
) -> MediaProbeBackendStatus {
    service.backend_status()
}

#[tauri::command]
pub fn media_probe_file(
    service: State<'_, MediaProbeServiceState>,
    path: String,
) -> Result<MediaProbeResult, MediaProbeError> {
    service.probe(&path)
}
