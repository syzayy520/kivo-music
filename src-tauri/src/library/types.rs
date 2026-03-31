use serde::{Deserialize, Serialize};

pub use crate::library_db::scan::types::ScanSummary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryCounts {
    /// Number of configured roots (if known). If unknown, it will be 0.
    pub roots: u64,
    /// Number of non-deleted tracks in the DB.
    pub tracks: u64,
    /// Number of tracks marked deleted in the DB.
    pub deleted: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackHit {
    pub id: u64,
    pub title: String,
    pub path: String,
}
