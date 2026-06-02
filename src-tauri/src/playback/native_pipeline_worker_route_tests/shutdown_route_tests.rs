use super::*;

#[test]
fn route_shutdown_from_playing_moves_to_stopped() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("route-track-5");
    state.mark_playing();
    let command = PlaybackWorkerCommand::Shutdown;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("route-track-5"));
    assert_eq!(snapshot.output_status.last_error.as_deref(), None);
}
