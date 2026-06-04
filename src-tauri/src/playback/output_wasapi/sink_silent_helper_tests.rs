use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::output::AudioOutputFrame;
use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;
use crate::playback::output_wasapi::ring_buffer::types::RingBufferFormat;

use super::frame_bridge::SilentWriteError;
use super::sink_silent_helper::{ScaffoldSilentWriteError, ScaffoldSilentWriter};

fn float32_stream(channels: u16, sample_rate_hz: u32) -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz,
        channels,
        sample_format: AudioSampleFormat::Float32,
    }
}

fn silent_frame(stream: &AudioStreamInfo) -> AudioOutputFrame {
    AudioOutputFrame {
        stream: stream.clone(),
        position_ms: 0,
        samples: vec![0.0; 256],
    }
}

fn non_silent_frame(stream: &AudioStreamInfo) -> AudioOutputFrame {
    AudioOutputFrame {
        stream: stream.clone(),
        position_ms: 0,
        samples: vec![0.5; 256],
    }
}

fn prepared_ring_buffer(stream: &AudioStreamInfo, capacity: u32) -> Option<RingBuffer> {
    let format = RingBufferFormat {
        sample_rate_hz: stream.sample_rate_hz,
        channels: stream.channels,
        bits_per_sample: 32,
        block_align: stream.channels * 4,
    };
    Some(RingBuffer::new(format, capacity).unwrap())
}

// RingBuffer frames per AudioOutputFrame: 256 samples / channels
// Stereo: 128 frames, Mono: 256 frames

#[test]
fn scaffold_silent_write_writes_to_prepared_buffer() {
    let stream = float32_stream(2, 44100);
    let frame = silent_frame(&stream);
    let mut rb = prepared_ring_buffer(&stream, 512);

    let result = ScaffoldSilentWriter::write_silent_frame_to_prepared_buffer(&frame, &mut rb);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 128); // 256 samples / 2 channels = 128 frames

    assert_eq!(rb.as_ref().unwrap().available_frames(), 128);
}

#[test]
fn scaffold_silent_write_rejects_when_no_ring_buffer() {
    let stream = float32_stream(2, 44100);
    let frame = silent_frame(&stream);
    let mut rb: Option<RingBuffer> = None;

    let result = ScaffoldSilentWriter::write_silent_frame_to_prepared_buffer(&frame, &mut rb);
    assert!(matches!(
        result,
        Err(ScaffoldSilentWriteError::NoRingBuffer)
    ));
}

#[test]
fn scaffold_silent_write_rejects_non_silent_frame() {
    let stream = float32_stream(2, 44100);
    let frame = non_silent_frame(&stream);
    let mut rb = prepared_ring_buffer(&stream, 512);

    let result = ScaffoldSilentWriter::write_silent_frame_to_prepared_buffer(&frame, &mut rb);
    assert!(matches!(
        result,
        Err(ScaffoldSilentWriteError::SilentWrite(
            SilentWriteError::Bridge(_)
        ))
    ));

    assert_eq!(rb.as_ref().unwrap().available_frames(), 0);
}

#[test]
fn scaffold_silent_write_multiple_frames() {
    let stream = float32_stream(2, 44100);
    let frame = silent_frame(&stream);
    let mut rb = prepared_ring_buffer(&stream, 1024);

    for _ in 0..5 {
        let result = ScaffoldSilentWriter::write_silent_frame_to_prepared_buffer(&frame, &mut rb);
        assert!(result.is_ok());
    }

    assert_eq!(rb.as_ref().unwrap().available_frames(), 640); // 5 * 128 = 640
}

#[test]
fn scaffold_silent_write_respects_buffer_capacity() {
    let stream = float32_stream(2, 44100);
    let frame = silent_frame(&stream);
    let mut rb = prepared_ring_buffer(&stream, 256); // holds exactly 2 frames worth (256/128=2)

    assert!(ScaffoldSilentWriter::write_silent_frame_to_prepared_buffer(&frame, &mut rb).is_ok());
    assert!(ScaffoldSilentWriter::write_silent_frame_to_prepared_buffer(&frame, &mut rb).is_ok());

    // Third write should fail with WouldBlock
    let result = ScaffoldSilentWriter::write_silent_frame_to_prepared_buffer(&frame, &mut rb);
    assert!(matches!(
        result,
        Err(ScaffoldSilentWriteError::SilentWrite(
            SilentWriteError::Buffer(_)
        ))
    ));
}

#[test]
fn scaffold_silent_write_mono_format() {
    let stream = float32_stream(1, 48000);
    let frame = silent_frame(&stream);
    let mut rb = prepared_ring_buffer(&stream, 512);

    let result = ScaffoldSilentWriter::write_silent_frame_to_prepared_buffer(&frame, &mut rb);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 256); // 256 samples / 1 channel = 256 frames
    assert_eq!(rb.as_ref().unwrap().available_frames(), 256);
}
