pub mod audio;
pub mod library;
mod library_db;

use std::path::PathBuf;

use crate::audio::commands::{
    audio_ffmpeg_decode_to_wav, audio_ffmpeg_play_file_wasapi_shared, audio_file_probe, audio_file_read_bytes,
    audio_service_next,
    audio_service_pause, audio_service_play_start, audio_service_prev, audio_service_queue_add,
    audio_service_queue_clear, audio_service_queue_get, audio_service_queue_set, audio_service_seek,
    audio_service_set_volume, audio_service_status, audio_service_stop, audio_service_tick,
    audio_wasapi_list_render_devices,
};
use crate::audio::state::AudioService;
use crate::library::commands::{library_counts, library_scan_run, library_search_prefix};
use tauri::{LogicalSize, Manager};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn audio_wasapi_shared_smoke_start(mode: String, duration_secs: u32) -> Result<String, String> {
    let parsed_mode = audio::SmokeMode::parse(&mode).map_err(|e| e.to_string())?;
    let config = audio::SmokeTestConfig {
        mode: parsed_mode,
        duration_secs: duration_secs.max(1),
        ..Default::default()
    };
    let log_path = audio::spawn_smoke_test(config).map_err(|e| e.to_string())?;
    Ok(log_path.display().to_string())
}

#[tauri::command]
fn library_scan_default(app: tauri::AppHandle) -> Result<crate::library::ScanSummary, String> {
    let user_profile = std::env::var("USERPROFILE").map_err(|e| format!("LIB_SCAN_FAILED: {e}"))?;
    let music_root = PathBuf::from(user_profile).join("Music");
    library_scan_run(app, vec![music_root.display().to_string()])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AudioService::default())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_size(LogicalSize::new(1180.0, 760.0));
                let _ = window.center();
                let _ = window.set_decorations(false);
            }
            crate::audio::logging::AudioLog::info("Audio subsystem starting");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            library_scan_default,
            library_scan_run,
            library_counts,
            library_search_prefix,
            audio_wasapi_shared_smoke_start,
            audio_wasapi_list_render_devices,
            audio_ffmpeg_decode_to_wav,
            audio_ffmpeg_play_file_wasapi_shared,
            audio_file_probe,
            audio_file_read_bytes,
            audio_service_play_start,
            audio_service_stop,
            audio_service_pause,
            audio_service_seek,
            audio_service_status,
            audio_service_tick,
            audio_service_set_volume,
            audio_service_queue_set,
            audio_service_queue_add,
            audio_service_queue_get,
            audio_service_queue_clear,
            audio_service_next,
            audio_service_prev,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
