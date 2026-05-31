use serde::{Deserialize, Serialize};

use super::events::PlaybackEvent;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaybackActivityLogSnapshot {
    pub entries: Vec<PlaybackEvent>,
    pub entry_count: usize,
    pub limit: usize,
}

impl PlaybackActivityLogSnapshot {
    pub fn new(entries: Vec<PlaybackEvent>, limit: usize) -> Self {
        let entry_count = entries.len();

        Self {
            entries,
            entry_count,
            limit,
        }
    }
}
