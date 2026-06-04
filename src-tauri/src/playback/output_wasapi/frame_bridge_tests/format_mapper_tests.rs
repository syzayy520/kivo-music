use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output_wasapi::frame_bridge::format_mapper::{
    ring_buffer_format_from_stream, FrameBridgeError,
};

#[test]
fn float32_stereo_maps_to_ring_buffer_format() {
    let stream = AudioStreamInfo {
        sample_rate_hz: 44100,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    };
    let result = ring_buffer_format_from_stream(&stream).unwrap();
    assert_eq!(result.sample_rate_hz, 44100);
    assert_eq!(result.channels, 2);
    assert_eq!(result.bits_per_sample, 32);
    assert_eq!(result.block_align, 8);
}

#[test]
fn float32_mono_maps_to_ring_buffer_format() {
    let stream = AudioStreamInfo {
        sample_rate_hz: 48000,
        channels: 1,
        sample_format: AudioSampleFormat::Float32,
    };
    let result = ring_buffer_format_from_stream(&stream).unwrap();
    assert_eq!(result.sample_rate_hz, 48000);
    assert_eq!(result.channels, 1);
    assert_eq!(result.bits_per_sample, 32);
    assert_eq!(result.block_align, 4);
}

#[test]
fn zero_channels_rejected() {
    let stream = AudioStreamInfo {
        sample_rate_hz: 44100,
        channels: 0,
        sample_format: AudioSampleFormat::Float32,
    };
    let result = ring_buffer_format_from_stream(&stream);
    assert_eq!(result.unwrap_err(), FrameBridgeError::ZeroChannels);
}

#[test]
fn zero_sample_rate_rejected() {
    let stream = AudioStreamInfo {
        sample_rate_hz: 0,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    };
    let result = ring_buffer_format_from_stream(&stream);
    assert_eq!(result.unwrap_err(), FrameBridgeError::ZeroSampleRate);
}

#[test]
fn non_float32_sample_format_rejected() {
    let formats = [
        AudioSampleFormat::Signed16,
        AudioSampleFormat::Signed24,
        AudioSampleFormat::Signed32,
    ];
    for format in formats {
        let stream = AudioStreamInfo {
            sample_rate_hz: 44100,
            channels: 2,
            sample_format: format.clone(),
        };
        let result = ring_buffer_format_from_stream(&stream);
        assert_eq!(
            result.unwrap_err(),
            FrameBridgeError::UnsupportedSampleFormat,
            "Expected UnsupportedSampleFormat for {:?}",
            format
        );
    }
}
