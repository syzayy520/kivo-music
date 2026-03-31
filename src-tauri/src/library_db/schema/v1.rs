//! v1 schema (pure SQL).
//!
//! Keep this file declarative: one SQL string, no logic.

pub const SQL: &str = r#"
CREATE TABLE IF NOT EXISTS meta (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS artists (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  name       TEXT NOT NULL COLLATE NOCASE,
  sort_name  TEXT COLLATE NOCASE,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS albums (
  id           INTEGER PRIMARY KEY AUTOINCREMENT,
  title        TEXT NOT NULL COLLATE NOCASE,
  album_artist TEXT COLLATE NOCASE,
  year         INTEGER,
  created_at   INTEGER NOT NULL,
  updated_at   INTEGER NOT NULL
);

-- Tracks are keyed by an integer id for stable foreign keys. The file path is UNIQUE.
CREATE TABLE IF NOT EXISTS tracks (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  path       TEXT NOT NULL UNIQUE,
  size       INTEGER NOT NULL,
  mtime_ms   INTEGER NOT NULL,

  title      TEXT COLLATE NOCASE,
  duration_ms INTEGER,
  sample_rate INTEGER,
  channels    INTEGER,

  album_id   INTEGER,
  deleted    INTEGER NOT NULL DEFAULT 0,
  indexed_at INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY(album_id) REFERENCES albums(id)
);

CREATE TABLE IF NOT EXISTS track_artists (
  track_id  INTEGER NOT NULL,
  artist_id INTEGER NOT NULL,
  role      TEXT NOT NULL DEFAULT 'main',
  ord       INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY(track_id, artist_id, role, ord),
  FOREIGN KEY(track_id) REFERENCES tracks(id) ON DELETE CASCADE,
  FOREIGN KEY(artist_id) REFERENCES artists(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_tracks_deleted ON tracks(deleted);
CREATE INDEX IF NOT EXISTS idx_tracks_album_id ON tracks(album_id);
CREATE INDEX IF NOT EXISTS idx_track_artists_artist_id ON track_artists(artist_id);
"#;
