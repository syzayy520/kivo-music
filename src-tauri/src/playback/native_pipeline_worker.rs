use super::decoder_request::AudioDecoderOpenRequest;
use super::errors::{PlaybackError, PlaybackResult};
use super::native_pipeline::NativePipeline;
use super::playback_worker_command::PlaybackWorkerCommand;
use super::playback_worker_state::PlaybackWorkerState;
use super::playback_worker_transition;
use super::types::PlaybackTrack;

impl NativePipeline {
    fn worker_operation_name(command: &PlaybackWorkerCommand) -> &'static str {
        match command {
            PlaybackWorkerCommand::Load { .. } => "load",
            PlaybackWorkerCommand::Play => "play",
            PlaybackWorkerCommand::Pause => "pause",
            PlaybackWorkerCommand::Resume => "resume",
            PlaybackWorkerCommand::Stop => "stop",
            PlaybackWorkerCommand::Seek { .. } => "seek",
            PlaybackWorkerCommand::SetVolume { .. } => "set_volume",
            PlaybackWorkerCommand::SetMuted { .. } => "set_muted",
            PlaybackWorkerCommand::Shutdown => "shutdown",
        }
    }

    fn load_worker_track(&mut self, track: &PlaybackTrack) -> PlaybackResult<()> {
        let request = AudioDecoderOpenRequest::from_track(track);
        self.open_decoder(request, 0)?;
        self.schedule_decode_step()
    }

    pub fn handle_worker_command(&mut self, command: &PlaybackWorkerCommand) -> PlaybackResult<()> {
        let operation = Self::worker_operation_name(command);

        if let PlaybackWorkerCommand::Load { track } = command {
            return self.load_worker_track(track);
        }

        if matches!(command, PlaybackWorkerCommand::Shutdown) {
            return self.shutdown();
        }

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

    pub fn route_worker_command(
        &mut self,
        state: &PlaybackWorkerState,
        command: &PlaybackWorkerCommand,
    ) -> (PlaybackWorkerState, PlaybackResult<()>) {
        let next = self.map_worker_state(state, command);
        let runtime = self.handle_worker_command(command);
        (next, runtime)
    }

    pub fn route_worker_command_record_runtime_error(
        &mut self,
        state: &PlaybackWorkerState,
        command: &PlaybackWorkerCommand,
    ) -> PlaybackWorkerState {
        let (next, runtime) = self.route_worker_command(state, command);
        if let Err(error) = runtime {
            self.state.output_status.last_error = Some(error.to_string());
        }
        next
    }
}
