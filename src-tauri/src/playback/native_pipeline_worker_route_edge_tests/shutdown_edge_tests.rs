use super::*;

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
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
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
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
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
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
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
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
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
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
}

#[test]
fn route_shutdown_from_idle_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Shutdown;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
}

#[test]
fn route_shutdown_from_playing_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-shutdown-ctx-1");
    state.mark_playing();
    let command = PlaybackWorkerCommand::Shutdown;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
}

#[test]
fn route_shutdown_from_paused_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-shutdown-ctx-2");
    state.mark_paused();
    let command = PlaybackWorkerCommand::Shutdown;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
}

#[test]
fn route_shutdown_from_stopped_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-shutdown-ctx-3");
    state.mark_stopped();
    let command = PlaybackWorkerCommand::Shutdown;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
}

#[test]
fn route_shutdown_from_failed_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-shutdown-ctx-4");
    state.mark_failed("edge-shutdown-runtime-error-before-ctx");
    let command = PlaybackWorkerCommand::Shutdown;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command shutdown is not implemented yet"
        )
    );
}
