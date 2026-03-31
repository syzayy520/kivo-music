use tauri::AppHandle;

use super::errors::LibraryError;
use super::types::{LibraryCounts, ScanSummary, TrackHit};

#[tauri::command]
pub fn library_scan_run(app: AppHandle, roots: Vec<String>) -> Result<ScanSummary, String> {
    let roots =
        super::normalize_roots(roots).map_err(|e: LibraryError| format!("LIB_SCAN_FAILED: {e}"))?;
    let mut db =
        crate::library_db::open_db(&app).map_err(|e| format!("LIB_OPEN_DB_FAILED: {e}"))?;
    crate::library_db::scan::incremental::scan_incremental(&mut db, roots)
        .map_err(|e| format!("LIB_SCAN_FAILED: {e}"))
}

#[tauri::command]
pub fn library_counts(app: AppHandle) -> Result<LibraryCounts, String> {
    let db = crate::library_db::open_db(&app).map_err(|e| format!("LIB_OPEN_DB_FAILED: {e}"))?;
    super::counts_with_conn(&db).map_err(|e| format!("LIB_COUNTS_FAILED: {e}"))
}

#[tauri::command]
pub fn library_search_prefix(
    app: AppHandle,
    q: String,
    limit: u32,
) -> Result<Vec<TrackHit>, String> {
    let db = crate::library_db::open_db(&app).map_err(|e| format!("LIB_OPEN_DB_FAILED: {e}"))?;
    super::search_prefix_with_conn(&db, &q, limit).map_err(|e| format!("LIB_SEARCH_FAILED: {e}"))
}
