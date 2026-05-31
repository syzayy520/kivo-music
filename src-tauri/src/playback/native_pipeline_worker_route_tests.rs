use super::errors::PlaybackError;
use super::native_pipeline::NativePipeline;
use super::playback_worker_command::PlaybackWorkerCommand;
use super::playback_worker_state::{PlaybackWorkerPhase, PlaybackWorkerState};
use super::types::{PlaybackTrack, TrackId};

fn track() -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId("route-track-1".to_string()),
        title: "Route Track".to_string(),
        artist: "Route Artist".to_string(),
        source_path: "C:/Music/route-track-1.wav".to_string(),
    }
}

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

#[test]
fn route_worker_command_records_runtime_error_context() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Seek { position_ms: 88 };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command seek is not implemented yet")
    );
}

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

#[test]
fn route_load_maps_phase_to_loaded() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Load { track: track() };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);

    assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
    assert_eq!(next.active_track_id.as_deref(), Some("route-track-1"));
}

#[test]
fn route_set_muted_keeps_phase_and_records_operation_context() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::SetMuted { muted: true };

    let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
    let snapshot = pipeline.state();

    assert_eq!(next.phase, PlaybackWorkerPhase::Idle);
    assert_eq!(
        snapshot.output_status.last_error.as_deref(),
        Some("unsupported operation: native pipeline worker command set_muted is not implemented yet")
    );
}
