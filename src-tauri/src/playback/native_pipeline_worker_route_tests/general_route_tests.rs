use super::*;

#[test]
fn route_worker_command_maps_state_and_keeps_runtime_unsupported() {
    let mut pipeline = NativePipeline::new();
    let state = PlaybackWorkerState::idle();
    let command = PlaybackWorkerCommand::Play;

    let (next, runtime) = pipeline.route_worker_command(&state, &command);

    assert_eq!(next.phase, PlaybackWorkerPhase::Playing);
    assert_eq!(state.phase, PlaybackWorkerPhase::Idle);
    assert!(runtime.is_ok());
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
        Err(PlaybackError::Backend(message)) => {
            assert!(message.contains("decoder is not open"));
        }
        Err(other) => panic!("expected backend error, got {other}"),
        Ok(_) => panic!("expected backend error, got success"),
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
    assert!(snapshot.output_status.last_error.is_some());
    assert!(snapshot
        .output_status
        .last_error
        .as_deref()
        .unwrap()
        .contains("decoder is not open"));
}

#[test]
fn route_non_phase_commands_keep_loaded_phase() {
    let mut pipeline = NativePipeline::new();
    let mut state = PlaybackWorkerState::idle();
    state.mark_loaded("route-track-8");

    let commands = vec![
        PlaybackWorkerCommand::Seek { position_ms: 321 },
        PlaybackWorkerCommand::SetVolume { level: 0.5 },
        PlaybackWorkerCommand::SetMuted { muted: false },
    ];

    for command in commands {
        let next = pipeline.route_worker_command_record_runtime_error(&state, &command);
        assert_eq!(next.phase, PlaybackWorkerPhase::Loaded);
        assert_eq!(next.active_track_id.as_deref(), Some("route-track-8"));
    }
}
