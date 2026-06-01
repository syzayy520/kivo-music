use super::playback_worker_state::{PlaybackWorkerPhase, PlaybackWorkerState};

#[test]
fn loaded_and_playing_transitions_are_modeled() {
    let mut state = PlaybackWorkerState::idle();

    state.mark_loaded("track-1");
    state.mark_playing();

    assert_eq!(state.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(state.active_track_id.as_deref(), Some("track-1"));
}

#[test]
fn paused_and_stopped_transitions_are_modeled() {
    let mut state = PlaybackWorkerState::idle();

    state.mark_loaded("track-2");
    state.mark_playing();
    state.mark_paused();
    state.mark_stopped();

    assert_eq!(state.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(state.active_track_id.as_deref(), Some("track-2"));
}

#[test]
fn failed_transition_keeps_error_message() {
    let mut state = PlaybackWorkerState::idle();

    state.mark_failed("worker failed");

    assert_eq!(state.phase, PlaybackWorkerPhase::Failed);
    assert_eq!(state.last_error.as_deref(), Some("worker failed"));
}

#[test]
fn mark_loaded_after_failed_clears_error() {
    let mut state = PlaybackWorkerState::idle();

    state.mark_failed("worker failed");
    state.mark_loaded("track-3");

    assert_eq!(state.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(state.active_track_id.as_deref(), Some("track-3"));
    assert!(state.last_error.is_none());
}
