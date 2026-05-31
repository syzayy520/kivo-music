use serde::{Deserialize, Serialize};

use super::event::MediaProbeEvent;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaProbeActivityLogSnapshot {
    pub entries: Vec<MediaProbeEvent>,
    pub entry_count: usize,
    pub limit: usize,
}

impl MediaProbeActivityLogSnapshot {
    pub fn new(entries: Vec<MediaProbeEvent>, limit: usize) -> Self {
        let entry_count = entries.len();

        Self {
            entries,
            entry_count,
            limit,
        }
    }
}
