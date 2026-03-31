use super::types::{LibraryCounts, TrackHit};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Default)]
pub struct LibraryStore {
    pub roots: Vec<String>,
    pub tracks: Vec<TrackHit>,
}

static STORE: OnceLock<Mutex<LibraryStore>> = OnceLock::new();

pub fn with_store<T>(f: impl FnOnce(&mut LibraryStore) -> T) -> T {
    let m = STORE.get_or_init(|| Mutex::new(LibraryStore::default()));
    // Commercial-grade tolerance: if another thread panicked while holding the
    // lock, the Mutex becomes "poisoned". We intentionally keep the app alive
    // and continue with the inner state instead of panicking again.
    let mut g = m.lock().unwrap_or_else(|e| e.into_inner());
    f(&mut g)
}

pub fn counts() -> LibraryCounts {
    with_store(|s| LibraryCounts {
        roots: s.roots.len() as u64,
        tracks: s.tracks.len() as u64,
    })
}

pub fn snapshot_tracks() -> Vec<TrackHit> {
    with_store(|s| s.tracks.clone())
}

pub fn replace_all(roots: Vec<String>, tracks: Vec<TrackHit>) {
    with_store(|s| {
        s.roots = roots;
        s.tracks = tracks;
    })
}
