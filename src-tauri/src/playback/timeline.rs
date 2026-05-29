use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaybackTimeline {
    pub position_ms: u64,
    pub duration_ms: Option<u64>,
    pub progress_event_interval_ms: u64,
}

impl Default for PlaybackTimeline {
    fn default() -> Self {
        Self {
            position_ms: 0,
            duration_ms: None,
            progress_event_interval_ms: 500,
        }
    }
}
