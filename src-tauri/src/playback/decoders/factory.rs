use std::path::Path;

use super::wav_decoder::WavDecoder;
use crate::playback::decoder::AudioDecoder;
use crate::playback::errors::{PlaybackError, PlaybackResult};

pub fn create_decoder_for_path(path: &str) -> PlaybackResult<Box<dyn AudioDecoder>> {
    let extension = Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .ok_or_else(|| PlaybackError::UnsupportedFormat("missing file extension".to_string()))?;

    match extension.as_str() {
        "wav" => Ok(Box::new(WavDecoder::default())),
        other => Err(PlaybackError::UnsupportedFormat(other.to_string())),
    }
}
