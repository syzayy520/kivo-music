use serde::{Deserialize, Serialize};

use super::types::{PlaybackTrack, RepeatMode};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlaybackQueue {
    pub items: Vec<PlaybackTrack>,
    pub current_index: Option<usize>,
    pub repeat_mode: RepeatMode,
    pub shuffle: bool,
}

impl Default for RepeatMode {
    fn default() -> Self {
        Self::Off
    }
}
