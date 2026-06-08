use super::*;

#[test]
fn route_seek_from_paused_keeps_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("route-track-7");
    state.mark_paused();
    let command = PlaybackWorkerCommand::Seek { position_ms: 1234 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.snapshot();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("route-track-7"));
    assert!(snapshot.output_status.last_error.is_some());
    assert!(snapshot
        .output_status
        .last_error
        .as_deref()
        .unwrap()
        .contains("decoder is not open"));
}
