use super::*;

#[test]
fn route_pause_from_loaded_keeps_track_and_updates_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("route-track-2");
    let command = PlaybackWorkerCommand::Pause;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("route-track-2"));
    assert!(snapshot.output_status.last_error.is_none());
}
