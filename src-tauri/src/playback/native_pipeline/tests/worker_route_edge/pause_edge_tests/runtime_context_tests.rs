use super::super::*;

#[test]
fn route_pause_from_idle_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Pause;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.snapshot();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert!(snapshot.output_status.last_error.is_none());
}

#[test]
fn route_pause_from_playing_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-pause-ctx-1");
    state.mark_playing();
    let command = PlaybackWorkerCommand::Pause;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.snapshot();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert!(snapshot.output_status.last_error.is_none());
}

#[test]
fn route_pause_from_paused_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-pause-ctx-2");
    state.mark_paused();
    let command = PlaybackWorkerCommand::Pause;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.snapshot();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert!(snapshot.output_status.last_error.is_none());
}

#[test]
fn route_pause_from_stopped_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-pause-ctx-3");
    state.mark_stopped();
    let command = PlaybackWorkerCommand::Pause;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.snapshot();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert!(snapshot.output_status.last_error.is_none());
}

#[test]
fn route_pause_from_failed_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-pause-ctx-4");
    state.mark_failed("edge-pause-runtime-error-before-ctx");
    let command = PlaybackWorkerCommand::Pause;

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.snapshot();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert!(snapshot.output_status.last_error.is_none());
}
