use super::super::errors::PlaybackError;
use super::wav_test_file::write_test_wav;
use super::*;

#[test]
fn start_opens_null_sink_output_boundary() {
    let mut pipeline = NativePipeline::new();

    pipeline.start().expect("null sink open should succeed");

    let state = pipeline.snapshot();
    assert!(state.output_status.is_open);
    assert!(state.output_status.is_active);
    assert!(state.output_status.last_error.is_none());
}

#[test]
fn submit_without_frame_returns_backend_error() {
    let mut pipeline = NativePipeline::new();

    let result = pipeline.submit();

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert_eq!(message, "pipeline buffer is empty, no frame to drain");
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
            assert_eq!(message, "pipeline buffer is empty, no frame to drain");
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

    let state = pipeline.snapshot();
    assert_eq!(state.decoder_state.phase, DecoderRuntimePhase::Closed);
    assert!(state.decoder_session.is_none());
    assert!(state.last_decoded_frame.is_none());
    assert!(!state.output_status.is_open);
}

#[test]
fn worker_set_volume_and_set_muted_update_output_controls() {
    let mut pipeline = NativePipeline::new();

    pipeline
        .handle_worker_command(&PlaybackWorkerCommand::SetVolume { level: 0.8 })
        .expect("worker set volume");
    pipeline
        .handle_worker_command(&PlaybackWorkerCommand::SetMuted { muted: true })
        .expect("worker set muted");

    let state = pipeline.snapshot();
    assert_eq!(state.output_status.controls.volume_level, 0.8);
    assert!(state.output_status.controls.muted);
    assert!(state.output_status.last_error.is_none());
}

#[test]
fn worker_load_opens_decoder_decodes_first_frame_and_submits_to_null_sink() {
    let path = write_test_wav();
    let mut pipeline = NativePipeline::new();
    let track = PlaybackTrack {
        id: TrackId("worker-wav-load-1".to_string()),
        title: "Worker Wav Load".to_string(),
        artist: "Worker Artist".to_string(),
        source_path: path.clone(),
    };

    pipeline
        .handle_worker_command(&PlaybackWorkerCommand::Load { track })
        .expect("worker load wav");

    let state = pipeline.snapshot();
    let session = state
        .decoder_session
        .expect("worker load should keep decoder session");
    let frame = state
        .last_decoded_frame
        .expect("worker load should decode one frame");

    assert_eq!(session.track_id, "worker-wav-load-1");
    assert_eq!(session.decoded_frame_count, 1);
    assert_eq!(frame.stream.sample_rate_hz, 48_000);
    assert_eq!(frame.stream.channels, 2);
    assert_eq!(state.output_status.pending_frames, 1);
    assert!(state.output_status.last_error.is_none());

    pipeline.shutdown().expect("shutdown pipeline");
    std::fs::remove_file(path).expect("remove wav test file");
}

#[test]
fn worker_shutdown_closes_pipeline_resources() {
    let mut pipeline = NativePipeline::new();

    pipeline
        .handle_worker_command(&PlaybackWorkerCommand::Shutdown)
        .expect("worker shutdown");

    let state = pipeline.snapshot();
    assert_eq!(state.decoder_state.phase, DecoderRuntimePhase::Closed);
    assert!(state.decoder_session.is_none());
    assert!(state.last_decoded_frame.is_none());
    assert!(!state.output_status.is_open);
}
