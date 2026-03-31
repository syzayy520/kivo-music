use std::path::Path;

use crate::audio::decoder_ffmpeg::{probe_media, FfmpegLoadOptions};
use crate::audio::validation::validate_file_path;

#[derive(serde::Serialize)]
pub struct AudioFileMetadata {
    pub album: Option<String>,
    pub artist: Option<String>,
    pub duration_seconds: f64,
    pub title: Option<String>,
}

#[tauri::command]
pub fn audio_file_read_bytes(input_path: String) -> Result<Vec<u8>, String> {
    let input_path = input_path.trim().to_string();
    validate_file_path(&input_path).map_err(|e| e.to_public_string())?;
    std::fs::read(&input_path).map_err(|e| format!("AUDIO_READ_FAILED: {e}"))
}

#[tauri::command]
pub fn audio_file_probe(input_path: String) -> Result<AudioFileMetadata, String> {
    let input_path = input_path.trim().to_string();
    validate_file_path(&input_path).map_err(|e| e.to_public_string())?;
    let info =
        probe_media(Path::new(&input_path), FfmpegLoadOptions::default()).map_err(|e| e.to_public_string())?;
    Ok(AudioFileMetadata {
        album: info.tags.album,
        artist: info.tags.artist,
        duration_seconds: info.duration_seconds,
        title: info.tags.title,
    })
}
