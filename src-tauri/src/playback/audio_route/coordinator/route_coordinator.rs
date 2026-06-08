use crate::playback::audio_bridge::PcmSourceChunk;
use crate::playback::audio_route::{AudioRouteError, AudioRouteOwner, AudioRouteReport};
use crate::playback::decoder::DecodedAudioFrame;
use crate::playback::output::AudioOutputFrame;

use super::config::AudioRouteCoordinatorConfig;
use super::error::AudioRouteCoordinatorError;
use super::mapping;
use super::report::AudioRouteCoordinatorReport;
use super::state::AudioRouteCoordinatorState;

#[derive(Debug)]
pub struct AudioRouteCoordinator {
    route: AudioRouteOwner,
    state: AudioRouteCoordinatorState,
}

impl AudioRouteCoordinator {
    pub fn new(config: AudioRouteCoordinatorConfig) -> Result<Self, AudioRouteCoordinatorError> {
        let route = AudioRouteOwner::new(config.route).map_err(mapping::map_config_error)?;
        Ok(Self {
            route,
            state: AudioRouteCoordinatorState::initialized(),
        })
    }

    pub fn feed_pcm_source_chunk(
        &mut self,
        chunk: PcmSourceChunk<'_>,
    ) -> Result<AudioRouteCoordinatorReport, AudioRouteCoordinatorError> {
        if self.is_closed() {
            return Err(AudioRouteCoordinatorError::Closed);
        }
        let result = self.route.feed_pcm_source_chunk(chunk);
        self.record_feed_result(result)
    }

    pub fn feed_decoded_frame(
        &mut self,
        frame: &DecodedAudioFrame,
    ) -> Result<AudioRouteCoordinatorReport, AudioRouteCoordinatorError> {
        if self.is_closed() {
            return Err(AudioRouteCoordinatorError::Closed);
        }
        let result = self.route.feed_decoded_frame(frame);
        self.record_feed_result(result)
    }

    pub fn feed_output_frame(
        &mut self,
        frame: &AudioOutputFrame,
    ) -> Result<AudioRouteCoordinatorReport, AudioRouteCoordinatorError> {
        if self.is_closed() {
            return Err(AudioRouteCoordinatorError::Closed);
        }
        let result = self.route.feed_output_frame(frame);
        self.record_feed_result(result)
    }

    pub fn reset_route(
        &mut self,
    ) -> Result<AudioRouteCoordinatorReport, AudioRouteCoordinatorError> {
        self.route.reset();
        self.state.reset();
        if self.route.is_closed() {
            self.state.mark_closed();
        }
        Ok(self.report())
    }

    pub fn close(&mut self) -> AudioRouteCoordinatorReport {
        self.route.close();
        self.state.mark_closed();
        self.report()
    }

    pub fn report(&self) -> AudioRouteCoordinatorReport {
        mapping::report_from_route(&self.route.report(), &self.state, self.is_closed())
    }

    pub fn is_closed(&self) -> bool {
        self.state.closed || self.route.is_closed()
    }

    fn record_feed_result(
        &mut self,
        result: Result<AudioRouteReport, AudioRouteError>,
    ) -> Result<AudioRouteCoordinatorReport, AudioRouteCoordinatorError> {
        match result {
            Ok(route_report) => {
                self.state.record_success(&route_report);
                Ok(mapping::report_from_route(
                    &route_report,
                    &self.state,
                    self.is_closed(),
                ))
            }
            Err(error) => {
                if mapping::route_error_is_backpressure(&error) {
                    self.state.record_backpressure();
                }
                if matches!(error, AudioRouteError::Closed) {
                    self.state.mark_closed();
                }
                Err(mapping::map_route_error(error))
            }
        }
    }
}
