use tauri::State;

use super::lifecycle_activity_log::PlaybackLifecycleActivityLogState;
use super::lifecycle_event::PlaybackLifecycleEvent;

#[tauri::command]
pub fn playback_get_lifecycle_activity_log(
    activity: State<'_, PlaybackLifecycleActivityLogState>,
) -> Vec<PlaybackLifecycleEvent> {
    activity.snapshot()
}

#[tauri::command]
pub fn playback_clear_lifecycle_activity_log(
    activity: State<'_, PlaybackLifecycleActivityLogState>,
) {
    activity.clear();
}
