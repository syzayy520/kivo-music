use rusqlite::{params, Connection};

use super::{LibraryError, TrackHit};

pub(crate) fn search_prefix_with_conn(
    conn: &Connection,
    q: &str,
    limit: u32,
) -> Result<Vec<TrackHit>, LibraryError> {
    let q = q.trim();
    if q.is_empty() || limit == 0 {
        return Ok(Vec::new());
    }

    let pattern = format!("{}%", escape_like(q));

    let mut stmt = conn
        .prepare(
            "SELECT rowid, COALESCE(title, ''), path \
             FROM tracks \
             WHERE deleted=0 AND (path LIKE ?1 ESCAPE '\\' OR title LIKE ?1 ESCAPE '\\') \
             ORDER BY path \
             LIMIT ?2",
        )
        .map_err(|e| crate::library_db::errors::LibraryDbError::Sql {
            context: "search_prefix.prepare",
            source: e,
        })?;

    let mut rows = stmt.query(params![pattern, limit as i64]).map_err(|e| {
        crate::library_db::errors::LibraryDbError::Sql {
            context: "search_prefix.query",
            source: e,
        }
    })?;

    let mut out: Vec<TrackHit> = Vec::new();
    while let Some(row) =
        rows.next()
            .map_err(|e| crate::library_db::errors::LibraryDbError::Sql {
                context: "search_prefix.next",
                source: e,
            })?
    {
        let id: i64 = row
            .get(0)
            .map_err(|e| crate::library_db::errors::LibraryDbError::Sql {
                context: "search_prefix.rowid",
                source: e,
            })?;
        let title: String =
            row.get(1)
                .map_err(|e| crate::library_db::errors::LibraryDbError::Sql {
                    context: "search_prefix.title",
                    source: e,
                })?;
        let path: String =
            row.get(2)
                .map_err(|e| crate::library_db::errors::LibraryDbError::Sql {
                    context: "search_prefix.path",
                    source: e,
                })?;
        out.push(TrackHit {
            id: id.max(0) as u64,
            title,
            path,
        });
    }

    Ok(out)
}

fn escape_like(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '%' | '_' | '\\' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}
