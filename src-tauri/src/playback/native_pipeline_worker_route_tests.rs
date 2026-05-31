use super::errors::PlaybackError;
use super::native_pipeline::NativePipeline;
use super::playback_worker_command::PlaybackWorkerCommand;
use super::playback_worker_state::{PlaybackWorkerPhase, PlaybackWorkerState};

#[test]
fn route_worker_command_maps_state_and_keeps_runtime_unsupported() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Play;

    let (next, runtime) = pipeline.route_worker_command(&state, &command);

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(state.phase, PlaybackWorkerPhase::Idle);
    assert!(matches!(
        runtime,
        Err(PlaybackError::UnsupportedOperation(_))
    ));
}

#[test]
fn route_worker_command_does_not_mutate_pipeline_snapshot() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Pause;
    let before = pipeline.state();

    let _ = pipeline.route_worker_command(&state, &command);
    let after = pipeline.state();

    assert_eq!(before.decoder_state.phase, after.decoder_state.phase);
    assert_eq!(
        before.output_status.pending_frames,
        after.output_status.pending_frames
    );
}

#[test]
fn route_worker_command_keeps_operation_context() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Seek { position_ms: 42 };

    let (_next, runtime) = pipeline.route_worker_command(&state, &command);

    match runtime {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(
                message,
                "native pipeline worker command seek is not implemented yet"
            );
        }
        Err(other) => panic!("expected unsupported operation, got {other}"),
        Ok(_) => panic!("expected unsupported operation, got success"),
    }
}
