use crate::audio::errors::AudioError;
use crate::audio::state::{
    self, AudioService, AudioSessionStatus, DecodeToWavArgs, DecodeToWavResult, PlayStartOptions,
};
use crate::audio::validation::{validate_file_path, validate_seconds, validate_volume};
use tauri::State;

/// Decode an audio file into WAV via the FFmpeg decoder (runtime-loaded DLLs).
///
/// This is used by tooling / debugging flows and is exposed as a Tauri command.
#[tauri::command]

pub fn audio_ffmpeg_decode_to_wav(
    input_path: String,
    output_path: Option<String>,
    start_seconds: Option<f64>,
    max_seconds: Option<f64>,
    ffmpeg_dir: Option<String>,
) -> Result<DecodeToWavResult, String> {
    let input_path = input_path.trim().to_string();
    validate_decode_request(&input_path, start_seconds, max_seconds)
        .map_err(|e| e.to_public_string())?;

    let args = DecodeToWavArgs {
        input_path,
        output_path,
        start_seconds,
        max_seconds,
        ffmpeg_dir,
    };

    state::run_ffmpeg_decode_to_wav(args).map_err(|e| e.to_public_string())
}

/// Play an audio file through WASAPI Shared using FFmpeg decoder (runtime-loaded DLLs).
///
/// This is a bring-up / E2E pipeline entry:
/// FFmpeg decode -> ring buffer -> WASAPI Shared render.
#[tauri::command]

pub fn audio_ffmpeg_play_file_wasapi_shared(
    input_path: String,
    start_seconds: Option<f64>,
    max_seconds: Option<f64>,
    ffmpeg_dir: Option<String>,
) -> Result<state::PlayFileResult, String> {
    let input_path = input_path.trim().to_string();
    validate_decode_request(&input_path, start_seconds, max_seconds)
        .map_err(|e| e.to_public_string())?;

    let args = state::PlayFileArgs {
        input_path,
        start_seconds,
        max_seconds,
        ffmpeg_dir,
    };

    state::run_ffmpeg_play_file_wasapi_shared(args).map_err(|e| e.to_public_string())
}

/// List WASAPI render (output) devices.
///
/// This is a thin command wrapper around `audio::output_wasapi::try_list_render_devices()`.
///
/// Errors are returned using `AudioError::to_public_string()`.
#[tauri::command]

pub fn audio_service_play_start(
    svc: State<'_, AudioService>,
    input_path: String,
    opts: Option<PlayStartOptions>,
) -> Result<u64, String> {
    svc.play_start(input_path, opts.unwrap_or_default())
        .map_err(|e| e.to_public_string())
}

#[tauri::command]

pub fn audio_service_stop(svc: State<'_, AudioService>, session_id: u64) -> Result<(), String> {
    svc.stop(session_id).map_err(|e| e.to_public_string())
}

#[tauri::command]

pub fn audio_service_pause(
    svc: State<'_, AudioService>,
    session_id: u64,
    paused: bool,
) -> Result<(), String> {
    svc.pause(session_id, paused)
        .map_err(|e| e.to_public_string())
}

#[tauri::command]

pub fn audio_service_seek(
    svc: State<'_, AudioService>,
    session_id: u64,
    seconds: f64,
) -> Result<(), String> {
    svc.seek(session_id, seconds)
        .map_err(|e| e.to_public_string())
}

#[tauri::command]

pub fn audio_service_status(
    svc: State<'_, AudioService>,
    session_id: u64,
) -> Result<AudioSessionStatus, String> {
    svc.status(session_id).map_err(|e| e.to_public_string())
}

#[tauri::command]

pub fn audio_service_set_volume(
    svc: State<'_, AudioService>,
    session_id: u64,
    volume: f32,
) -> Result<(), String> {
    validate_volume(volume).map_err(|e| e.to_public_string())?;

    svc.set_volume(session_id, volume)
        .map_err(|e| e.to_public_string())
}

fn validate_decode_request(
    input_path: &str,
    start_seconds: Option<f64>,
    max_seconds: Option<f64>,
) -> Result<(), AudioError> {
    validate_file_path(input_path)?;
    validate_seconds(start_seconds, "start_seconds", true)?;
    validate_seconds(max_seconds, "max_seconds", false)?;
    Ok(())
}
