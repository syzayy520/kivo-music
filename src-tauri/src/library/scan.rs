use super::types::{ScanSummary, TrackHit};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::Instant;

const SUPPORTED_EXTS: &[&str] = &[
    "mp3", "flac", "wav", "m4a", "aac", "ogg", "opus", "alac", "aiff", "ape", "wma",
];

pub fn scan_roots(roots: &[String]) -> (Vec<TrackHit>, ScanSummary) {
    let start = Instant::now();
    let mut tracks: Vec<TrackHit> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let mut skipped_files: u64 = 0;
    let mut next_id: u64 = 1;

    for r in roots {
        let root = PathBuf::from(r);
        if !root.exists() {
            errors.push(format!("[SCAN_ROOT_MISSING] {}", root.display()));
            continue;
        }
        walk_dir(
            &root,
            &mut |p| {
                if is_supported_audio(&p) {
                    let title = file_stem_string(&p);
                    tracks.push(TrackHit {
                        id: next_id,
                        title,
                        path: p.display().to_string(),
                    });
                    next_id = next_id.saturating_add(1);
                } else {
                    skipped_files = skipped_files.saturating_add(1);
                }
            },
            &mut errors,
        );
    }

    let summary = ScanSummary {
        roots: roots.to_vec(),
        tracks_indexed: tracks.len() as u64,
        skipped_files,
        errors,
        elapsed_ms: start.elapsed().as_millis() as u64,
    };
    (tracks, summary)
}

fn walk_dir(root: &Path, on_file: &mut dyn FnMut(PathBuf), errors: &mut Vec<String>) {
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let rd = match std::fs::read_dir(&dir) {
            Ok(v) => v,
            Err(e) => {
                errors.push(format!("[SCAN_READ_DIR_FAILED] {}: {}", dir.display(), e));
                continue;
            }
        };
        for ent in rd {
            let ent = match ent {
                Ok(v) => v,
                Err(e) => {
                    errors.push(format!(
                        "[SCAN_READ_DIR_ENTRY_FAILED] {}: {}",
                        dir.display(),
                        e
                    ));
                    continue;
                }
            };
            let path = ent.path();
            let ft = match ent.file_type() {
                Ok(v) => v,
                Err(e) => {
                    errors.push(format!("[SCAN_FILE_TYPE_FAILED] {}: {}", path.display(), e));
                    continue;
                }
            };
            if ft.is_dir() {
                stack.push(path);
            } else if ft.is_file() {
                on_file(path);
            }
        }
    }
}

fn is_supported_audio(p: &Path) -> bool {
    match p.extension().and_then(OsStr::to_str) {
        Some(ext) => {
            let ext = ext.to_ascii_lowercase();
            SUPPORTED_EXTS.iter().any(|e| *e == ext)
        }
        None => false,
    }
}

fn file_stem_string(p: &Path) -> String {
    p.file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or_default()
        .to_string()
}
