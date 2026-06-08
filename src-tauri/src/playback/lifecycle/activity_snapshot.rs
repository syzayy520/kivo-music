use serde::{Deserialize, Serialize};

use super::lifecycle_event::PlaybackLifecycleEvent;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaybackLifecycleActivityLogSnapshot {
    pub entries: Vec<PlaybackLifecycleEvent>,
    pub entry_count: usize,
    pub limit: usize,
}

impl PlaybackLifecycleActivityLogSnapshot {
    pub fn new(entries: Vec<PlaybackLifecycleEvent>, limit: usize) -> Self {
        let entry_count = entries.len();

        Self {
            entries,
            entry_count,
            limit,
        }
    }
}
