use super::super::*;

#[test]
fn route_seek_from_playing_keeps_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-1");
    state.mark_playing();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::Seek { position_ms: 9_999 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-1"));
    assert!(snapshot.output_status.last_error.is_some());
    assert!(snapshot
        .output_status
        .last_error
        .as_deref()
        .unwrap()
        .contains("decoder is not open"));
}

#[test]
fn route_seek_from_stopped_keeps_stopped_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-9");
    state.mark_stopped();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::Seek { position_ms: 2048 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Stopped);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-9"));
    assert!(snapshot.output_status.last_error.is_some());
    assert!(snapshot
        .output_status
        .last_error
        .as_deref()
        .unwrap()
        .contains("decoder is not open"));
}

#[test]
fn route_seek_from_failed_keeps_failed_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-18");
    state.mark_failed("edge-failed");

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::Seek { position_ms: 8080 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Failed);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-18"));
    assert!(snapshot.output_status.last_error.is_some());
    assert!(snapshot
        .output_status
        .last_error
        .as_deref()
        .unwrap()
        .contains("decoder is not open"));
}

#[test]
fn route_seek_from_idle_keeps_idle_phase() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::Seek { position_ms: 1 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert_eq!(next.active_track_id, None);
    assert!(snapshot.output_status.last_error.is_some());
    assert!(snapshot
        .output_status
        .last_error
        .as_deref()
        .unwrap()
        .contains("decoder is not open"));
}

#[test]
fn route_seek_from_paused_keeps_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-29");
    state.mark_paused();

    let next = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::Seek { position_ms: 555 },
    );
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-29"));
    assert!(snapshot.output_status.last_error.is_some());
    assert!(snapshot
        .output_status
        .last_error
        .as_deref()
        .unwrap()
        .contains("decoder is not open"));
}
