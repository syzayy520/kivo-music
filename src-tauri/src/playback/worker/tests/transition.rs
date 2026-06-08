use crate::playback::playback_worker_command::PlaybackWorkerCommand;
use crate::playback::playback_worker_state::{PlaybackWorkerPhase, PlaybackWorkerState};
use crate::playback::playback_worker_transition::apply_command;
use crate::playback::types::{PlaybackTrack, TrackId};

fn track() -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId("track-101".to_string()),
        title: "Track 101".to_string(),
        artist: "Artist".to_string(),
        source_path: "C:/Music/track-101.flac".to_string(),
    }
}

#[test]
fn load_command_moves_state_to_loaded() {
    let mut state = PlaybackWorkerState::idle();

    apply_command(&mut state, &PlaybackWorkerCommand::Load { track: track() });

    assert_eq!(state.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(state.active_track_id.as_deref(), Some("track-101"));
}

#[test]
fn play_pause_resume_stop_flow_is_modeled() {
    let mut state = PlaybackWorkerState::idle();

    apply_command(&mut state, &PlaybackWorkerCommand::Load { track: track() });
    apply_command(&mut state, &PlaybackWorkerCommand::Play);
    apply_command(&mut state, &PlaybackWorkerCommand::Pause);
    apply_command(&mut state, &PlaybackWorkerCommand::Resume);
    apply_command(&mut state, &PlaybackWorkerCommand::Stop);

    assert_eq!(state.phase, PlaybackWorkerPhase::Stopped);
}

#[test]
fn seek_volume_and_mute_do_not_change_phase() {
    let mut state = PlaybackWorkerState::idle();
    apply_command(&mut state, &PlaybackWorkerCommand::Load { track: track() });

    apply_command(
        &mut state,
        &PlaybackWorkerCommand::Seek { position_ms: 5_000 },
    );
    apply_command(&mut state, &PlaybackWorkerCommand::SetVolume { level: 0.7 });
    apply_command(&mut state, &PlaybackWorkerCommand::SetMuted { muted: true });

    assert_eq!(state.phase, PlaybackWorkerPhase::Loaded);
}
