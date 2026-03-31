//! FFmpeg-backed metadata probe (no Symphonia/Rodio, no ffprobe.exe).
//!
//! This module intentionally uses FFmpeg's C structs (via a minimal "view")
//! to access `duration` and `metadata` without introducing extra dependencies.
//!
//! NOTE: This relies on the FFmpeg ABI layout for the specific DLL build you ship.

use std::{
    ffi::{CStr, CString},
    path::Path,
    ptr,
};

use crate::audio::errors::{AudioError, AudioResult};

use super::media_probe_abi::{
    open_and_probe_format, AVFormatContextView, AVStreamView, CloseInputGuard, ProbeApi,
};
use super::{dll_set::FfmpegDllSet, ffi, FfmpegLoadOptions};

const ERR_PREFIX: &str = "FFPROBE_MEDIA";

/// Minimal tag set for library/indexing.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct FfmpegMediaTags {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub date: Option<String>,
    pub year: Option<String>,
}

/// Result of probing a media file via FFmpeg dynamic libraries.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct FfmpegMediaInfo {
    pub duration_seconds: f64,
    pub tags: FfmpegMediaTags,
}

/// Probe a media file for duration + tags.
///
/// Hard requirements:
/// - no symphonia/rodio
/// - no spawning ffprobe.exe
/// - errors must include a stable prefix: `[FFPROBE_MEDIA] ...`
pub fn probe_media(path: &Path, opts: FfmpegLoadOptions) -> AudioResult<FfmpegMediaInfo> {
    if !path.is_file() {
        return Err(ffprobe_err(format!("input not found: {}", path.display())));
    }

    let (dlls, _report) =
        FfmpegDllSet::load(opts.dll_dir).map_err(|e| ffprobe_err(e.to_public_string()))?;
    let api = unsafe { ProbeApi::load(&dlls) }.map_err(|e| ffprobe_err(e.to_public_string()))?;

    let c_path = CString::new(path.to_string_lossy().as_bytes())
        .map_err(|_| ffprobe_err("input path contains NUL byte".to_string()))?;

    let fmt = unsafe { open_and_probe_format(&api, c_path.as_ptr()) }.map_err(ffprobe_err)?;
    let _guard = CloseInputGuard { api: &api, fmt };

    // ---- duration ----
    let fmt_view = unsafe { &*(fmt as *const AVFormatContextView) };
    let fmt_dur = fmt_view.duration;
    let mut duration_seconds = duration_from_format(fmt_dur);

    // Fallback: best audio stream duration * time_base.
    if duration_seconds.is_none() {
        duration_seconds = unsafe { duration_from_best_audio_stream(&api, fmt, fmt_view) };
    }

    let duration_seconds = duration_seconds
        .ok_or_else(|| ffprobe_err(format!("duration unknown (format_duration={fmt_dur})")))?;

    // ---- tags ----
    let tags = unsafe { read_tags(&api, fmt_view) };

    Ok(FfmpegMediaInfo {
        duration_seconds,
        tags,
    })
}

fn ffprobe_err(msg: impl Into<String>) -> AudioError {
    AudioError::InvalidInput(format!("[{ERR_PREFIX}] {}", msg.into()))
}

fn duration_from_format(duration_us: i64) -> Option<f64> {
    // AV_NOPTS_VALUE is i64::MIN.
    if duration_us <= 0 || duration_us == i64::MIN {
        return None;
    }
    Some(duration_us as f64 / ffi::AV_TIME_BASE as f64)
}

unsafe fn duration_from_best_audio_stream(
    api: &ProbeApi,
    fmt: *mut ffi::AVFormatContext,
    fmt_view: &AVFormatContextView,
) -> Option<f64> {
    let mut codec: *const ffi::AVCodec = ptr::null();
    let idx = (api.av_find_best_stream)(fmt, ffi::AVMEDIA_TYPE_AUDIO, -1, -1, &mut codec, 0);
    if idx < 0 {
        return None;
    }

    let idx = idx as usize;
    if fmt_view.streams.is_null() || idx >= fmt_view.nb_streams as usize {
        return None;
    }

    let st_ptr = *fmt_view.streams.add(idx);
    if st_ptr.is_null() {
        return None;
    }
    let st = &*(st_ptr as *const AVStreamView);
    if st.duration <= 0 || st.duration == i64::MIN {
        return None;
    }
    let den = st.time_base.den as f64;
    if den <= 0.0 {
        return None;
    }
    let sec = st.duration as f64 * (st.time_base.num as f64 / den);
    if sec.is_finite() && sec > 0.0 {
        Some(sec)
    } else {
        None
    }
}

unsafe fn read_tags(api: &ProbeApi, fmt_view: &AVFormatContextView) -> FfmpegMediaTags {
    let mut tags = FfmpegMediaTags::default();

    // Prefer format-level metadata.
    let dict = fmt_view.metadata;
    tags.title = dict_get(api, dict, "title");
    tags.artist = dict_get(api, dict, "artist");
    tags.album = dict_get(api, dict, "album");
    tags.date = dict_get(api, dict, "date");
    tags.year = dict_get(api, dict, "year");

    // Some files store common fields with upper-case keys.
    if tags.title.is_none() {
        tags.title = dict_get(api, dict, "TITLE");
    }
    if tags.artist.is_none() {
        tags.artist = dict_get(api, dict, "ARTIST");
    }
    if tags.album.is_none() {
        tags.album = dict_get(api, dict, "ALBUM");
    }
    if tags.date.is_none() {
        tags.date = dict_get(api, dict, "DATE");
    }
    if tags.year.is_none() {
        tags.year = dict_get(api, dict, "YEAR");
    }

    tags
}

unsafe fn dict_get(api: &ProbeApi, dict: *mut ffi::AVDictionary, key: &str) -> Option<String> {
    if dict.is_null() {
        return None;
    }
    let c_key = CString::new(key).ok()?;
    let e = (api.av_dict_get)(dict, c_key.as_ptr(), ptr::null(), 0);
    if e.is_null() {
        return None;
    }
    let v = (*e).value;
    if v.is_null() {
        return None;
    }
    Some(CStr::from_ptr(v).to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_missing_file_returns_diagnostic_error() {
        let p = Path::new("this_file_should_not_exist_1234567890.flac");
        let r = probe_media(p, FfmpegLoadOptions::default());
        assert!(r.is_err());
        let s = r.err().unwrap().to_string();
        assert!(s.contains("[FFPROBE_MEDIA]"));
        assert!(s.to_ascii_lowercase().contains("not found"));
    }
}
