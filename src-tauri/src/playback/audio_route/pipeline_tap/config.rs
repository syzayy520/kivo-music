use crate::playback::audio_route_integration::AudioRouteIntegrationConfig;
use crate::playback::decoder::AudioStreamInfo;

use super::error::AudioRoutePipelineTapError;

#[derive(Clone, Debug)]
pub struct AudioRoutePipelineTapConfig {
    pub integration: AudioRouteIntegrationConfig,
}

impl AudioRoutePipelineTapConfig {
    pub fn new(
        stream: AudioStreamInfo,
        capacity_frames: u32,
    ) -> Result<Self, AudioRoutePipelineTapError> {
        AudioRouteIntegrationConfig::new(stream, capacity_frames)
            .map(Self::from_integration)
            .map_err(AudioRoutePipelineTapError::InvalidConfig)
    }

    pub fn from_integration(integration: AudioRouteIntegrationConfig) -> Self {
        Self { integration }
    }
}
