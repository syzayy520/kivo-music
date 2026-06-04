use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output::AudioOutputFrame;
use crate::playback::output_wasapi::frame_bridge::format_mapper::FrameBridgeError;
use crate::playback::output_wasapi::frame_bridge::silent_guard::ensure_silent_frame;

fn dummy_stream() -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz: 44100,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    }
}

#[test]
fn empty_frame_is_silent() {
    let frame = AudioOutputFrame {
        stream: dummy_stream(),
        position_ms: 0,
        samples: vec![],
    };
    assert!(ensure_silent_frame(&frame).is_ok());
}

#[test]
fn all_zero_frame_is_silent() {
    let frame = AudioOutputFrame {
        stream: dummy_stream(),
        position_ms: 0,
        samples: vec![0.0; 256],
    };
    assert!(ensure_silent_frame(&frame).is_ok());
}

#[test]
fn non_zero_positive_sample_is_rejected() {
    let mut samples = vec![0.0; 256];
    samples[128] = 1.0;
    let frame = AudioOutputFrame {
        stream: dummy_stream(),
        position_ms: 0,
        samples,
    };
    let result = ensure_silent_frame(&frame);
    assert_eq!(
        result.unwrap_err(),
        FrameBridgeError::NonSilentFrameRejected
    );
}

#[test]
fn non_zero_negative_sample_is_rejected() {
    let mut samples = vec![0.0; 256];
    samples[64] = -0.1;
    let frame = AudioOutputFrame {
        stream: dummy_stream(),
        position_ms: 0,
        samples,
    };
    let result = ensure_silent_frame(&frame);
    assert_eq!(
        result.unwrap_err(),
        FrameBridgeError::NonSilentFrameRejected
    );
}

#[test]
fn mixed_zero_and_non_zero_is_rejected() {
    let mut samples = vec![0.0; 256];
    samples[0] = 0.001; // very small non-zero
    let frame = AudioOutputFrame {
        stream: dummy_stream(),
        position_ms: 0,
        samples,
    };
    let result = ensure_silent_frame(&frame);
    assert_eq!(
        result.unwrap_err(),
        FrameBridgeError::NonSilentFrameRejected
    );
}
