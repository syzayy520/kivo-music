use super::*;

#[test]
fn route_stop_from_stopped_keeps_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-6");
    state.mark_stopped();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Stop);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-6"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}

#[test]
fn route_stop_from_failed_moves_to_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-16");
    state.mark_failed("edge-failed");

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Stop);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-16"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}

#[test]
fn route_stop_from_idle_moves_to_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Stop);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id, None);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}

#[test]
fn route_stop_from_paused_moves_to_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-26");
    state.mark_paused();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Stop);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-26"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}

#[test]
fn route_stop_from_playing_moves_to_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-31");
    state.mark_playing();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Stop);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-31"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}

#[test]
fn route_stop_from_idle_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Stop;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}

#[test]
fn route_stop_from_playing_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-stop-ctx-1");
    state.mark_playing();
    let command = PlaybackWorkerCommand::Stop;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}

#[test]
fn route_stop_from_paused_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-stop-ctx-2");
    state.mark_paused();
    let command = PlaybackWorkerCommand::Stop;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}

#[test]
fn route_stop_from_stopped_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-stop-ctx-3");
    state.mark_stopped();
    let command = PlaybackWorkerCommand::Stop;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}

#[test]
fn route_stop_from_failed_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-stop-ctx-4");
    state.mark_failed("edge-stop-runtime-error-before-ctx");
    let command = PlaybackWorkerCommand::Stop;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command stop is not implemented yet")
    );
}
