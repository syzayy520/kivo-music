use serde::{Deserialize, Serialize};

use crate::playback::metadata::AudioMetadata;
use crate::playback::timeline::PlaybackTimeline;
use crate::playback::types::{PlaybackStatus, PlaybackTrack};
use crate::playback::volume::PlaybackVolume;

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

pub mod timeline;
pub mod volume;
