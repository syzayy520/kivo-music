use crate::playback::audio_bridge::PcmSourceChunk;
use crate::playback::decoder::DecodedAudioFrame;
use crate::playback::output::AudioOutputFrame;
use crate::playback::output_wasapi::ring_buffer::RingBuffer;

use super::config::{validate_route_config, AudioRouteConfig};
use super::error::AudioRouteError;
use super::feeder;
use super::format::derive_route_ring_buffer_format;
use super::report::AudioRouteReport;

#[derive(Debug)]
pub struct AudioRouteOwner {
    pub(super) ring_buffer: RingBuffer,
    pub(super) report: AudioRouteReport,
    pub(super) closed: bool,
}

impl AudioRouteOwner {
    pub fn new(config: AudioRouteConfig) -> Result<Self, AudioRouteError> {
        validate_route_config(&config)?;
        let format = derive_route_ring_buffer_format(&config.stream)?;
        let ring_buffer = RingBuffer::new(format, config.capacity_frames)
            .map_err(AudioRouteError::RingBufferInitFailed)?;
        Ok(Self {
            ring_buffer,
            report: AudioRouteReport::initialized(config.capacity_frames),
            closed: false,
        })
    }

    pub fn feed_pcm_source_chunk(
        &mut self,
        chunk: PcmSourceChunk<'_>,
    ) -> Result<AudioRouteReport, AudioRouteError> {
        feeder::feed_pcm_source_chunk(self, chunk)
    }

    pub fn feed_decoded_frame(
        &mut self,
        frame: &DecodedAudioFrame,
    ) -> Result<AudioRouteReport, AudioRouteError> {
        feeder::feed_decoded_frame(self, frame)
    }

    pub fn feed_output_frame(
        &mut self,
        frame: &AudioOutputFrame,
    ) -> Result<AudioRouteReport, AudioRouteError> {
        feeder::feed_output_frame(self, frame)
    }

    pub fn report(&self) -> AudioRouteReport {
        self.report
    }

    pub fn pending_frames(&self) -> u32 {
        self.ring_buffer.available_frames()
    }

    pub fn capacity_frames(&self) -> u32 {
        self.ring_buffer.capacity_frames()
    }

    pub fn reset(&mut self) {
        self.ring_buffer.reset();
        self.report.reset();
    }

    pub fn close(&mut self) {
        self.ring_buffer.close();
        self.closed = true;
    }

    pub fn is_closed(&self) -> bool {
        self.closed || self.ring_buffer.is_closed()
    }
}
