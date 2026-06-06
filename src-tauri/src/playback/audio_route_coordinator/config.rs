use crate::playback::audio_route::AudioRouteConfig;
use crate::playback::decoder::AudioStreamInfo;

use super::error::AudioRouteCoordinatorError;

#[derive(Clone, Debug)]
pub struct AudioRouteCoordinatorConfig {
    pub route: AudioRouteConfig,
}

impl AudioRouteCoordinatorConfig {
    pub fn new(
        stream: AudioStreamInfo,
        capacity_frames: u32,
    ) -> Result<Self, AudioRouteCoordinatorError> {
        AudioRouteConfig::new(stream, capacity_frames)
            .map(Self::from_route)
            .map_err(AudioRouteCoordinatorError::InvalidConfig)
    }

    pub fn from_route(route: AudioRouteConfig) -> Self {
        Self { route }
    }
}
