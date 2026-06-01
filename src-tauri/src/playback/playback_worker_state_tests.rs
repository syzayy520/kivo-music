use super::playback_worker_state::{PlaybackWorkerPhase, PlaybackWorkerState};

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
fn idle_state_starts_without_track_or_error() {
    let state = PlaybackWorkerState::idle();

    assert_state(&state, PlaybackWorkerPhase::Idle, None, None);
}

#[test]
fn loaded_and_playing_transitions_are_modeled() {
    let mut state = PlaybackWorkerState::idle();

    state.mark_loaded("track-1");
    assert_state(&state, PlaybackWorkerPhase::Loaded, Some("track-1"), None);

    state.mark_playing();
    assert_state(&state, PlaybackWorkerPhase::Playing, Some("track-1"), None);
}

#[test]
fn paused_and_stopped_transitions_preserve_track() {
    let mut state = PlaybackWorkerState::idle();

    state.mark_loaded("track-2");
    state.mark_playing();
    state.mark_paused();
    assert_state(&state, PlaybackWorkerPhase::Paused, Some("track-2"), None);

    state.mark_stopped();
    assert_state(&state, PlaybackWorkerPhase::Stopped, Some("track-2"), None);
}

#[test]
fn failed_transition_keeps_error_message() {
    let mut state = PlaybackWorkerState::idle();

    state.mark_failed("worker failed");

    assert_state(
        &state,
        PlaybackWorkerPhase::Failed,
        None,
        Some("worker failed"),
    );
}

#[test]
fn mark_loaded_after_failed_replaces_track_and_clears_error() {
    let mut state = PlaybackWorkerState::idle();

    state.mark_failed("worker failed");
    state.active_track_id = Some("stale-track".to_string());
    state.mark_loaded("track-3");

    assert_state(&state, PlaybackWorkerPhase::Loaded, Some("track-3"), None);
}

#[test]
fn play_pause_stop_do_not_clear_existing_error() {
    let mut state = PlaybackWorkerState::idle();
    state.mark_failed("recoverable");
    state.active_track_id = Some("track-4".to_string());

    state.mark_playing();
    assert_state(
        &state,
        PlaybackWorkerPhase::Playing,
        Some("track-4"),
        Some("recoverable"),
    );

    state.mark_paused();
    assert_state(
        &state,
        PlaybackWorkerPhase::Paused,
        Some("track-4"),
        Some("recoverable"),
    );

    state.mark_stopped();
    assert_state(
        &state,
        PlaybackWorkerPhase::Stopped,
        Some("track-4"),
        Some("recoverable"),
    );
}
