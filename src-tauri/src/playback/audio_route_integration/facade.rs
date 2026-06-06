use crate::playback::audio_bridge::PcmSourceChunk;
use crate::playback::audio_route_coordinator::{
    AudioRouteCoordinator, AudioRouteCoordinatorError, AudioRouteCoordinatorReport,
};
use crate::playback::decoder::DecodedAudioFrame;
use crate::playback::output::AudioOutputFrame;

use super::config::AudioRouteIntegrationConfig;
use super::error::AudioRouteIntegrationError;
use super::frame_input::AudioRouteFrameInput;
use super::mapping;
use super::report::AudioRouteIntegrationReport;
use super::state::AudioRouteIntegrationState;

#[derive(Debug)]
pub struct AudioRouteIntegration {
    coordinator: AudioRouteCoordinator,
    state: AudioRouteIntegrationState,
}

impl AudioRouteIntegration {
    pub fn new(config: AudioRouteIntegrationConfig) -> Result<Self, AudioRouteIntegrationError> {
        let coordinator =
            AudioRouteCoordinator::new(config.coordinator).map_err(mapping::map_config_error)?;
        Ok(Self {
            coordinator,
            state: AudioRouteIntegrationState::initialized(),
        })
    }

    pub fn feed_pcm_source_chunk(
        &mut self,
        chunk: PcmSourceChunk<'_>,
    ) -> Result<AudioRouteIntegrationReport, AudioRouteIntegrationError> {
        if self.is_closed() {
            return Err(AudioRouteIntegrationError::Closed);
        }
        let result = self.coordinator.feed_pcm_source_chunk(chunk);
        self.record_feed_result(result)
    }

    pub fn feed_decoded_frame(
        &mut self,
        frame: &DecodedAudioFrame,
    ) -> Result<AudioRouteIntegrationReport, AudioRouteIntegrationError> {
        self.feed_frame_input(AudioRouteFrameInput::DecodedFrame(frame))
    }

    pub fn feed_output_frame(
        &mut self,
        frame: &AudioOutputFrame,
    ) -> Result<AudioRouteIntegrationReport, AudioRouteIntegrationError> {
        self.feed_frame_input(AudioRouteFrameInput::OutputFrame(frame))
    }

    pub fn feed_frame_input(
        &mut self,
        input: AudioRouteFrameInput<'_>,
    ) -> Result<AudioRouteIntegrationReport, AudioRouteIntegrationError> {
        if self.is_closed() {
            return Err(AudioRouteIntegrationError::Closed);
        }
        let result = match input {
            AudioRouteFrameInput::PcmSourceChunk(chunk) => {
                self.coordinator.feed_pcm_source_chunk(chunk)
            }
            AudioRouteFrameInput::DecodedFrame(frame) => self.coordinator.feed_decoded_frame(frame),
            AudioRouteFrameInput::OutputFrame(frame) => self.coordinator.feed_output_frame(frame),
        };
        self.record_feed_result(result)
    }

    pub fn reset_route(
        &mut self,
    ) -> Result<AudioRouteIntegrationReport, AudioRouteIntegrationError> {
        let coordinator_report = self
            .coordinator
            .reset_route()
            .map_err(mapping::map_coordinator_error)?;
        self.state.reset();
        if self.coordinator.is_closed() {
            self.state.mark_closed();
        }
        Ok(mapping::report_from_coordinator(
            &coordinator_report,
            &self.state,
            self.is_closed(),
        ))
    }

    pub fn close(&mut self) -> AudioRouteIntegrationReport {
        let coordinator_report = self.coordinator.close();
        self.state.mark_closed();
        mapping::report_from_coordinator(&coordinator_report, &self.state, true)
    }

    pub fn report(&self) -> AudioRouteIntegrationReport {
        mapping::report_from_coordinator(&self.coordinator.report(), &self.state, self.is_closed())
    }

    pub fn is_closed(&self) -> bool {
        self.state.closed || self.coordinator.is_closed()
    }

    fn record_feed_result(
        &mut self,
        result: Result<AudioRouteCoordinatorReport, AudioRouteCoordinatorError>,
    ) -> Result<AudioRouteIntegrationReport, AudioRouteIntegrationError> {
        match result {
            Ok(coordinator_report) => {
                self.state.record_successful_input();
                Ok(mapping::report_from_coordinator(
                    &coordinator_report,
                    &self.state,
                    self.is_closed(),
                ))
            }
            Err(error) => {
                if matches!(error, AudioRouteCoordinatorError::Closed) {
                    self.state.mark_closed();
                }
                Err(mapping::map_coordinator_error(error))
            }
        }
    }
}
