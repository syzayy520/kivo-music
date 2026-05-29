use std::path::{Path, PathBuf};

use super::errors::{PlaybackError, PlaybackResult};

pub const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["mp3", "flac", "wav", "m4a", "aac", "alac", "ogg", "opus"];

pub fn validate_local_audio_path(raw_path: &str) -> PlaybackResult<PathBuf> {
    if raw_path.trim().is_empty() {
        return Err(PlaybackError::Path("empty audio path".to_string()));
    }

    let path = Path::new(raw_path);

    if !path.exists() {
        return Err(PlaybackError::Path("audio file does not exist".to_string()));
    }

    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .ok_or_else(|| PlaybackError::UnsupportedFormat("missing file extension".to_string()))?;

    if !SUPPORTED_AUDIO_EXTENSIONS.contains(&extension.as_str()) {
        return Err(PlaybackError::UnsupportedFormat(extension));
    }

    Ok(path.to_path_buf())
}
