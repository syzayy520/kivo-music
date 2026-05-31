use super::native_pipeline::NativePipeline;
use super::playback_worker_command::PlaybackWorkerCommand;
use super::playback_worker_state::{PlaybackWorkerPhase, PlaybackWorkerState};

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
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command seek is not implemented yet")
    );
}

#[test]
fn route_play_from_playing_keeps_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-2");
    state.mark_playing();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Play);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-2"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command play is not implemented yet")
    );
}

#[test]
fn route_pause_from_paused_keeps_paused_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-3");
    state.mark_paused();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Pause);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Paused);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-3"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command pause is not implemented yet")
    );
}

#[test]
fn route_resume_from_playing_keeps_playing_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("edge-track-4");
    state.mark_playing();

    let next =
        pipeline.route_worker_command_record_runtime_error(&state, &PlaybackWorkerCommand::Resume);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(next.active_track_id.as_deref(), Some("edge-track-4"));
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command resume is not implemented yet")
    );
}

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
