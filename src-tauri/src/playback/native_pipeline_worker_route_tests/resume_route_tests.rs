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
    assert!(snapshot.output_status.last_error.is_none());
}
