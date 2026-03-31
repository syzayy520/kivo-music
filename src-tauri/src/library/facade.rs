use super::errors::LibraryError;
use super::scan::scan_roots;
use super::search::search_prefix as do_search;
use super::store;
use super::types::{LibraryCounts, ScanSummary, TrackHit};

pub fn run_scan(roots: Vec<String>) -> Result<ScanSummary, LibraryError> {
    if roots.is_empty() {
        return Err(LibraryError::InvalidInput("roots is empty"));
    }
    let (tracks, summary) = scan_roots(&roots);
    store::replace_all(roots, tracks);
    Ok(summary)
}

pub fn counts() -> LibraryCounts {
    store::counts()
}

pub fn search_prefix(q: String, limit: u32) -> Vec<TrackHit> {
    // keep deterministic upper bound for UI usage
    let lim = limit.min(200);
    let tracks = store::snapshot_tracks();
    do_search(&tracks, &q, lim)
}
