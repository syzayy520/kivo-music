use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::decoder_request::AudioDecoderOpenRequest;
use crate::playback::decoder_runtime_state::{DecoderRuntimePhase, DecoderRuntimeState};
use crate::playback::native_pipeline::NativePipeline;
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings};
use crate::playback::playback_worker_command::PlaybackWorkerCommand;
use crate::playback::playback_worker_state::PlaybackWorkerState;
use crate::playback::types::{PlaybackTrack, TrackId};

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

mod decoder_config_tests;
mod decoder_step_tests;
mod output_runtime_tests;
mod runtime_error_tests;
mod seek_transaction_tests;
mod state_snapshot_tests;
mod wav_test_file;
mod worker_route_tests;
