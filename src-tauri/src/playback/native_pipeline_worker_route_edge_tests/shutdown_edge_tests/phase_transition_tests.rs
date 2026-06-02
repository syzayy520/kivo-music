use super::super::*;

#[test]
fn route_shutdown_from_stopped_keeps_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-5");
    state.mark_stopped();

    let next = pipeline
        .route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Shutdown);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-5"));
    assert_eq!(snapshot.output_status.last_error.as_deref(), None);
}

#[test]
fn route_shutdown_from_failed_moves_to_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-17");
    state.mark_failed("edge-failed");

    let next = pipeline
        .route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Shutdown);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-17"));
    assert_eq!(snapshot.output_status.last_error.as_deref(), None);
}

#[test]
fn route_shutdown_from_idle_moves_to_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next = pipeline
        .route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Shutdown);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id, None);
    assert_eq!(snapshot.output_status.last_error.as_deref(), None);
}

#[test]
fn route_shutdown_from_paused_moves_to_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-30");
    state.mark_paused();

    let next = pipeline
        .route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Shutdown);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-30"));
    assert_eq!(snapshot.output_status.last_error.as_deref(), None);
}

#[test]
fn route_shutdown_from_playing_moves_to_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-32");
    state.mark_playing();

    let next = pipeline
        .route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Shutdown);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-32"));
    assert_eq!(snapshot.output_status.last_error.as_deref(), None);
}
