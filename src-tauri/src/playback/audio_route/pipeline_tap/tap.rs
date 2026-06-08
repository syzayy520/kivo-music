use crate::playback::audio_route_integration::{AudioRouteIntegration, AudioRouteIntegrationError};
use crate::playback::output::AudioOutputFrame;

use super::config::AudioRoutePipelineTapConfig;
use super::error::{AudioRoutePipelineTapError, AudioRoutePipelineTapErrorKind};
use super::frame_kind::AudioRouteTapFrameKind;
use super::mapping;
use super::report::AudioRoutePipelineTapReport;

#[derive(Debug)]
pub struct AudioRoutePipelineTap {
    integration: AudioRouteIntegration,
    last_error: Option<AudioRoutePipelineTapErrorKind>,
}

impl AudioRoutePipelineTap {
    pub fn new(config: AudioRoutePipelineTapConfig) -> Result<Self, AudioRoutePipelineTapError> {
        let integration = AudioRouteIntegration::new(config.integration)
            .map_err(AudioRoutePipelineTapError::InvalidConfig)?;
        Ok(Self {
            integration,
            last_error: None,
        })
    }

    pub fn tap_output_frame(&mut self, frame: &AudioOutputFrame) -> AudioRoutePipelineTapReport {
        match self.integration.feed_output_frame(frame) {
            Ok(report) => {
                self.last_error = None;
                mapping::report_from_integration(
                    &report,
                    AudioRouteTapFrameKind::OutputFrame,
                    self.last_error,
                )
            }
            Err(error) => self.record_non_fatal_error(error),
        }
    }

    pub fn reset(&mut self) -> Result<AudioRoutePipelineTapReport, AudioRoutePipelineTapError> {
        let report = self
            .integration
            .reset_route()
            .map_err(mapping::map_integration_error)?;
        self.last_error = None;
        Ok(mapping::report_from_integration(
            &report,
            AudioRouteTapFrameKind::OutputFrame,
            self.last_error,
        ))
    }

    pub fn close(&mut self) -> AudioRoutePipelineTapReport {
        let report = self.integration.close();
        mapping::report_from_integration(
            &report,
            AudioRouteTapFrameKind::OutputFrame,
            self.last_error,
        )
    }

    pub fn report(&self) -> AudioRoutePipelineTapReport {
        mapping::report_from_integration(
            &self.integration.report(),
            AudioRouteTapFrameKind::OutputFrame,
            self.last_error,
        )
    }

    pub fn is_closed(&self) -> bool {
        self.integration.is_closed()
    }

    fn record_non_fatal_error(
        &mut self,
        error: AudioRouteIntegrationError,
    ) -> AudioRoutePipelineTapReport {
        self.last_error = Some(mapping::error_kind(&error));
        mapping::report_from_integration(
            &self.integration.report(),
            AudioRouteTapFrameKind::OutputFrame,
            self.last_error,
        )
    }
}
