use super::playback_worker_command::PlaybackWorkerCommand;
use super::playback_worker_state::{PlaybackWorkerPhase, PlaybackWorkerState};
use super::playback_worker_transition::apply_command;
use super::types::{PlaybackTrack, TrackId};

fn track_with_id(id: &str) -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId(id.to_string()),
        title: format!("Track {id}"),
        artist: "Artist".to_string(),
        source_path: format!("C:/Music/{id}.flac"),
    }
}

fn track() -> PlaybackTrack {
    track_with_id("track-101")
}

fn assert_state(
    state: &PlaybackWorkerState,
    expected_phase: PlaybackWorkerPhase,
    expected_track_id: Option<&str>,
    expected_error: Option<&str>,
) {
    assert_eq!(state.phase, expected_phase);
    assert_eq!(state.active_track_id.as_deref(), expected_track_id);
    assert_eq!(state.last_error.as_deref(), expected_error);
}

#[test]
fn load_command_moves_state_to_loaded() {
    let mut state = PlaybackWorkerState::idle();

    apply_command(&mut state, &PlaybackWorkerCommand::Load { track: track() });

    assert_state(&state, PlaybackWorkerPhase::Loaded, Some("track-101"), None);
}

#[test]
fn play_pause_resume_stop_flow_is_modeled() {
    let mut state = PlaybackWorkerState::idle();

    apply_command(&mut state, &PlaybackWorkerCommand::Load { track: track() });
    apply_command(&mut state, &PlaybackWorkerCommand::Play);
    assert_state(
        &state,
        PlaybackWorkerPhase::Playing,
        Some("track-101"),
        None,
    );

    apply_command(&mut state, &PlaybackWorkerCommand::Pause);
    assert_state(&state, PlaybackWorkerPhase::Paused, Some("track-101"), None);

    apply_command(&mut state, &PlaybackWorkerCommand::Resume);
    assert_state(
        &state,
        PlaybackWorkerPhase::Playing,
        Some("track-101"),
        None,
    );

    apply_command(&mut state, &PlaybackWorkerCommand::Stop);
    assert_state(
        &state,
        PlaybackWorkerPhase::Stopped,
        Some("track-101"),
        None,
    );
}

#[test]
fn seek_volume_and_mute_do_not_change_state() {
    let mut state = PlaybackWorkerState::idle();
    apply_command(&mut state, &PlaybackWorkerCommand::Load { track: track() });

    apply_command(
        &mut state,
        &PlaybackWorkerCommand::Seek { position_ms: 5_000 },
    );
    apply_command(&mut state, &PlaybackWorkerCommand::SetVolume { level: 0.7 });
    apply_command(&mut state, &PlaybackWorkerCommand::SetMuted { muted: true });

    assert_state(&state, PlaybackWorkerPhase::Loaded, Some("track-101"), None);
}

#[test]
fn shutdown_command_moves_state_to_stopped() {
    let mut state = PlaybackWorkerState::idle();
    apply_command(&mut state, &PlaybackWorkerCommand::Load { track: track() });
    apply_command(&mut state, &PlaybackWorkerCommand::Play);

    apply_command(&mut state, &PlaybackWorkerCommand::Shutdown);

    assert_state(
        &state,
        PlaybackWorkerPhase::Stopped,
        Some("track-101"),
        None,
    );
}

#[test]
fn load_after_failed_replaces_error_and_track() {
    let mut state = PlaybackWorkerState::idle();
    state.mark_failed("failed-before-reload");

    apply_command(
        &mut state,
        &PlaybackWorkerCommand::Load {
            track: track_with_id("track-202"),
        },
    );

    assert_state(&state, PlaybackWorkerPhase::Loaded, Some("track-202"), None);
}

#[test]
fn stop_and_shutdown_do_not_clear_track_or_error() {
    let mut state = PlaybackWorkerState::idle();
    state.mark_failed("still-visible");
    state.active_track_id = Some("track-303".to_string());

    apply_command(&mut state, &PlaybackWorkerCommand::Stop);
    assert_state(
        &state,
        PlaybackWorkerPhase::Stopped,
        Some("track-303"),
        Some("still-visible"),
    );

    apply_command(&mut state, &PlaybackWorkerCommand::Shutdown);
    assert_state(
        &state,
        PlaybackWorkerPhase::Stopped,
        Some("track-303"),
        Some("still-visible"),
    );
}
