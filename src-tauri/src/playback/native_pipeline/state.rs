use serde::{Deserialize, Serialize};

use crate::playback::decoder_request::AudioDecoderOpenRequest;
use crate::playback::decoder_runtime_state::DecoderRuntimeState;
use crate::playback::decoder_session::DecoderSession;
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(in crate::playback) struct NativePipelineState {
    pub(in crate::playback) decoder_request: Option<AudioDecoderOpenRequest>,
    pub(in crate::playback) decoder_session: Option<DecoderSession>,
    pub(in crate::playback) decoder_state: DecoderRuntimeState,
    pub(in crate::playback) output_settings: OutputSettings,
    pub(in crate::playback) output_status: OutputRuntimeStatus,
    pub(in crate::playback) last_decoded_frame: Option<AudioOutputFrame>,
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
