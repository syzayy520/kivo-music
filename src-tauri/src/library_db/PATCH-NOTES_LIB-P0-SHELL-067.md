# PATCH-NOTES — LIB-P0-SHELL-067 (Library DB foundation)

## Scope / constraints
- ✅ Only touched `src-tauri/src/library_db/**`, `src-tauri/Cargo.toml`, and a **1-line module export** in `src-tauri/src/lib.rs` (to compile/enable tests).
- ✅ No changes under `src/**`, `public/**`, or `src-tauri/src/audio/**`.

## What changed

### 1) Added: `src-tauri/src/library_db/`
- `mod.rs`
  - Module façade + re-exports: `open_db`, `open_db_at_path`, result/error types.
- `errors.rs`
  - `LibraryDbError` (thiserror) + `LibraryDbResult<T>`.
- `open.rs`
  - `open_db(app_handle)`:
    - resolves DB path under **AppData** via `app.path().app_data_dir()` (Tauri v2)
    - creates directory
    - opens connection
    - applies conservative pragmas: `WAL`, `foreign_keys=ON`, `busy_timeout=5000`
    - calls `schema::ensure_schema`
  - `open_db_at_path(path)` helper (used by tests and `open_db`)
  - Unit test: `open_db_at_path + ensure_schema` creates tables and does not panic.
- `schema/mod.rs`
  - `ensure_schema(conn)` + `PRAGMA user_version` versioning (`SCHEMA_VERSION = 1`).
  - Strategy: idempotent `CREATE TABLE IF NOT EXISTS ...` for v1, then bump `user_version` to 1.
- `schema/v1.rs`
  - Declarative SQL string for P0 tables.

### 2) Modified: `src-tauri/Cargo.toml`
- Added dependency (as required by ticket, pinned):
  - `rusqlite = { version = "0.32", features = ["bundled"] }`

### 3) Modified: `src-tauri/src/lib.rs`
- Added `pub mod library_db;` (required so the new module is compiled and the unit test runs).

## DB path strategy (Windows/AppData)
- Uses `app.path().app_data_dir()` (Tauri v2) → `<AppData>/<package_name>/db/library.sqlite3`
- Uses `app.package_info().name` (crate name; `kivo-shell` in this repo) as the product-scoped directory.

## Schema overview (P0)
- `meta(key TEXT PRIMARY KEY, value TEXT NOT NULL)`
- `artists(id INTEGER PK, name, sort_name, created_at, updated_at)`
- `albums(id INTEGER PK, title, album_artist, year, created_at, updated_at)`
- `tracks(id INTEGER PK, path UNIQUE, size, mtime_ms, deleted, indexed_at, ... optional columns, album_id FK)`
  - Required fields included: `path`, `size`, `mtime_ms`, `deleted`, `indexed_at`
- `track_artists(track_id, artist_id, role, ord, PK(...), FK -> tracks/artists)`

## How to verify (expected)
- `cd src-tauri`
- `cargo fmt`
- `cargo test`  (runs `open_db_and_ensure_schema_does_not_panic`)
- `cargo clippy -- -D warnings`
