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
fn worker_set_volume_and_set_muted_update_output_controls() {
    let mut pipeline = NativePipeline::new();

    pipeline
        .handle_worker_command(&PlaybackWorkerCommand::SetVolume { level: 1.5 })
        .expect("set output volume");
    pipeline
        .handle_worker_command(&PlaybackWorkerCommand::SetMuted { muted: true })
        .expect("set output muted");

    let state = pipeline.state();
    assert_eq!(state.output_status.controls.volume_level, 1.0);
    assert!(state.output_status.controls.muted);
    assert!(state.output_status.last_error.is_none());
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
