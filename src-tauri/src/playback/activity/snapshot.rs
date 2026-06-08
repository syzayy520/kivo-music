use serde::{Deserialize, Serialize};

use crate::playback::activity_entry::PlaybackActivityLogEntry;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaybackActivityLogSnapshot {
    pub entries: Vec<PlaybackActivityLogEntry>,
    pub entry_count: usize,
    pub limit: usize,
}

impl PlaybackActivityLogSnapshot {
    pub fn new(entries: Vec<PlaybackActivityLogEntry>, limit: usize) -> Self {
        let entry_count = entries.len();

        Self {
            entries,
            entry_count,
            limit,
        }
    }
}
