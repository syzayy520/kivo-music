//! Schema and migrations for the library database.
//!
//! P0: a single schema version (v1) with a conservative, forward-migratable layout.

use rusqlite::Connection;

use super::errors::LibraryDbError;

mod v1;

const SCHEMA_VERSION: i32 = 1;

/// Ensure the schema exists and is at the expected version.
///
/// Strategy:
/// - Always run `CREATE TABLE IF NOT EXISTS ...` for v1 (idempotent).
/// - Track version via `PRAGMA user_version`.
pub fn ensure_schema(conn: &Connection) -> Result<(), LibraryDbError> {
    let user_version: i32 = conn
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|e| LibraryDbError::Sql {
            context: "read user_version",
            source: e,
        })?;

    if user_version > SCHEMA_VERSION {
        return Err(LibraryDbError::Migration(format!(
            "unsupported schema version {user_version} (max supported {SCHEMA_VERSION})"
        )));
    }

    // Idempotent schema init.
    conn.execute_batch(v1::SQL)
        .map_err(|e| LibraryDbError::Sql {
            context: "init schema v1",
            source: e,
        })?;

    // Bump user_version if needed.
    if user_version < SCHEMA_VERSION {
        conn.pragma_update(None, "user_version", SCHEMA_VERSION)
            .map_err(|e| LibraryDbError::Sql {
                context: "set user_version",
                source: e,
            })?;
    }

    Ok(())
}
