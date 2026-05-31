use super::*;

#[test]
fn route_set_volume_keeps_phase_and_records_operation_context() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::SetVolume { level: 0.7 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command set_volume is not implemented yet")
    );
}
