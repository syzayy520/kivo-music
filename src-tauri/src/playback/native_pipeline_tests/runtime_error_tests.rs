use super::*;

#[test]
fn start_routes_to_output_open_boundary() {
    let mut pipeline = NativePipeline::new();

    let result = pipeline.start();

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, "kivo native output open is not implemented yet");
        }
        Err(other) => panic!("expected unsupported output open, got {other}"),
        Ok(_) => panic!("expected unsupported output open, got success"),
    }

    let state = pipeline.state();
    assert_eq!(
        state.output_status.last_error.as_deref(),
        Some("kivo native output open is not implemented yet")
    );
}

#[test]
fn submit_without_frame_returns_backend_error() {
    let mut pipeline = NativePipeline::new();

    let result = pipeline.submit();

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert_eq!(message, "native pipeline output frame is not ready");
        }
        Err(other) => panic!("expected backend error, got {other}"),
        Ok(_) => panic!("expected backend error, got success"),
    }
}

#[test]
fn schedule_output_submit_step_without_frame_returns_backend_error() {
    let mut pipeline = NativePipeline::new();
    let result = pipeline.schedule_output_submit_step();

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert_eq!(message, "native pipeline output frame is not ready");
        }
        Err(other) => panic!("expected backend error, got {other}"),
        Ok(_) => panic!("expected backend error, got success"),
    }
}

#[test]
fn schedule_decode_step_without_open_decoder_returns_backend_error() {
    let mut pipeline = NativePipeline::new();
    let result = pipeline.schedule_decode_step();

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert_eq!(message, "native pipeline decoder is not open");
        }
        Err(other) => panic!("expected backend error, got {other}"),
        Ok(_) => panic!("expected backend error, got success"),
    }
}

#[test]
fn seek_decoder_without_open_decoder_returns_backend_error() {
    let mut pipeline = NativePipeline::new();
    let result = pipeline.seek_decoder(1_000);

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert_eq!(message, "native pipeline decoder is not open");
        }
        Err(other) => panic!("expected backend error, got {other}"),
        Ok(_) => panic!("expected backend error, got success"),
    }
}

#[test]
fn shutdown_closes_empty_pipeline_without_fake_runtime_error() {
    let mut pipeline = NativePipeline::new();

    pipeline.shutdown().expect("shutdown empty pipeline");

    let state = pipeline.state();
    assert_eq!(state.decoder_state.phase, DecoderRuntimePhase::Closed);
    assert!(state.decoder_session.is_none());
    assert!(state.last_decoded_frame.is_none());
    assert!(!state.output_status.is_open);
}

#[test]
fn worker_commands_are_typed_unsupported() {
    let mut pipeline = NativePipeline::new();
    let commands = vec![
        (
            PlaybackWorkerCommand::Load {
                track: worker_track(),
            },
            "load",
        ),
        (PlaybackWorkerCommand::Play, "play"),
        (PlaybackWorkerCommand::Pause, "pause"),
        (PlaybackWorkerCommand::Resume, "resume"),
        (PlaybackWorkerCommand::Stop, "stop"),
        (PlaybackWorkerCommand::Seek { position_ms: 1_000 }, "seek"),
        (
            PlaybackWorkerCommand::SetVolume { level: 0.8 },
            "set_volume",
        ),
        (PlaybackWorkerCommand::SetMuted { muted: true }, "set_muted"),
    ];

    for (command, operation) in commands {
        assert_worker_unsupported(pipeline.handle_worker_command(&command), operation);
    }
}

#[test]
fn worker_shutdown_closes_pipeline_resources() {
    let mut pipeline = NativePipeline::new();

    pipeline
        .handle_worker_command(&PlaybackWorkerCommand::Shutdown)
        .expect("worker shutdown");

    let state = pipeline.state();
    assert_eq!(state.decoder_state.phase, DecoderRuntimePhase::Closed);
    assert!(state.decoder_session.is_none());
    assert!(state.last_decoded_frame.is_none());
    assert!(!state.output_status.is_open);
}
