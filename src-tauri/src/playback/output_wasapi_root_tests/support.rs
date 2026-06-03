// output_wasapi_root_tests/support.rs
//
// Common test helpers for root WASAPI tests.

use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::errors::PlaybackError;
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus};

pub fn test_frame() -> AudioOutputFrame {
    AudioOutputFrame {
        stream: AudioStreamInfo {
            sample_rate_hz: 48_000,
            channels: 2,
            sample_format: AudioSampleFormat::Float32,
        },
        position_ms: 0,
        samples: vec![0.0, 0.1],
    }
}

pub fn assert_unsupported(result: Result<OutputRuntimeStatus, PlaybackError>, operation: &str) {
    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert!(
                message.contains("WASAPI output is not implemented yet"),
                "Expected WASAPI not implemented message, got: {message}"
            );
            assert!(
                message.contains(operation),
                "Expected operation '{operation}' in message, got: {message}"
            );
        }
        Err(other) => panic!("Expected UnsupportedOperation, got: {other}"),
        Ok(_) => panic!("Expected UnsupportedOperation, got success"),
    }
}
