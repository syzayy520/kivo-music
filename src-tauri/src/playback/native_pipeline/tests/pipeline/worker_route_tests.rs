use super::*;

#[test]
fn map_worker_state_routes_transition_without_runtime_side_effects() {
    let pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Play;

    let next = pipeline.map_worker_state(&state, &command);

    assert_eq!(
        next.phase,
        crate::playback::playback_worker_state::PlaybackWorkerPhase::Playing
    );
    assert_eq!(
        state.phase,
        crate::playback::playback_worker_state::PlaybackWorkerPhase::Idle
    );
}

#[test]
fn map_worker_state_does_not_mutate_pipeline_snapshot() {
    let pipeline = NativePipeline::new();
    let before = pipeline.snapshot();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Play;

    let _ = pipeline.map_worker_state(&state, &command);
    let after = pipeline.snapshot();

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
