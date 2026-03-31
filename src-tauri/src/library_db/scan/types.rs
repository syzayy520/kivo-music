use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Summary of a scan run.
///
/// Notes:
/// - `inserted`/`updated` are counts of rows that required an upsert.
/// - `upserts` is kept as a convenience (= inserted + updated).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScanSummary {
    /// Roots (as strings) that were requested for this scan run.
    pub roots: Vec<String>,

    /// Number of directories walked.
    pub scanned_dirs: u64,

    /// Number of audio files discovered by the scanner.
    pub scanned_files: u64,

    /// IO / permission errors encountered during traversal.
    pub io_errors: u64,

    /// Breakdown by file extension (lowercase).
    pub by_ext: BTreeMap<String, u64>,

    /// Number of rows inserted (did not exist before).
    pub inserted: u64,

    /// Number of rows updated (existed, and size/mtime changed OR was previously deleted).
    pub updated: u64,

    /// Convenience: inserted + updated.
    pub upserts: u64,

    /// Number of files that matched existing DB entries (no change).
    pub unchanged: u64,

    /// Number of rows marked as deleted.
    pub marked_deleted: u64,

    /// Total elapsed time for the scan.
    pub elapsed_ms: u64,
}
