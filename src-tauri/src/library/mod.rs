use std::path::{Path, PathBuf};

use crate::library_db::{open_db_at_path, scan::scan_incremental};

pub mod commands;
mod counts;
pub mod errors;
mod search_db;
pub mod types;

#[cfg(test)]
mod tests;

pub use errors::LibraryError;
pub use types::{LibraryCounts, ScanSummary, TrackHit};

/// Run an incremental scan against an explicit DB path.
///
/// This is primarily for tests and tooling. App code should use the tauri commands
/// (which resolve the default DB location under the app data dir).
pub fn run_scan(db_path: &Path, roots: Vec<String>) -> Result<ScanSummary, LibraryError> {
    let roots = normalize_roots(roots)?;
    let mut db = open_db_at_path(db_path).map_err(LibraryError::from)?;
    scan_incremental(&mut db, roots).map_err(LibraryError::from)
}

pub fn counts(db_path: &Path) -> Result<LibraryCounts, LibraryError> {
    let db = open_db_at_path(db_path).map_err(LibraryError::from)?;
    counts::counts_with_conn(&db)
}

pub fn search_prefix(db_path: &Path, q: String, limit: u32) -> Result<Vec<TrackHit>, LibraryError> {
    let db = open_db_at_path(db_path).map_err(LibraryError::from)?;
    search_db::search_prefix_with_conn(&db, &q, limit)
}

fn normalize_roots(roots: Vec<String>) -> Result<Vec<PathBuf>, LibraryError> {
    let mut out = Vec::new();
    for r in roots {
        let r = r.trim();
        if r.is_empty() {
            continue;
        }
        out.push(PathBuf::from(r));
    }
    if out.is_empty() {
        return Err(LibraryError::InvalidRoots);
    }
    Ok(out)
}

pub(crate) use counts::counts_with_conn;
#[cfg(test)]
pub(crate) use counts::counts_with_conn_diagnostics;
pub(crate) use search_db::search_prefix_with_conn;
