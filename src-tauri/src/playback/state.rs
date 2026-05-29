use serde::{Deserialize, Serialize};

use super::metadata::AudioMetadata;
use super::timeline::PlaybackTimeline;
use super::types::{PlaybackStatus, PlaybackTrack};
use super::volume::PlaybackVolume;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaybackState {
    pub status: PlaybackStatus,
    pub current_track: Option<PlaybackTrack>,
    pub timeline: PlaybackTimeline,
    pub volume: PlaybackVolume,
    pub metadata: Option<AudioMetadata>,
    pub error: Option<String>,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            status: PlaybackStatus::Idle,
            current_track: None,
            timeline: PlaybackTimeline::default(),
            volume: PlaybackVolume::default(),
            metadata: None,
            error: None,
        }
    }
}
