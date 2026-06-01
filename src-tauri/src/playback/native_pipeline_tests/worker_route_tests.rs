use super::*;

#[test]
fn map_worker_state_routes_transition_without_runtime_side_effects() {
    let pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Play;

    let next = pipeline.map_worker_state(&state, &command);

    assert_eq!(
        next.phase,
        super::super::playback_worker_state::PlaybackWorkerPhase::Playing
    );
    assert_eq!(
        state.phase,
        super::super::playback_worker_state::PlaybackWorkerPhase::Idle
    );
}

#[test]
fn map_worker_state_does_not_mutate_pipeline_snapshot() {
    let pipeline = NativePipeline::new();
    let before = pipeline.state();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Play;

    let _ = pipeline.map_worker_state(&state, &command);
    let after = pipeline.state();

    assert_eq!(
        before.decoder_request.is_none(),
        after.decoder_request.is_none()
    );
    assert_eq!(
        before.output_status.pending_frames,
        after.output_status.pending_frames
    );
    assert_eq!(before.decoder_state.phase, after.decoder_state.phase);
}

#[test]
fn route_runtime_error_context_is_overwritten_by_latest_operation() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();

    let _ = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::Seek { position_ms: 10 },
    );
    let _ = pipeline.route_worker_command_record_runtime_error(
        &state,
        &PlaybackWorkerCommand::SetMuted { muted: true },
    );

    let after = pipeline.state();
    assert_eq!(
        after.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command set_muted is not implemented yet")
    );
}
