use serde::{Deserialize, Serialize};

use super::clock::current_timestamp_ms;
use super::events::PlaybackEvent;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaybackActivityLogEntry {
    pub event: PlaybackEvent,
    pub timestamp_ms: u64,
}

impl PlaybackActivityLogEntry {
    pub fn new(event: PlaybackEvent) -> Self {
        Self {
            event,
            timestamp_ms: current_timestamp_ms(),
        }
    }
}
