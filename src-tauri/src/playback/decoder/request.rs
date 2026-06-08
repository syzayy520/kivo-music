use serde::{Deserialize, Serialize};

use crate::playback::types::PlaybackTrack;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioDecoderOpenRequest {
    pub track_id: String,
    pub source_path: String,
}

impl AudioDecoderOpenRequest {
    pub fn from_track(track: &PlaybackTrack) -> Self {
        Self {
            track_id: track.id.0.clone(),
            source_path: track.source_path.clone(),
        }
    }
}
