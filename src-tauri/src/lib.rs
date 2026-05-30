pub mod media_probe;
pub mod playback;

pub fn run() {
    tauri::Builder::default()
        .manage(playback::manager::PlaybackManagerState::default())
        .manage(media_probe::service::MediaProbeServiceState::default())
        .invoke_handler(tauri::generate_handler![
            playback::commands::playback_get_state,
            playback::commands::playback_get_primary_backend,
            playback::commands::playback_get_compatibility_backends,
            media_probe::commands::media_probe_get_backend_name,
            media_probe::commands::media_probe_file,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Kivo Music");
}
