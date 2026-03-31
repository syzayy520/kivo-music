use std::path::{Path, PathBuf};

use crate::audio::errors::{AudioError, AudioResult};

use super::{
    dll_dirs::{ffmpeg_dir_candidates, format_dir_order},
    dll_names,
    dynlib::DynLib,
};

#[derive(Debug)]
pub struct FfmpegDllSet {
    pub avformat: DynLib,
    pub avcodec: DynLib,
    pub avutil: DynLib,
    pub swresample: DynLib,
}

/// Load report for FFmpeg DLL resolution.
#[derive(Debug, Clone)]
pub struct FfmpegDllLoadReport {
    /// The directory that successfully loaded all required FFmpeg DLLs.
    pub resolved_dir: Option<PathBuf>,
    /// All candidate directories that were attempted (in order).
    pub attempted_dirs: Vec<PathBuf>,
}

impl FfmpegDllSet {
    /// Load the required FFmpeg DLLs.
    ///
    /// Search order (when `user_dir` is None):
    /// 1) env: KIVO_FFMPEG_DIR
    /// 2) dev: <repo>/third_party/ffmpeg/bin
    /// 3) release: <exe_dir>/resources/ffmpeg/bin
    ///
    /// If `user_dir` is provided, it is treated as an explicit override and is tried first.
    pub fn load(user_dir: Option<PathBuf>) -> AudioResult<(Self, FfmpegDllLoadReport)> {
        let candidates = ffmpeg_dir_candidates(user_dir)?;
        if candidates.is_empty() {
            return Err(AudioError::InvalidInput(
                "no ffmpeg dll dir candidates. set env KIVO_FFMPEG_DIR or place DLLs under third_party/ffmpeg/bin"
                    .to_string(),
            ));
        }

        let attempted_dirs: Vec<PathBuf> = candidates.iter().map(|c| c.dir.clone()).collect();
        let order = format_dir_order(&candidates);

        let mut last_err: Option<AudioError> = None;
        for c in &candidates {
            let search = format!(
                "order={order}; active={}; dir={}",
                c.source,
                c.dir.display()
            );
            match try_load_from_dir(c.dir.as_path(), &search, &attempted_dirs) {
                Ok(dlls) => {
                    return Ok((
                        dlls,
                        FfmpegDllLoadReport {
                            resolved_dir: Some(c.dir.clone()),
                            attempted_dirs,
                        },
                    ))
                }
                Err(e) => last_err = Some(e),
            }
        }

        // Failed all candidates.
        Err(last_err.unwrap_or_else(|| AudioError::InvalidInput("ffmpeg dll load failed".into())))
    }
}

fn try_load_from_dir(
    dir: &Path,
    search: &str,
    attempted_dirs: &[PathBuf],
) -> AudioResult<FfmpegDllSet> {
    let avformat = load_one(
        "avformat",
        dll_names::avformat_candidates(),
        Some(dir),
        search,
        attempted_dirs,
    )?;
    let avcodec = load_one(
        "avcodec",
        dll_names::avcodec_candidates(),
        Some(dir),
        search,
        attempted_dirs,
    )?;
    let avutil = load_one(
        "avutil",
        dll_names::avutil_candidates(),
        Some(dir),
        search,
        attempted_dirs,
    )?;
    let swresample = load_one(
        "swresample",
        dll_names::swresample_candidates(),
        Some(dir),
        search,
        attempted_dirs,
    )?;

    Ok(FfmpegDllSet {
        avformat,
        avcodec,
        avutil,
        swresample,
    })
}

fn load_one(
    prefix: &'static str,
    candidates: &[&'static str],
    dir: Option<&Path>,
    search: &str,
    attempted_dirs: &[PathBuf],
) -> AudioResult<DynLib> {
    let mut tried: Vec<String> = Vec::new();
    let mut last_error: Option<String> = None;

    for name in candidates {
        tried.push(name.to_string());
        match DynLib::load(name, dir, search) {
            Ok(l) => return Ok(l),
            Err(e) => last_error = Some(e.to_string()),
        }
    }

    Err(AudioError::FfmpegLibraryNotFound {
        prefix,
        search: search.to_string(),
        attempted_dirs: attempted_dirs
            .iter()
            .map(|p| p.display().to_string())
            .collect(),
        tried,
        last_error,
    })
}
