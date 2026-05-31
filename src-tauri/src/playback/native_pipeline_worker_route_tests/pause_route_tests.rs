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
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command pause is not implemented yet")
    );
}
