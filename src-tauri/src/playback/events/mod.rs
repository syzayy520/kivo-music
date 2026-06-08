use serde::{Deserialize, Serialize};

use crate::playback::errors::PlaybackError;
use crate::playback::state::PlaybackState;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PlaybackEvent {
    StateChanged(PlaybackState),
    Progress {
        position_ms: u64,
        duration_ms: Option<u64>,
    },
    TrackChanged(PlaybackState),
    Error(PlaybackError),
}

pub const DEFAULT_PROGRESS_EVENT_INTERVAL_MS: u64 = 500;
