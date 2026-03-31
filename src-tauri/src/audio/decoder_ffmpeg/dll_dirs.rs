use std::{
    env,
    path::{Path, PathBuf},
};

use crate::audio::errors::{AudioError, AudioResult};

#[derive(Debug, Clone)]
pub(super) struct DirCandidate {
    pub(super) dir: PathBuf,
    pub(super) source: &'static str,
}

pub(super) fn ffmpeg_dir_candidates(user_dir: Option<PathBuf>) -> AudioResult<Vec<DirCandidate>> {
    let mut out: Vec<DirCandidate> = Vec::new();

    if let Some(d) = user_dir {
        if !d.is_dir() {
            return Err(AudioError::FfmpegDirInvalid(d));
        }
        push_unique(&mut out, d, "user(dll_dir)");
    }

    if let Ok(s) = env::var("KIVO_FFMPEG_DIR") {
        if !s.trim().is_empty() {
            let d = PathBuf::from(s);
            if !d.is_dir() {
                return Err(AudioError::FfmpegDirInvalid(d));
            }
            push_unique(&mut out, d, "env(KIVO_FFMPEG_DIR)");
        }
    }

    push_unique(
        &mut out,
        dev_third_party_ffmpeg_bin_dir(),
        "dev(third_party/ffmpeg/bin)",
    );

    for d in resources_ffmpeg_bin_dirs() {
        push_unique(&mut out, d, "resources(ffmpeg/bin)");
    }

    Ok(out)
}

pub(super) fn format_dir_order(candidates: &[DirCandidate]) -> String {
    candidates
        .iter()
        .map(|c| format!("{}={}", c.source, c.dir.display()))
        .collect::<Vec<String>>()
        .join(" | ")
}

fn dev_third_party_ffmpeg_bin_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir.parent().unwrap_or(&manifest_dir);
    repo_root.join("third_party").join("ffmpeg").join("bin")
}

fn resources_ffmpeg_bin_dirs() -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = Vec::new();

    let exe = match env::current_exe() {
        Ok(p) => p,
        Err(_) => return v,
    };
    let exe_dir = match exe.parent() {
        Some(p) => p.to_path_buf(),
        None => return v,
    };

    v.push(exe_dir.join("resources").join("ffmpeg").join("bin"));
    v.push(
        exe_dir
            .join("resources")
            .join("third_party")
            .join("ffmpeg")
            .join("bin"),
    );

    if let Some(parent) = exe_dir.parent() {
        v.push(parent.join("resources").join("ffmpeg").join("bin"));
        v.push(
            parent
                .join("resources")
                .join("third_party")
                .join("ffmpeg")
                .join("bin"),
        );
    }

    v
}

fn push_unique(out: &mut Vec<DirCandidate>, dir: PathBuf, source: &'static str) {
    if out.iter().any(|c| c.dir == dir) {
        return;
    }
    out.push(DirCandidate { dir, source });
}

#[allow(dead_code)]
fn _assert_path_used(_: &Path) {}
