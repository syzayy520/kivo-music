use super::super::*;

#[test]
fn route_seek_from_idle_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Seek { position_ms: 5000 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.snapshot();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert!(snapshot.output_status.last_error.is_some());
    assert!(snapshot
        .output_status
        .last_error
        .as_deref()
        .unwrap()
        .contains("decoder is not open"));
}

#[test]
fn route_seek_from_playing_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-seek-ctx-1");
    state.mark_playing();
    let command = PlaybackWorkerCommand::Seek { position_ms: 6000 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.snapshot();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert!(snapshot.output_status.last_error.is_some());
    assert!(snapshot
        .output_status
        .last_error
        .as_deref()
        .unwrap()
        .contains("decoder is not open"));
}

#[test]
fn route_seek_from_paused_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-seek-ctx-2");
    state.mark_paused();
    let command = PlaybackWorkerCommand::Seek { position_ms: 7000 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.snapshot();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert!(snapshot.output_status.last_error.is_some());
    assert!(snapshot
        .output_status
        .last_error
        .as_deref()
        .unwrap()
        .contains("decoder is not open"));
}

#[test]
fn route_seek_from_stopped_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-seek-ctx-3");
    state.mark_stopped();
    let command = PlaybackWorkerCommand::Seek { position_ms: 8000 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.snapshot();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert!(snapshot.output_status.last_error.is_some());
    assert!(snapshot
        .output_status
        .last_error
        .as_deref()
        .unwrap()
        .contains("decoder is not open"));
}

#[test]
fn route_seek_from_failed_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-seek-ctx-4");
    state.mark_failed("edge-seek-runtime-error-before-ctx");
    let command = PlaybackWorkerCommand::Seek { position_ms: 9000 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.snapshot();

    assert_eq!(next.phase, PlaybackWorkerPhase::Failed);
    assert!(snapshot.output_status.last_error.is_some());
    assert!(snapshot
        .output_status
        .last_error
        .as_deref()
        .unwrap()
        .contains("decoder is not open"));
}
