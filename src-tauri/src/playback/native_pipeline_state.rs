use serde::{Deserialize, Serialize};

use super::decoder_request::AudioDecoderOpenRequest;
use super::decoder_runtime_state::DecoderRuntimeState;
use super::decoder_session::DecoderSession;
use super::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NativePipelineState {
    pub decoder_request: Option<AudioDecoderOpenRequest>,
    pub decoder_session: Option<DecoderSession>,
    pub decoder_state: DecoderRuntimeState,
    pub output_settings: OutputSettings,
    pub output_status: OutputRuntimeStatus,
    pub last_decoded_frame: Option<AudioOutputFrame>,
}

impl Default for NativePipelineState {
    fn default() -> Self {
        Self {
            decoder_request: None,
            decoder_session: None,
            decoder_state: DecoderRuntimeState::idle(),
            output_settings: OutputSettings::default(),
            output_status: OutputRuntimeStatus::default(),
            last_decoded_frame: None,
        }
    }
}
