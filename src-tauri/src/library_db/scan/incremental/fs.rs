use std::{
    collections::BTreeMap,
    ffi::OsStr,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FileStat {
    pub path: String,
    pub size: i64,
    pub mtime_ms: i64,
}

#[derive(Debug, Clone, Default)]
pub struct FsScanResult {
    /// Normalized prefixes (with trailing separator) used to scope deleted=1 marking.
    pub roots_for_delete: Vec<String>,
    /// Discovered audio files.
    pub files: Vec<FileStat>,
    /// Number of directories walked.
    pub scanned_dirs: u64,
    /// IO / permission errors encountered during traversal.
    pub io_errors: u64,
    /// Breakdown by file extension (lowercase).
    pub by_ext: BTreeMap<String, u64>,
}

/// Collect audio files and basic traversal stats.
pub fn collect_audio_files(roots: &[PathBuf]) -> std::io::Result<FsScanResult> {
    let mut res = FsScanResult::default();

    for root in roots {
        let root_str = root.to_string_lossy().to_string();
        // Data safety:
        // - Only mark a root eligible for "mark deleted" if we can actually traverse it.
        // - If the root can't be opened (NAS offline / removable missing / permission denied),
        //   we must NOT include it in roots_for_delete, otherwise we'd mass-mark tracks deleted.
        // - If traversal has ANY IO/metadata errors, we conservatively skip deletion for that root
        //   to avoid false deletions due to incomplete visibility.
        let mut root_opened = false;
        let mut root_is_dir = false;
        let mut root_had_errors = false;

        for entry in WalkDir::new(root).follow_links(false).into_iter() {
            let entry = match entry {
                Ok(e) => {
                    if !root_opened {
                        root_opened = true;
                        root_is_dir = e.file_type().is_dir();
                    }
                    e
                }
                Err(_) => {
                    res.io_errors += 1;
                    root_had_errors = true;
                    continue; // skip unreadable entries; scanner must not panic
                }
            };

            if entry.file_type().is_dir() {
                res.scanned_dirs += 1;
                continue;
            }

            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            let ext = audio_ext(path);
            let Some(ext) = ext else {
                continue;
            };

            let meta = match entry.metadata() {
                Ok(m) => m,
                Err(_) => {
                    res.io_errors += 1;
                    root_had_errors = true;
                    continue;
                }
            };

            let size = meta.len() as i64;
            let mtime_ms = meta
                .modified()
                .ok()
                .and_then(system_time_to_ms)
                .unwrap_or(0);

            *res.by_ext.entry(ext.clone()).or_insert(0) += 1;
            res.files.push(FileStat {
                path: path.to_string_lossy().to_string(),
                size,
                mtime_ms,
            });
        }

        if root_opened && root_is_dir && !root_had_errors {
            res.roots_for_delete.push(normalize_root_prefix(&root_str));
        }
    }

    Ok(res)
}

fn audio_ext(path: &Path) -> Option<String> {
    let ext = path
        .extension()
        .and_then(OsStr::to_str)?
        .to_ascii_lowercase();
    match ext.as_str() {
        "mp3" | "flac" | "m4a" | "aac" | "wav" | "ogg" => Some(ext),
        _ => None,
    }
}

fn system_time_to_ms(t: SystemTime) -> Option<i64> {
    t.duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_millis() as i64)
}

fn normalize_root_prefix(root: &str) -> String {
    // Avoid matching siblings (e.g., C:\Music2) when root is C:\Music.
    // We append a path separator if absent.
    let sep = std::path::MAIN_SEPARATOR;
    if root.ends_with(sep) || root.ends_with('/') || root.ends_with('\\') {
        root.to_string()
    } else {
        format!("{root}{sep}")
    }
}
