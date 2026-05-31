use super::activity_log::MediaProbeActivityLogState;
use super::errors::MediaProbeError;
use super::event::MediaProbeEvent;

pub fn record_probe_started(
    activity_log: &MediaProbeActivityLogState,
    path: &str,
    backend_name: &str,
) {
    activity_log.record(MediaProbeEvent::started(
        path.to_string(),
        backend_name.to_string(),
    ));
}

pub fn record_probe_succeeded(
    activity_log: &MediaProbeActivityLogState,
    path: &str,
    backend_name: &str,
) {
    activity_log.record(MediaProbeEvent::succeeded(
        path.to_string(),
        backend_name.to_string(),
    ));
}

pub fn record_probe_failed(
    activity_log: &MediaProbeActivityLogState,
    path: &str,
    backend_name: &str,
    error: &MediaProbeError,
) {
    activity_log.record(MediaProbeEvent::failed(
        path.to_string(),
        backend_name.to_string(),
        error.to_string(),
    ));
}
