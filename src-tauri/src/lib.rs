pub mod media_probe;
pub mod playback;

pub fn run() {
    tauri::Builder::default()
        .manage(playback::manager::PlaybackManagerState::default())
        .manage(playback::activity_log::PlaybackActivityLogState::default())
        .manage(playback::lifecycle_activity_log::PlaybackLifecycleActivityLogState::default())
        .manage(media_probe::service::MediaProbeServiceState::default())
        .manage(media_probe::activity_log::MediaProbeActivityLogState::default())
        .invoke_handler(tauri::generate_handler![
            playback::commands::playback_get_state,
            playback::commands::playback_get_primary_backend,
            playback::commands::playback_get_compatibility_backends,
            playback::commands::playback_get_core_profile,
            playback::commands::playback_get_activity_log,
            playback::commands::playback_clear_activity_log,
            playback::commands::playback_get_backend_status,
            playback::commands::playback_load,
            playback::commands::playback_play,
            playback::commands::playback_pause,
            playback::commands::playback_resume,
            playback::commands::playback_stop,
            playback::commands::playback_seek,
            playback::commands::playback_set_volume,
            playback::commands::playback_set_muted,
            playback::commands::playback_queue_get,
            playback::commands::playback_queue_append,
            playback::commands::playback_queue_remove,
            playback::commands::playback_queue_set_repeat_mode,
            playback::commands::playback_queue_set_shuffle,
            playback::commands::playback_queue_set_current,
            playback::commands::playback_queue_next,
            playback::commands::playback_queue_previous,
            playback::lifecycle_commands::playback_get_lifecycle_activity_log,
            playback::lifecycle_commands::playback_clear_lifecycle_activity_log,
            media_probe::commands::media_probe_get_backend_name,
            media_probe::commands::media_probe_get_backend_status,
            media_probe::commands::media_probe_get_activity_log,
            media_probe::commands::media_probe_clear_activity_log,
            media_probe::commands::media_probe_file,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Kivo Music");
}
