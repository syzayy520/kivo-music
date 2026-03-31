//! Library-facing wrapper for FFmpeg media probing.
//!
//! This file is intentionally kept small and stable. The underlying implementation
//! lives in `crate::audio::decoder_ffmpeg::media_probe`.
//!
//! NOTE: In the current "shell" crate layout, `crate::library` may not yet be
//! wired into the module tree. This file is added as a ready-to-plug module for
//! the full app crate (which will expose it via `mod library`).

use std::path::Path;

use crate::audio::{
    decoder_ffmpeg::{probe_media, FfmpegLoadOptions, FfmpegMediaInfo},
    errors::AudioResult,
};

/// Probe a media file using FFmpeg dynamic libraries.
///
/// Errors include a stable prefix `FFPROBE_MEDIA` from the underlying layer.
pub fn probe_media_ffmpeg(path: &Path, opts: FfmpegLoadOptions) -> AudioResult<FfmpegMediaInfo> {
    probe_media(path, opts)
}
