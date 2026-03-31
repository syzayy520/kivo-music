use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use rusqlite::{params, Connection};

use super::scan_incremental;

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

fn unique_tmp_dir_name(prefix: &str) -> String {
    // Simple uniqueness without extra deps.
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{prefix}_{}_{}", std::process::id(), nanos)
}

fn create_tracks_table(conn: &Connection) {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS meta(\
            key TEXT PRIMARY KEY,\
            value TEXT NOT NULL\
        );\
        CREATE TABLE tracks(\
            path TEXT PRIMARY KEY,\
            size INTEGER NOT NULL,\
            mtime_ms INTEGER NOT NULL,\
            indexed_at INTEGER NOT NULL,\
            deleted INTEGER NOT NULL,\
            created_at INTEGER NOT NULL,\
            updated_at INTEGER NOT NULL\
        );",
    )
    .unwrap();
}

fn insert_track(conn: &Connection, path: &str, deleted: i64) {
    let t = now_ms();
    conn.execute(
        "INSERT INTO tracks(path, size, mtime_ms, indexed_at, deleted, created_at, updated_at)\
         VALUES(?1, 123, 456, ?2, ?3, ?2, ?2);",
        params![path, t, deleted],
    )
    .unwrap();
}

fn get_deleted(conn: &Connection, path: &str) -> i64 {
    conn.query_row(
        "SELECT deleted FROM tracks WHERE path=?1;",
        params![path],
        |row| row.get(0),
    )
    .unwrap()
}

#[test]
fn root_missing_does_not_mass_mark_deleted() {
    let mut conn = Connection::open_in_memory().unwrap();
    create_tracks_table(&conn);

    // Root does not exist on disk.
    let missing_root = std::env::temp_dir().join(unique_tmp_dir_name("kivo_missing_root"));
    let track_path = missing_root.join("song.mp3");
    let track_path_str = track_path.to_string_lossy().to_string();
    insert_track(&conn, &track_path_str, 0);

    let summary = scan_incremental(&mut conn, vec![missing_root]).unwrap();

    // Safety invariant: a root we cannot open/traverse must NOT participate in deletion marking.
    assert_eq!(summary.marked_deleted, 0);
    assert_eq!(get_deleted(&conn, &track_path_str), 0);
}

#[test]
fn root_accessible_empty_still_marks_deleted() {
    let mut conn = Connection::open_in_memory().unwrap();
    create_tracks_table(&conn);

    let root = std::env::temp_dir().join(unique_tmp_dir_name("kivo_empty_root"));
    fs::create_dir_all(&root).unwrap();

    let track_path = root.join("song.mp3");
    let track_path_str = track_path.to_string_lossy().to_string();
    insert_track(&conn, &track_path_str, 0);

    let summary = scan_incremental(&mut conn, vec![root.clone()]).unwrap();

    assert_eq!(summary.marked_deleted, 1);
    assert_eq!(get_deleted(&conn, &track_path_str), 1);

    // Best-effort cleanup.
    let _ = fs::remove_dir_all(&root);
}
