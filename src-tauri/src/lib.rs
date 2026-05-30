pub mod media_probe;
pub mod playback;

pub fn run() {
    tauri::Builder::default()
        .manage(playback::manager::PlaybackManagerState::default())
        .invoke_handler(tauri::generate_handler![
            playback::commands::playback_get_state,
            playback::commands::playback_get_primary_backend,
            playback::commands::playback_get_compatibility_backends,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Kivo Music");
}
