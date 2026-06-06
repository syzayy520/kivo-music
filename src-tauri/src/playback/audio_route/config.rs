use crate::playback::decoder::AudioStreamInfo;

use super::error::AudioRouteError;
use super::format::derive_route_ring_buffer_format;

#[derive(Clone, Debug)]
pub struct AudioRouteConfig {
    pub stream: AudioStreamInfo,
    pub capacity_frames: u32,
}

impl AudioRouteConfig {
    pub fn new(stream: AudioStreamInfo, capacity_frames: u32) -> Result<Self, AudioRouteError> {
        let config = Self {
            stream,
            capacity_frames,
        };
        validate_route_config(&config)?;
        Ok(config)
    }
}

pub(super) fn validate_route_config(config: &AudioRouteConfig) -> Result<(), AudioRouteError> {
    if config.capacity_frames == 0 {
        return Err(AudioRouteError::InvalidCapacity);
    }
    derive_route_ring_buffer_format(&config.stream)?;
    Ok(())
}
