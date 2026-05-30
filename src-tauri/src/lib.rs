pub mod media_probe;
pub mod playback;

pub fn run() {
    tauri::Builder::default()
        .manage(playback::manager::PlaybackManagerState::default())
        .manage(playback::activity_log::PlaybackActivityLogState::default())
        .manage(media_probe::service::MediaProbeServiceState::default())
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
            media_probe::commands::media_probe_get_backend_name,
            media_probe::commands::media_probe_get_backend_status,
            media_probe::commands::media_probe_file,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Kivo Music");
}
