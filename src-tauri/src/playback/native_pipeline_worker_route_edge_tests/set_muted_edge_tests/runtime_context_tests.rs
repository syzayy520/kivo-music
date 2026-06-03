use super::super::*;

#[test]
fn route_set_muted_from_idle_updates_output_controls() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::SetMuted { muted: true };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert!(snapshot.output_status.last_error.is_none());
}

#[test]
fn route_set_muted_from_playing_updates_output_controls() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-set-muted-ctx-1");
    state.mark_playing();
    let command = PlaybackWorkerCommand::SetMuted { muted: false };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert!(snapshot.output_status.last_error.is_none());
}

#[test]
fn route_set_muted_from_paused_updates_output_controls() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-set-muted-ctx-2");
    state.mark_paused();
    let command = PlaybackWorkerCommand::SetMuted { muted: true };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert!(snapshot.output_status.last_error.is_none());
}

#[test]
fn route_set_muted_from_stopped_updates_output_controls() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-set-muted-ctx-3");
    state.mark_stopped();
    let command = PlaybackWorkerCommand::SetMuted { muted: false };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert!(snapshot.output_status.last_error.is_none());
}

#[test]
fn route_set_muted_from_failed_updates_output_controls() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-set-muted-ctx-4");
    state.mark_failed("edge-set-muted-runtime-error-before-ctx");
    let command = PlaybackWorkerCommand::SetMuted { muted: true };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Failed);
    assert!(snapshot.output_status.last_error.is_none());
}
