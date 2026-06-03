use super::*;

#[test]
fn route_set_volume_keeps_phase_and_updates_output_controls() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::SetVolume { level: 0.7 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.snapshot();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert_eq!(snapshot.output_status.controls.volume_level, 0.7);
    assert!(snapshot.output_status.last_error.is_none());
}
