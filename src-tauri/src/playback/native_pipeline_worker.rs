use super::decoder_request::AudioDecoderOpenRequest;
use super::errors::PlaybackResult;
use super::native_pipeline::NativePipeline;
use super::playback_worker_command::PlaybackWorkerCommand;
use super::playback_worker_state::PlaybackWorkerState;
use super::playback_worker_transition;
use super::types::PlaybackTrack;

impl NativePipeline {
    fn load_worker_track(&mut self, track: &PlaybackTrack) -> PlaybackResult<()> {
        let request = AudioDecoderOpenRequest::from_track(track);
        self.open_decoder(request, 0)?;
        self.schedule_decode_step()?;
        self.drain_next_frame_to_output()
    }

    pub fn handle_worker_command(&mut self, command: &PlaybackWorkerCommand) -> PlaybackResult<()> {
        match command {
            PlaybackWorkerCommand::Load { track } => self.load_worker_track(track),
            PlaybackWorkerCommand::Play => self.start(),
            PlaybackWorkerCommand::Pause => self.pause_output(),
            PlaybackWorkerCommand::Resume => self.resume_output(),
            PlaybackWorkerCommand::Stop => self.stop_output(),
            PlaybackWorkerCommand::Seek { position_ms } => self.seek_decoder(*position_ms),
            PlaybackWorkerCommand::SetVolume { level } => self.set_output_volume(*level),
            PlaybackWorkerCommand::SetMuted { muted } => self.set_output_muted(*muted),
            PlaybackWorkerCommand::Shutdown => self.shutdown(),
        }
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
