use std::path::PathBuf;

use crate::audio::errors::{AudioError, AudioResult};

use super::{api::FfmpegLoadOptions, dll_set::FfmpegDllSet};

/// Result of probing FFmpeg dynamic libraries.
#[derive(Debug, Clone)]
pub struct FfmpegProbe {
    /// FFmpeg version string reported by `av_version_info()`.
    pub version_info: String,
    /// The directory that successfully loaded all required FFmpeg DLLs.
    pub resolved_dir: Option<PathBuf>,
    /// All candidate directories that were attempted (in order).
    pub attempted_dirs: Vec<PathBuf>,
}

/// Probe FFmpeg availability and basic metadata.
///
/// This is a pure Rust API (no Tauri command) intended for diagnostics and smoke tests.
pub fn probe_ffmpeg(opts: &FfmpegLoadOptions) -> AudioResult<FfmpegProbe> {
    let (dlls, report) = FfmpegDllSet::load(opts.dll_dir.clone())?;

    let version_info = unsafe {
        let ver_fn: unsafe extern "C" fn() -> *const i8 = dlls.avutil.sym("av_version_info")?;
        let p = ver_fn();
        if p.is_null() {
            "unknown".to_string()
        } else {
            std::ffi::CStr::from_ptr(p).to_string_lossy().to_string()
        }
    };

    let resolved_dir = report.resolved_dir;
    if resolved_dir.is_none() {
        // This should not happen in normal flows because `load()` returns Ok only
        // after successfully loading from a candidate directory.
        return Err(AudioError::InvalidInput(
            "ffmpeg probe: resolved_dir is None after successful load".to_string(),
        ));
    }

    Ok(FfmpegProbe {
        version_info,
        resolved_dir,
        attempted_dirs: report.attempted_dirs,
    })
}
