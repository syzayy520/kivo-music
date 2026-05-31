use serde::{Deserialize, Serialize};

use super::decoder::AudioStreamInfo;
use super::decoder_request::AudioDecoderOpenRequest;
use super::decoder_runtime_state::DecoderRuntimeState;
use super::decoder_session::DecoderSession;
use super::errors::{PlaybackError, PlaybackResult};
use super::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings};
use super::playback_worker_command::PlaybackWorkerCommand;
use super::playback_worker_state::PlaybackWorkerState;
use super::playback_worker_transition;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NativePipelineState {
    pub decoder_request: Option<AudioDecoderOpenRequest>,
    pub decoder_session: Option<DecoderSession>,
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
            decoder_session: None,
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

    pub fn configure_decoder_open(
        &mut self,
        request: AudioDecoderOpenRequest,
        stream_info: AudioStreamInfo,
        opened_at_ms: u64,
    ) {
        self.state.decoder_state.begin_opening();
        self.state.decoder_request = Some(request.clone());
        self.state.decoder_session = Some(DecoderSession::from_open_request(
            &request,
            stream_info,
            opened_at_ms,
        ));
        self.state.decoder_state.mark_open();
    }

    pub fn update_decoder_position(&mut self, position_ms: u64) {
        if let Some(session) = self.state.decoder_session.as_mut() {
            session.update_position(position_ms);
        }
    }

    pub fn count_decoded_frame(&mut self) {
        if let Some(session) = self.state.decoder_session.as_mut() {
            session.count_frame();
        }
    }

    pub fn set_decoder_state(&mut self, state: DecoderRuntimeState) {
        self.state.decoder_state = state;
    }

    pub fn set_output_settings(&mut self, settings: OutputSettings) {
        self.state.output_settings = settings;
    }

    pub fn set_output_status(&mut self, status: OutputRuntimeStatus) {
        self.state.output_status = status;
    }

    pub fn note_frame_submitted(&mut self, _frame: &AudioOutputFrame) {
        self.state.output_status.pending_frames =
            self.state.output_status.pending_frames.saturating_add(1);
        self.state.output_status.is_active = true;
    }

    pub fn handle_worker_command(&mut self, command: &PlaybackWorkerCommand) -> PlaybackResult<()> {
        let operation = match command {
            PlaybackWorkerCommand::Load { .. } => "load",
            PlaybackWorkerCommand::Play => "play",
            PlaybackWorkerCommand::Pause => "pause",
            PlaybackWorkerCommand::Resume => "resume",
            PlaybackWorkerCommand::Stop => "stop",
            PlaybackWorkerCommand::Seek { .. } => "seek",
            PlaybackWorkerCommand::SetVolume { .. } => "set_volume",
            PlaybackWorkerCommand::SetMuted { .. } => "set_muted",
            PlaybackWorkerCommand::Shutdown => "shutdown",
        };

        Err(PlaybackError::UnsupportedOperation(format!(
            "native pipeline worker command {operation} is not implemented yet"
        )))
    }

    pub fn map_worker_state(
        &self,
        state: &PlaybackWorkerState,
        command: &PlaybackWorkerCommand,
    ) -> PlaybackWorkerState {
        let mut next = state.clone();
        playback_worker_transition::apply_command(&mut next, command);
        next
    }

    pub fn start(&mut self) -> PlaybackResult<()> {
        Err(PlaybackError::UnsupportedOperation(
            "native pipeline start is not implemented yet".to_string(),
        ))
    }

    pub fn schedule_decode_step(&mut self) -> PlaybackResult<()> {
        Err(PlaybackError::UnsupportedOperation(
            "native pipeline schedule_decode_step is not implemented yet".to_string(),
        ))
    }

    pub fn submit(&mut self) -> PlaybackResult<()> {
        Err(PlaybackError::UnsupportedOperation(
            "native pipeline submit is not implemented yet".to_string(),
        ))
    }

    pub fn schedule_output_submit_step(&mut self) -> PlaybackResult<()> {
        Err(PlaybackError::UnsupportedOperation(
            "native pipeline schedule_output_submit_step is not implemented yet".to_string(),
        ))
    }

    pub fn shutdown(&mut self) -> PlaybackResult<()> {
        Err(PlaybackError::UnsupportedOperation(
            "native pipeline shutdown is not implemented yet".to_string(),
        ))
    }
}
