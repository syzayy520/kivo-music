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
fn worker_runtime_error_messages_include_operation_context() {
    let mut pipeline = NativePipeline::new();
    let commands = vec![
        (PlaybackWorkerCommand::Load { track: track() }, "load"),
        (PlaybackWorkerCommand::Play, "play"),
        (PlaybackWorkerCommand::Pause, "pause"),
        (PlaybackWorkerCommand::Resume, "resume"),
        (PlaybackWorkerCommand::Stop, "stop"),
        (PlaybackWorkerCommand::Seek { position_ms: 100 }, "seek"),
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
