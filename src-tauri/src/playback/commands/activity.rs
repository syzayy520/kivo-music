use tauri::State;

use crate::playback::activity_log::PlaybackActivityLogState;
use crate::playback::activity_snapshot::PlaybackActivityLogSnapshot;

#[tauri::command]
pub fn playback_get_activity_log(
    activity: State<'_, PlaybackActivityLogState>,
) -> PlaybackActivityLogSnapshot {
    activity.snapshot()
}

#[tauri::command]
pub fn playback_clear_activity_log(activity: State<'_, PlaybackActivityLogState>) {
    activity.clear();
}
