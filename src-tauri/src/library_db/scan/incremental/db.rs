use rusqlite::{params, types::Value, OptionalExtension, Statement, Transaction};

#[derive(Debug, Clone)]
pub struct TrackMeta {
    pub size: i64,
    pub mtime_ms: i64,
    pub deleted: i64,
}

pub struct DbContext<'a> {
    pub tx: &'a Transaction<'a>,
    stmt_seen_insert: Statement<'a>,
    stmt_track_get_meta: Statement<'a>,
    stmt_track_upsert: Statement<'a>,
}

impl<'a> DbContext<'a> {
    pub fn new(tx: &'a Transaction<'a>) -> rusqlite::Result<Self> {
        let stmt_seen_insert = tx.prepare("INSERT OR IGNORE INTO scan_seen(path) VALUES (?1)")?;
        let stmt_track_get_meta =
            tx.prepare("SELECT size, mtime_ms, deleted FROM tracks WHERE path = ?1")?;
        let stmt_track_upsert = tx.prepare(
            "INSERT INTO tracks(path, size, mtime_ms, indexed_at, deleted, created_at, updated_at)              VALUES (?1, ?2, ?3, ?4, 0, ?5, ?5)              ON CONFLICT(path) DO UPDATE SET                size=excluded.size,                mtime_ms=excluded.mtime_ms,                indexed_at=excluded.indexed_at,                deleted=0,                updated_at=excluded.updated_at",
        )?;

        Ok(Self {
            tx,
            stmt_seen_insert,
            stmt_track_get_meta,
            stmt_track_upsert,
        })
    }
}

pub fn ensure_temp_seen_table(tx: &Transaction) -> rusqlite::Result<()> {
    tx.execute_batch("CREATE TEMP TABLE IF NOT EXISTS scan_seen(path TEXT PRIMARY KEY);")?;
    Ok(())
}

pub fn clear_temp_seen_table(tx: &Transaction) -> rusqlite::Result<()> {
    tx.execute("DELETE FROM scan_seen;", [])?;
    Ok(())
}

pub fn seen_insert(ctx: &mut DbContext<'_>, path: &str) -> rusqlite::Result<()> {
    ctx.stmt_seen_insert.execute(params![path])?;
    Ok(())
}

pub fn track_get_meta(ctx: &mut DbContext<'_>, path: &str) -> rusqlite::Result<Option<TrackMeta>> {
    ctx.stmt_track_get_meta
        .query_row(params![path], |row| {
            Ok(TrackMeta {
                size: row.get(0)?,
                mtime_ms: row.get(1)?,
                deleted: row.get(2)?,
            })
        })
        .optional()
}

pub fn track_upsert(
    ctx: &mut DbContext<'_>,
    path: &str,
    size: i64,
    mtime_ms: i64,
    indexed_at: i64,
    now_ms: i64,
) -> rusqlite::Result<()> {
    ctx.stmt_track_upsert
        .execute(params![path, size, mtime_ms, indexed_at, now_ms])?;
    Ok(())
}

pub fn mark_deleted_under_roots_not_seen(
    ctx: &mut DbContext<'_>,
    roots_for_delete: &[String],
    now_ms: i64,
) -> rusqlite::Result<usize> {
    if roots_for_delete.is_empty() {
        return Ok(0);
    }

    // UPDATE tracks SET deleted=1, updated_at=? WHERE deleted=0 AND (path LIKE ? OR ...) AND path NOT IN (seen)
    let mut sql = String::from("UPDATE tracks SET deleted=1, updated_at=? WHERE deleted=0 AND (");
    for i in 0..roots_for_delete.len() {
        if i > 0 {
            sql.push_str(" OR ");
        }
        sql.push_str("path LIKE ?");
    }
    sql.push_str(") AND path NOT IN (SELECT path FROM scan_seen);");

    let mut params_vec: Vec<Value> = Vec::with_capacity(1 + roots_for_delete.len());
    params_vec.push(Value::Integer(now_ms));
    for r in roots_for_delete {
        params_vec.push(Value::Text(format!("{r}%")));
    }

    let mut stmt = ctx.tx.prepare(&sql)?;
    let changed = stmt.execute(rusqlite::params_from_iter(params_vec))?;
    Ok(changed)
}
