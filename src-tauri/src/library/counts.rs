use rusqlite::{Connection, OptionalExtension};

use super::{LibraryCounts, LibraryError};

pub(crate) fn counts_with_conn(conn: &Connection) -> Result<LibraryCounts, LibraryError> {
    let (counts, diag) = counts_with_conn_diagnostics(conn)?;
    if let Some(msg) = diag {
        eprintln!("[library][warn] {msg}");
    }
    Ok(counts)
}

pub(crate) fn counts_with_conn_diagnostics(
    conn: &Connection,
) -> Result<(LibraryCounts, Option<String>), LibraryError> {
    let tracks: u64 = conn
        .query_row("SELECT COUNT(*) FROM tracks WHERE deleted=0", [], |r| {
            r.get::<_, i64>(0)
        })
        .map_err(|e| crate::library_db::errors::LibraryDbError::Sql {
            context: "counts(tracks)",
            source: e,
        })? as u64;

    let deleted: u64 = conn
        .query_row("SELECT COUNT(*) FROM tracks WHERE deleted=1", [], |r| {
            r.get::<_, i64>(0)
        })
        .map_err(|e| crate::library_db::errors::LibraryDbError::Sql {
            context: "counts(deleted)",
            source: e,
        })? as u64;

    let (roots, diag) = roots_count_from_meta(conn, tracks, deleted)?;

    Ok((
        LibraryCounts {
            roots,
            tracks,
            deleted,
        },
        diag,
    ))
}

fn roots_count_from_meta(
    conn: &Connection,
    tracks: u64,
    deleted: u64,
) -> Result<(u64, Option<String>), LibraryError> {
    let has_any_data = tracks > 0 || deleted > 0;

    let meta_opt: Option<String> = conn
        .query_row(
            "SELECT value FROM meta WHERE key='library_roots_json'",
            [],
            |r| r.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| crate::library_db::errors::LibraryDbError::Sql {
            context: "counts(roots_meta)",
            source: e,
        })?;

    let meta_val = match meta_opt {
        Some(v) => v,
        None => {
            if has_any_data {
                let msg = format!(
                    "counts: roots meta missing (key=library_roots_json); fallback roots=1 (unknown). tracks={tracks} deleted={deleted}"
                );
                return Ok((1, Some(msg)));
            }
            return Ok((0, None));
        }
    };

    match serde_json::from_str::<Vec<String>>(&meta_val) {
        Ok(v) => {
            if v.is_empty() && has_any_data {
                let msg = format!(
                    "counts: roots meta is empty array but DB has data; fallback roots=1 (unknown). tracks={tracks} deleted={deleted}"
                );
                Ok((1, Some(msg)))
            } else {
                Ok((v.len() as u64, None))
            }
        }
        Err(err) => {
            let fallback = if has_any_data { 1 } else { 0 };

            let mut snippet: String = meta_val.chars().take(120).collect();
            if meta_val.chars().count() > 120 {
                snippet.push_str("...");
            }

            let msg = format!(
                "counts: roots meta invalid JSON (key=library_roots_json); fallback roots={fallback} (unknown_as_one={has_any_data}). tracks={tracks} deleted={deleted}; value_snippet={snippet:?}; parse_error={err}"
            );
            Ok((fallback, Some(msg)))
        }
    }
}
