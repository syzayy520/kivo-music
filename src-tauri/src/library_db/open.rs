use std::{
    fs,
    path::{Path, PathBuf},
};

use rusqlite::Connection;
use tauri::Manager;

use super::{errors::LibraryDbError, schema};

/// Database file name (kept stable for forward migrations).
const DB_FILE_NAME: &str = "library.sqlite3";

/// Resolve the database path under AppData and ensure the parent directory exists.
fn db_path(app: &tauri::AppHandle) -> Result<PathBuf, LibraryDbError> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| LibraryDbError::PathResolve {
            context: "app_data_dir",
            source: e,
        })?;

    // We intentionally keep the DB under a product-scoped directory (same pattern as project.zip).
    // Uses `app.package_info().name` (crate/package name) to scope the directory.
    let product_dir = app_data.join(app.package_info().name.clone());
    let dir = product_dir.join("db");
    Ok(dir.join(DB_FILE_NAME))
}

/// Open (or create) the library database at the resolved AppData location, apply pragmas, then ensure schema.
pub fn open_db(app: &tauri::AppHandle) -> Result<Connection, LibraryDbError> {
    let path = db_path(app)?;
    open_db_at_path(&path)
}

/// Open (or create) the library database at a specific path (used by tests and by `open_db`).
pub fn open_db_at_path(path: &Path) -> Result<Connection, LibraryDbError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| LibraryDbError::Io {
            context: "create db dir",
            source: e,
        })?;
    }

    let conn = Connection::open(path).map_err(|e| LibraryDbError::Sql {
        context: "open connection",
        source: e,
    })?;

    // Pragmas: keep conservative and maintainable.
    // - WAL: better concurrency and crash resilience
    // - foreign_keys: keep referential integrity
    // - busy_timeout: reduce SQLITE_BUSY flakes under contention
    conn.execute_batch(
        r#"
        PRAGMA journal_mode=WAL;
        PRAGMA foreign_keys=ON;
        PRAGMA busy_timeout=5000;
        "#,
    )
    .map_err(|e| LibraryDbError::Sql {
        context: "apply pragmas",
        source: e,
    })?;

    schema::ensure_schema(&conn)?;
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_db_and_ensure_schema_does_not_panic() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("kivo_shell_libdb_test_{stamp}"));
        let path = dir.join("library.sqlite3");

        let conn = open_db_at_path(&path).expect("open_db_at_path");
        // Verify one key table exists.
        let table: String = conn
            .query_row(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='tracks'",
                [],
                |row| row.get(0),
            )
            .expect("tracks table exists");
        assert_eq!(table, "tracks");

        // Best-effort cleanup.
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
