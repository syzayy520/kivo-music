use super::super::*;

#[test]
fn route_pause_from_paused_keeps_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-3");
    state.mark_paused();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Pause);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-3"));
    assert!(snapshot.output_status.last_error.is_none());
}

#[test]
fn route_pause_from_stopped_moves_to_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-12");
    state.mark_stopped();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Pause);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-12"));
    assert!(snapshot.output_status.last_error.is_none());
}

#[test]
fn route_pause_from_failed_moves_to_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-15");
    state.mark_failed("edge-failed");

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Pause);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-15"));
    assert!(snapshot.output_status.last_error.is_none());
}

#[test]
fn route_pause_from_idle_moves_to_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Pause);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id, None);
    assert!(snapshot.output_status.last_error.is_none());
}

#[test]
fn route_pause_from_playing_moves_to_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-25");
    state.mark_playing();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Pause);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-25"));
    assert!(snapshot.output_status.last_error.is_none());
}
