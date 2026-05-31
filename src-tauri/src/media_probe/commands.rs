use tauri::State;

use super::activity_log::MediaProbeActivityLogState;
use super::activity_logger::{
    record_probe_failed, record_probe_started, record_probe_succeeded,
};
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

    record_probe_started(&activity_log, &path, &backend_name);

    match service.probe(&path) {
        Ok(result) => {
            record_probe_succeeded(&activity_log, &path, &backend_name);
            Ok(result)
        }
        Err(error) => {
            record_probe_failed(&activity_log, &path, &backend_name, &error);
            Err(error)
        }
    }
}
