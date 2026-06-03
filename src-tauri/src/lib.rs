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
            playback::commands::status::playback_get_state,
            playback::commands::status::playback_get_primary_backend,
            playback::commands::status::playback_get_compatibility_backends,
            playback::commands::status::playback_get_core_profile,
            playback::commands::activity::playback_get_activity_log,
            playback::commands::activity::playback_clear_activity_log,
            playback::commands::status::playback_get_backend_status,
            playback::commands::controls::playback_load,
            playback::commands::controls::playback_play,
            playback::commands::controls::playback_pause,
            playback::commands::controls::playback_resume,
            playback::commands::controls::playback_stop,
            playback::commands::controls::playback_seek,
            playback::commands::controls::playback_set_volume,
            playback::commands::controls::playback_set_muted,
            playback::commands::queue::playback_queue_get,
            playback::commands::queue::playback_queue_append,
            playback::commands::queue::playback_queue_remove,
            playback::commands::queue::playback_queue_set_repeat_mode,
            playback::commands::queue::playback_queue_set_shuffle,
            playback::commands::queue::playback_queue_set_current,
            playback::commands::queue::playback_queue_next,
            playback::commands::queue::playback_queue_previous,
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
