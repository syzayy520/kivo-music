use crate::playback::audio_route_coordinator::AudioRouteCoordinatorConfig;
use crate::playback::decoder::AudioStreamInfo;

use super::error::AudioRouteIntegrationError;

#[derive(Clone, Debug)]
pub struct AudioRouteIntegrationConfig {
    pub coordinator: AudioRouteCoordinatorConfig,
}

impl AudioRouteIntegrationConfig {
    pub fn new(
        stream: AudioStreamInfo,
        capacity_frames: u32,
    ) -> Result<Self, AudioRouteIntegrationError> {
        AudioRouteCoordinatorConfig::new(stream, capacity_frames)
            .map(Self::from_coordinator)
            .map_err(AudioRouteIntegrationError::InvalidConfig)
    }

    pub fn from_coordinator(coordinator: AudioRouteCoordinatorConfig) -> Self {
        Self { coordinator }
    }
}
