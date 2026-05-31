use super::*;

#[test]
fn route_set_volume_from_stopped_keeps_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-7");
    state.mark_stopped();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetVolume { level: 0.25 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-7"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}

#[test]
fn route_set_volume_from_failed_keeps_failed_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-19");
    state.mark_failed("edge-failed");

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetVolume { level: 0.66 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Failed);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-19"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}

#[test]
fn route_set_volume_from_idle_keeps_idle_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetVolume { level: 0.9 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert_eq!(next.active_track_id, None);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}

#[test]
fn route_set_volume_from_paused_keeps_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-21");
    state.mark_paused();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetVolume { level: 0.33 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-21"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}

#[test]
fn route_set_volume_from_playing_keeps_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-23");
    state.mark_playing();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetVolume { level: 0.44 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-23"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some(
            "unsupported operation: native pipeline worker command set_volume is not implemented yet"
        )
    );
}
