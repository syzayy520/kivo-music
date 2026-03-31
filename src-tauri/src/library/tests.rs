use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::library_db::{open_db_at_path, scan::scan_incremental};

use super::{counts_with_conn, counts_with_conn_diagnostics};

fn unique_tmp_name(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{prefix}_{}_{}", std::process::id(), nanos)
}

fn touch_file(p: &PathBuf) {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    // Content doesn't matter for incremental scan; only metadata + extension are used.
    fs::write(p, b"").unwrap();
}

#[test]
fn library_counts_roots_is_real_from_db_meta() {
    let tmp = std::env::temp_dir().join(unique_tmp_name("kivo_lib_counts_roots"));
    let db_path = tmp.join("library.sqlite3");

    let mut conn = open_db_at_path(&db_path).expect("open db");

    let root1 = tmp.join("root_a");
    let root2 = tmp.join("root_b");

    fs::create_dir_all(&root1).unwrap();
    fs::create_dir_all(&root2).unwrap();

    touch_file(&root1.join("a.mp3"));
    touch_file(&root2.join("b.mp3"));

    let _summary = scan_incremental(&mut conn, vec![root1, root2]).expect("scan");

    let counts = counts_with_conn(&conn).expect("counts");
    assert_eq!(counts.roots, 2);

    // Best-effort cleanup (WAL may create extra files).
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn library_counts_roots_meta_missing_is_diagnosable_and_nonzero_when_db_has_tracks() {
    let tmp = std::env::temp_dir().join(unique_tmp_name("kivo_lib_counts_roots_meta_missing"));
    let db_path = tmp.join("library.sqlite3");

    let mut conn = open_db_at_path(&db_path).expect("open db");

    let root1 = tmp.join("root_a");
    let root2 = tmp.join("root_b");
    fs::create_dir_all(&root1).unwrap();
    fs::create_dir_all(&root2).unwrap();
    touch_file(&root1.join("a.mp3"));
    touch_file(&root2.join("b.mp3"));

    let _summary = scan_incremental(&mut conn, vec![root1, root2]).expect("scan");

    // Simulate a meta loss (e.g. manual DB edit, partial migration, or older DB).
    conn.execute("DELETE FROM meta WHERE key='library_roots_json'", [])
        .expect("delete meta");

    let (counts, diag) = counts_with_conn_diagnostics(&conn).expect("counts");
    assert_eq!(counts.tracks, 2);
    assert_eq!(
        counts.roots, 1,
        "roots must not silently become 0 when DB has data"
    );
    let diag = diag.expect("expected diagnostic for missing roots meta");
    assert!(
        diag.contains("roots meta missing"),
        "diag must contain reason; got: {diag}"
    );

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn library_counts_roots_meta_invalid_json_is_diagnosable_and_nonzero_when_db_has_tracks() {
    let tmp = std::env::temp_dir().join(unique_tmp_name("kivo_lib_counts_roots_meta_bad_json"));
    let db_path = tmp.join("library.sqlite3");

    let mut conn = open_db_at_path(&db_path).expect("open db");

    let root1 = tmp.join("root_a");
    let root2 = tmp.join("root_b");
    fs::create_dir_all(&root1).unwrap();
    fs::create_dir_all(&root2).unwrap();
    touch_file(&root1.join("a.mp3"));
    touch_file(&root2.join("b.mp3"));

    let _summary = scan_incremental(&mut conn, vec![root1, root2]).expect("scan");

    // Corrupt the JSON meta value intentionally.
    conn.execute(
        "INSERT INTO meta(key, value) VALUES('library_roots_json', 'not-json')          ON CONFLICT(key) DO UPDATE SET value=excluded.value;",
        [],
    )
    .expect("corrupt meta");

    let (counts, diag) = counts_with_conn_diagnostics(&conn).expect("counts");
    assert_eq!(counts.tracks, 2);
    assert_eq!(
        counts.roots, 1,
        "roots must not silently become 0 when DB has data"
    );
    let diag = diag.expect("expected diagnostic for invalid JSON");
    assert!(
        diag.contains("invalid JSON"),
        "diag must contain reason; got: {diag}"
    );

    let _ = fs::remove_dir_all(&tmp);
}
