mod playback;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![playback::commands::playback_get_state])
        .run(tauri::generate_context!())
        .expect("failed to run Kivo Music");
}
