use super::decoder::{AudioSampleFormat, AudioStreamInfo};
use super::decoder_request::AudioDecoderOpenRequest;
use super::decoder_runtime_state::{DecoderRuntimePhase, DecoderRuntimeState};
use super::errors::PlaybackError;
use super::native_pipeline::NativePipeline;
use super::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings};
use super::playback_worker_command::PlaybackWorkerCommand;
use super::playback_worker_state::PlaybackWorkerState;
use super::types::{PlaybackTrack, TrackId};

fn request() -> AudioDecoderOpenRequest {
    AudioDecoderOpenRequest {
        track_id: "track-77".to_string(),
        source_path: "C:/Music/track-77.flac".to_string(),
    }
}

fn stream_info() -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz: 48_000,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    }
}

fn output_frame() -> AudioOutputFrame {
    AudioOutputFrame {
        stream: stream_info(),
        position_ms: 100,
        samples: vec![0.0, 0.1, -0.1, 0.2],
    }
}

fn assert_worker_unsupported(result: Result<(), PlaybackError>, operation: &str) {
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

mod decoder_config_tests;
mod decoder_step_tests;
mod output_runtime_tests;
mod runtime_error_tests;
mod state_snapshot_tests;
mod wav_test_file;
mod worker_route_tests;
