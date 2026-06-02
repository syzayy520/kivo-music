use super::errors::PlaybackError;
use super::native_pipeline::NativePipeline;
use super::playback_worker_command::PlaybackWorkerCommand;
use super::types::{PlaybackTrack, TrackId};

fn track() -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId("runtime-track-1".to_string()),
        title: "Runtime Track".to_string(),
        artist: "Runtime Artist".to_string(),
        source_path: "C:/Music/runtime-track-1.wav".to_string(),
    }
}

#[test]
fn worker_set_volume_and_set_muted_are_typed_unsupported() {
    let mut pipeline = NativePipeline::new();
    let commands = vec![
        (
            PlaybackWorkerCommand::SetVolume { level: 0.5 },
            "set_volume",
        ),
        (PlaybackWorkerCommand::SetMuted { muted: true }, "set_muted"),
    ];

    for (command, operation) in commands {
        let result = pipeline.handle_worker_command(&command);
        match result {
            Err(PlaybackError::UnsupportedOperation(message)) => {
                assert_eq!(
                    message,
                    format!("native pipeline worker command {operation} is not implemented yet")
                );
            }
            Err(other) => panic!("expected unsupported operation, got {other}"),
            Ok(_) => panic!("expected unsupported operation, got success"),
        }
    }
}

#[test]
fn worker_play_opens_null_sink_output_boundary() {
    let mut pipeline = NativePipeline::new();

    pipeline
        .handle_worker_command(&PlaybackWorkerCommand::Play)
        .expect("worker play should open null sink");

    let state = pipeline.state();
    assert!(state.output_status.is_open);
    assert!(state.output_status.is_active);
    assert!(state.output_status.last_error.is_none());
}

#[test]
fn worker_pause_output_succeeds_via_null_sink() {
    let mut pipeline = NativePipeline::new();

    pipeline
        .handle_worker_command(&PlaybackWorkerCommand::Pause)
        .expect("null sink pause should succeed");

    let state = pipeline.state();
    assert!(state.output_status.last_error.is_none());
}

#[test]
fn worker_resume_output_succeeds_via_null_sink() {
    let mut pipeline = NativePipeline::new();

    pipeline
        .handle_worker_command(&PlaybackWorkerCommand::Resume)
        .expect("null sink resume should succeed");

    let state = pipeline.state();
    assert!(state.output_status.last_error.is_none());
}

#[test]
fn worker_stop_output_succeeds_via_null_sink() {
    let mut pipeline = NativePipeline::new();

    pipeline
        .handle_worker_command(&PlaybackWorkerCommand::Stop)
        .expect("null sink stop should succeed");

    let state = pipeline.state();
    assert!(state.output_status.last_error.is_none());
}

#[test]
fn worker_seek_without_decoder_returns_backend_error() {
    let mut pipeline = NativePipeline::new();

    let result = pipeline.handle_worker_command(&PlaybackWorkerCommand::Seek { position_ms: 100 });

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert_eq!(message, "native pipeline decoder is not open");
        }
        Err(other) => panic!("expected backend error, got {other}"),
        Ok(_) => panic!("expected backend error, got success"),
    }
}

#[test]
fn worker_load_reports_backend_error_for_missing_wav_file() {
    let mut pipeline = NativePipeline::new();
    let result = pipeline.handle_worker_command(&PlaybackWorkerCommand::Load { track: track() });

    match result {
        Err(PlaybackError::Backend(message)) => {
            assert!(!message.is_empty());
        }
        Err(other) => panic!("expected backend error, got {other}"),
        Ok(_) => panic!("expected backend error, got success"),
    }
}
