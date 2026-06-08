use super::lifecycle_activity_log::PlaybackLifecycleActivityLogState;
use super::lifecycle_event::PlaybackLifecycleEvent;

pub fn record_lifecycle_initialized(
    activity: &PlaybackLifecycleActivityLogState,
    backend_name: &str,
) {
    activity.append(PlaybackLifecycleEvent::initialized(
        backend_name.to_string(),
    ));
}

pub fn record_lifecycle_shutdown_started(
    activity: &PlaybackLifecycleActivityLogState,
    backend_name: &str,
) {
    activity.append(PlaybackLifecycleEvent::shutdown_started(
        backend_name.to_string(),
    ));
}

pub fn record_lifecycle_shutdown_succeeded(
    activity: &PlaybackLifecycleActivityLogState,
    backend_name: &str,
) {
    activity.append(PlaybackLifecycleEvent::shutdown_succeeded(
        backend_name.to_string(),
    ));
}

pub fn record_lifecycle_shutdown_failed(
    activity: &PlaybackLifecycleActivityLogState,
    backend_name: &str,
    message: &str,
) {
    activity.append(PlaybackLifecycleEvent::shutdown_failed(
        backend_name.to_string(),
        message.to_string(),
    ));
}
