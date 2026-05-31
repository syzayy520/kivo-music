use super::*;

#[test]
fn route_load_maps_phase_to_loaded() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Load { track: track() };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);

    assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(next.active_track_id.as_deref(), Some("route-track-1"));
}
