use super::super::*;

#[test]
fn route_set_volume_from_idle_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::SetVolume { level: 0.2 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}

#[test]
fn route_set_volume_from_playing_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-set-volume-ctx-1");
    state.mark_playing();
    let command = PlaybackWorkerCommand::SetVolume { level: 0.3 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}

#[test]
fn route_set_volume_from_paused_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-set-volume-ctx-2");
    state.mark_paused();
    let command = PlaybackWorkerCommand::SetVolume { level: 0.4 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}

#[test]
fn route_set_volume_from_stopped_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-set-volume-ctx-3");
    state.mark_stopped();
    let command = PlaybackWorkerCommand::SetVolume { level: 0.5 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}

#[test]
fn route_set_volume_from_failed_records_runtime_unsupported_context() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-set-volume-ctx-4");
    state.mark_failed("edge-set-volume-runtime-error-before-ctx");
    let command = PlaybackWorkerCommand::SetVolume { level: 0.6 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Failed);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}
