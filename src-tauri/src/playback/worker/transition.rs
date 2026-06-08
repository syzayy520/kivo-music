use crate::playback::playback_worker_command::PlaybackWorkerCommand;
use crate::playback::playback_worker_state::PlaybackWorkerState;

pub fn apply_command(state: &mut PlaybackWorkerState, command: &PlaybackWorkerCommand) {
    match command {
        PlaybackWorkerCommand::Load { track } => state.mark_loaded(track.id.0.clone()),
        PlaybackWorkerCommand::Play | PlaybackWorkerCommand::Resume => state.mark_playing(),
        PlaybackWorkerCommand::Pause => state.mark_paused(),
        PlaybackWorkerCommand::Stop | PlaybackWorkerCommand::Shutdown => state.mark_stopped(),
        PlaybackWorkerCommand::Seek { .. }
        | PlaybackWorkerCommand::SetVolume { .. }
        | PlaybackWorkerCommand::SetMuted { .. } => {}
    }
}
