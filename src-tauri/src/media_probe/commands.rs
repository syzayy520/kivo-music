use tauri::State;

use super::activity_log::MediaProbeActivityLogState;
use super::errors::MediaProbeError;
use super::event::MediaProbeEvent;
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
pub fn media_probe_get_activity_log(
    activity_log: State<'_, MediaProbeActivityLogState>,
) -> Vec<MediaProbeEvent> {
    activity_log.entries()
}

#[tauri::command]
pub fn media_probe_clear_activity_log(activity_log: State<'_, MediaProbeActivityLogState>) {
    activity_log.clear();
}

#[tauri::command]
pub fn media_probe_file(
    service: State<'_, MediaProbeServiceState>,
    activity_log: State<'_, MediaProbeActivityLogState>,
    path: String,
) -> Result<MediaProbeResult, MediaProbeError> {
    let backend_name = service.backend_name().to_string();

    activity_log.record(MediaProbeEvent::started(path.clone(), backend_name.clone()));

    match service.probe(&path) {
        Ok(result) => {
            activity_log.record(MediaProbeEvent::succeeded(path, backend_name));
            Ok(result)
        }
        Err(error) => {
            activity_log.record(MediaProbeEvent::failed(
                path,
                backend_name,
                error.to_string(),
            ));
            Err(error)
        }
    }
}
