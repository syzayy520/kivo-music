use serde::{Deserialize, Serialize};

use super::decoder_request::AudioDecoderOpenRequest;
use super::decoder_runtime_state::DecoderRuntimeState;
use super::errors::{PlaybackError, PlaybackResult};
use super::output::{OutputRuntimeStatus, OutputSettings};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NativePipelineState {
    pub decoder_request: Option<AudioDecoderOpenRequest>,
    pub decoder_state: DecoderRuntimeState,
    pub output_settings: OutputSettings,
    pub output_status: OutputRuntimeStatus,
}

#[derive(Clone, Debug, Default)]
pub struct NativePipeline {
    state: NativePipelineState,
}

impl Default for NativePipelineState {
    fn default() -> Self {
        Self {
            decoder_request: None,
            decoder_state: DecoderRuntimeState::idle(),
            output_settings: OutputSettings::default(),
            output_status: OutputRuntimeStatus::default(),
        }
    }
}

impl NativePipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> NativePipelineState {
        self.state.clone()
    }

    pub fn set_decoder_request(&mut self, request: AudioDecoderOpenRequest) {
        self.state.decoder_request = Some(request);
    }

    pub fn set_decoder_state(&mut self, state: DecoderRuntimeState) {
        self.state.decoder_state = state;
    }

    pub fn set_output_settings(&mut self, settings: OutputSettings) {
        self.state.output_settings = settings;
    }

    pub fn start(&mut self) -> PlaybackResult<()> {
        Err(PlaybackError::UnsupportedOperation(
            "native pipeline start is not implemented yet".to_string(),
        ))
    }

    pub fn submit(&mut self) -> PlaybackResult<()> {
        Err(PlaybackError::UnsupportedOperation(
            "native pipeline submit is not implemented yet".to_string(),
        ))
    }

    pub fn shutdown(&mut self) -> PlaybackResult<()> {
        Err(PlaybackError::UnsupportedOperation(
            "native pipeline shutdown is not implemented yet".to_string(),
        ))
    }
}
