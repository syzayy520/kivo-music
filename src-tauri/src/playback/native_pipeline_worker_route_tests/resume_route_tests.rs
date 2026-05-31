use super::*;

#[test]
fn route_resume_from_paused_moves_to_playing() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("route-track-3");
    state.mark_paused();
    let command = PlaybackWorkerCommand::Resume;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("route-track-3"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command resume is not implemented yet")
    );
}
