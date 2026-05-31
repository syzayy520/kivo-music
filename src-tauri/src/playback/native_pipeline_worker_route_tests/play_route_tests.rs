use super::*;

#[test]
fn route_play_from_loaded_moves_to_playing() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("route-track-6");
    let command = PlaybackWorkerCommand::Play;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("route-track-6"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command play is not implemented yet")
    );
}
