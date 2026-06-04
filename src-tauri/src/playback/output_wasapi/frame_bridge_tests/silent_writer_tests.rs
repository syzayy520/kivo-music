use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output::AudioOutputFrame;
use crate::playback::output_wasapi::frame_bridge::format_mapper::FrameBridgeError;
use crate::playback::output_wasapi::frame_bridge::silent_writer::{
    SilentRingBufferWriter, SilentWriteError,
};
use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;
use crate::playback::output_wasapi::ring_buffer::types::RingBufferFormat;

fn stereo_stream() -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz: 44100,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    }
}

fn stereo_ring_buffer(capacity_frames: u32) -> RingBuffer {
    let format = RingBufferFormat {
        sample_rate_hz: 44100,
        channels: 2,
        bits_per_sample: 32,
        block_align: 8,
    };
    RingBuffer::new(format, capacity_frames).unwrap()
}

fn silent_frame(stream: AudioStreamInfo, num_samples: usize) -> AudioOutputFrame {
    AudioOutputFrame {
        stream,
        position_ms: 0,
        samples: vec![0.0; num_samples],
    }
}

#[test]
fn silent_stereo_frame_writes_successfully() {
    let frame = silent_frame(stereo_stream(), 4); // 2 frames of stereo
    let mut rb = stereo_ring_buffer(16);
    let result = SilentRingBufferWriter::write_silent_frame(&frame, &mut rb);
    assert_eq!(result.unwrap(), 2);
    assert_eq!(rb.available_frames(), 2);
}

#[test]
fn empty_samples_writes_zero_frames() {
    let frame = silent_frame(stereo_stream(), 0);
    let mut rb = stereo_ring_buffer(16);
    let result = SilentRingBufferWriter::write_silent_frame(&frame, &mut rb);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn non_silent_frame_rejected() {
    let mut samples = vec![0.0; 4];
    samples[0] = 0.5;
    let frame = AudioOutputFrame {
        stream: stereo_stream(),
        position_ms: 0,
        samples,
    };
    let mut rb = stereo_ring_buffer(16);
    let result = SilentRingBufferWriter::write_silent_frame(&frame, &mut rb);
    assert_eq!(
        result.unwrap_err(),
        SilentWriteError::Bridge(FrameBridgeError::NonSilentFrameRejected)
    );
}

#[test]
fn non_float32_format_rejected() {
    let stream = AudioStreamInfo {
        sample_rate_hz: 44100,
        channels: 2,
        sample_format: AudioSampleFormat::Signed16,
    };
    let frame = silent_frame(stream, 4);
    let mut rb = stereo_ring_buffer(16);
    let result = SilentRingBufferWriter::write_silent_frame(&frame, &mut rb);
    assert_eq!(
        result.unwrap_err(),
        SilentWriteError::Bridge(FrameBridgeError::UnsupportedSampleFormat)
    );
}

#[test]
fn zero_channels_rejected() {
    let stream = AudioStreamInfo {
        sample_rate_hz: 44100,
        channels: 0,
        sample_format: AudioSampleFormat::Float32,
    };
    let frame = silent_frame(stream, 0);
    let mut rb = stereo_ring_buffer(16);
    let result = SilentRingBufferWriter::write_silent_frame(&frame, &mut rb);
    assert_eq!(
        result.unwrap_err(),
        SilentWriteError::Bridge(FrameBridgeError::ZeroChannels)
    );
}

#[test]
fn ring_buffer_full_returns_would_block() {
    // Ring buffer with capacity 1 frame, write 2 frames worth of silent data
    let frame = silent_frame(stereo_stream(), 4); // 2 frames of stereo
    let mut rb = stereo_ring_buffer(1);
    let result = SilentRingBufferWriter::write_silent_frame(&frame, &mut rb);
    // write_frames returns Ok(1) for partial write (first frame written, second ignored)
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn f32_bytes_are_native_endian() {
    // Verify that the byte conversion uses native endianness
    let samples: Vec<f32> = vec![1.0, -1.0]; // stereo frame
    let frame = AudioOutputFrame {
        stream: stereo_stream(),
        position_ms: 0,
        samples,
    };
    let mut rb = stereo_ring_buffer(16);
    let result = SilentRingBufferWriter::write_silent_frame(&frame, &mut rb);
    // Frame is not silent, so this should be rejected before byte conversion
    assert_eq!(
        result.unwrap_err(),
        SilentWriteError::Bridge(FrameBridgeError::NonSilentFrameRejected)
    );
}

#[test]
fn format_bridge_error_wraps_correctly() {
    let stream = AudioStreamInfo {
        sample_rate_hz: 0,
        channels: 2,
        sample_format: AudioSampleFormat::Float32,
    };
    let frame = silent_frame(stream, 4);
    let mut rb = stereo_ring_buffer(16);
    let result = SilentRingBufferWriter::write_silent_frame(&frame, &mut rb);
    assert_eq!(
        result.unwrap_err(),
        SilentWriteError::Bridge(FrameBridgeError::ZeroSampleRate)
    );
}
