use super::super::*;

#[test]
fn route_set_muted_from_stopped_keeps_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-8");
    state.mark_stopped();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetMuted { muted: true },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-8"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_muted is not implemented yet"
        )
    );
}

#[test]
fn route_set_muted_from_failed_keeps_failed_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-20");
    state.mark_failed("edge-failed");

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetMuted { muted: false },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Failed);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-20"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_muted is not implemented yet"
        )
    );
}

#[test]
fn route_set_muted_from_idle_keeps_idle_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetMuted { muted: true },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert_eq!(next.active_track_id, None);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_muted is not implemented yet"
        )
    );
}

#[test]
fn route_set_muted_from_paused_keeps_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-22");
    state.mark_paused();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetMuted { muted: true },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-22"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_muted is not implemented yet"
        )
    );
}

#[test]
fn route_set_muted_from_playing_keeps_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-24");
    state.mark_playing();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetMuted { muted: true },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-24"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_muted is not implemented yet"
        )
    );
}
