use std::{
    path::PathBuf,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use rusqlite::{params, Connection};

use super::super::errors::LibraryDbError;
use super::types::ScanSummary;
use crate::library_db::errors::LibraryDbResult;

mod db;
mod fs;

#[cfg(test)]
mod tests;

pub fn scan_incremental(
    conn: &mut Connection,
    roots: Vec<PathBuf>,
) -> LibraryDbResult<ScanSummary> {
    if roots.is_empty() {
        return Ok(ScanSummary::default());
    }

    let started = Instant::now();
    let roots_str: Vec<String> = roots
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    let fs_res = fs::collect_audio_files(&roots).map_err(|e| LibraryDbError::Io {
        context: "collect_audio_files",
        source: e,
    })?;

    let scanned_files = fs_res.files.len() as u64;
    let scanned_dirs = fs_res.scanned_dirs;
    let io_errors = fs_res.io_errors;
    let by_ext = fs_res.by_ext.clone();
    let now_ms = now_ms();

    let tx = conn.transaction().map_err(|e| LibraryDbError::Sql {
        context: "transaction",
        source: e,
    })?;

    db::ensure_temp_seen_table(&tx).map_err(|e| LibraryDbError::Sql {
        context: "ensure_temp_seen_table",
        source: e,
    })?;
    db::clear_temp_seen_table(&tx).map_err(|e| LibraryDbError::Sql {
        context: "clear_temp_seen_table",
        source: e,
    })?;

    // Persist configured roots for `LibraryCounts.roots`.
    // We store the *requested* roots (strings) in meta as JSON so counts can be computed without
    // inferring prefixes from paths (schema v1 does not have a dedicated roots table/column).
    let roots_json = serde_json::to_string(&roots_str).unwrap_or_else(|_| "[]".to_string());
    tx.execute(
        "INSERT INTO meta(key, value) VALUES('library_roots_json', ?1)          ON CONFLICT(key) DO UPDATE SET value=excluded.value;",
        params![roots_json],
    )
    .map_err(|e| LibraryDbError::Sql {
        context: "persist_roots_meta",
        source: e,
    })?;

    let mut inserted = 0u64;
    let mut updated = 0u64;
    let mut unchanged = 0u64;
    let marked_deleted: u64;

    {
        let mut ctx = db::DbContext::new(&tx).map_err(|e| LibraryDbError::Sql {
            context: "prepare_statements",
            source: e,
        })?;

        for f in fs_res.files {
            db::seen_insert(&mut ctx, &f.path).map_err(|e| LibraryDbError::Sql {
                context: "seen_insert",
                source: e,
            })?;

            match db::track_get_meta(&mut ctx, &f.path).map_err(|e| LibraryDbError::Sql {
                context: "track_get_meta",
                source: e,
            })? {
                None => {
                    inserted += 1;
                    db::track_upsert(&mut ctx, &f.path, f.size, f.mtime_ms, now_ms, now_ms)
                        .map_err(|e| LibraryDbError::Sql {
                            context: "track_upsert(insert)",
                            source: e,
                        })?;
                }
                Some(existing) => {
                    if existing.deleted != 0
                        || existing.size != f.size
                        || existing.mtime_ms != f.mtime_ms
                    {
                        updated += 1;
                        db::track_upsert(&mut ctx, &f.path, f.size, f.mtime_ms, now_ms, now_ms)
                            .map_err(|e| LibraryDbError::Sql {
                                context: "track_upsert(update)",
                                source: e,
                            })?;
                    } else {
                        unchanged += 1;
                    }
                }
            }
        }

        marked_deleted =
            db::mark_deleted_under_roots_not_seen(&mut ctx, &fs_res.roots_for_delete, now_ms)
                .map_err(|e| LibraryDbError::Sql {
                    context: "mark_deleted_under_roots_not_seen",
                    source: e,
                })? as u64;
    }
    tx.commit().map_err(|e| LibraryDbError::Sql {
        context: "commit",
        source: e,
    })?;

    let elapsed_ms = started.elapsed().as_millis() as u64;

    Ok(ScanSummary {
        roots: roots_str,
        scanned_dirs,
        scanned_files,
        io_errors,
        by_ext,
        inserted,
        updated,
        upserts: inserted + updated,
        unchanged,
        marked_deleted,
        elapsed_ms,
    })
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
