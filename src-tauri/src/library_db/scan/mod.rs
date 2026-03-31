//! Minimal incremental scanner for the library database.
//!
//! Core idea: use (path, size, mtime_ms) to decide upsert/unchanged, and mark unseen rows as deleted=1.

pub mod incremental;
pub mod types;

pub use incremental::scan_incremental;
