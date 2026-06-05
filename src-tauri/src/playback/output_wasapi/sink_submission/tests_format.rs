//! Tests for format validation in submit_frame.
//!
//! All tests are pure — no real audio device, no Windows API.

use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::errors::PlaybackError;
use crate::playback::output::OutputRuntimeStatus;
use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;
use crate::playback::output_wasapi::status::WasapiOutputStatus;

use super::buffered::submit_frame_to_ring_buffer;

fn float32_stream(channels: u16, sample_rate_hz: u32) -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz,
        channels,
        sample_format: AudioSampleFormat::Float32,
    }
}

fn make_frame(channels: u16, sample_rate_hz: u32, samples: Vec<f32>) -> crate::playback::output::AudioOutputFrame {
    crate::playback::output::AudioOutputFrame {
        stream: float32_stream(channels, sample_rate_hz),
        position_ms: 0,
        samples,
    }
}

/// Test existing ring buffer with matching format writes successfully.
#[test]
fn existing_ring_buffer_format_match_writes() {
    use crate::playback::output_wasapi::frame_bridge::ring_buffer_format_from_stream;

    let mut status = WasapiOutputStatus::default();
    status.is_real_device_open = true;
    status.is_render_client_acquired = true;

    // Create ring buffer with matching format
    let format = ring_buffer_format_from_stream(&float32_stream(2, 44100)).unwrap();
    let mut ring_buffer: Option<RingBuffer> = Some(RingBuffer::new(format, 1024).unwrap());
    let mut runtime = OutputRuntimeStatus::default();

    let frame = make_frame(2, 44100, vec![0.1, 0.2, 0.3, 0.4]);
    let result = submit_frame_to_ring_buffer(frame, &mut status, &mut ring_buffer, &mut runtime);
    assert!(result.is_ok());
    assert_eq!(status.submitted_frames, 2); // 4 samples / 2 channels = 2 frames
    assert_eq!(runtime.pending_frames, 2);
    assert!(runtime.last_error.is_none());
}

/// Test existing ring buffer with mismatched format returns error.
#[test]
fn existing_ring_buffer_format_mismatch_returns_error() {
    use crate::playback::output_wasapi::frame_bridge::ring_buffer_format_from_stream;

    let mut status = WasapiOutputStatus::default();
    status.is_real_device_open = true;
    status.is_render_client_acquired = true;

    // Create ring buffer with 48000 Hz
    let format_48k = ring_buffer_format_from_stream(&float32_stream(2, 48000)).unwrap();
    let mut ring_buffer: Option<RingBuffer> = Some(RingBuffer::new(format_48k, 1024).unwrap());
    let mut runtime = OutputRuntimeStatus::default();

    // Submit frame with 44100 Hz — mismatch
    let frame = make_frame(2, 44100, vec![0.1, 0.2, 0.3, 0.4]);
    let result = submit_frame_to_ring_buffer(frame, &mut status, &mut ring_buffer, &mut runtime);

    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedFormat(ref msg)) if msg.contains("format mismatch")
    ));
    assert_eq!(status.submitted_frames, 0); // no write
    assert_eq!(runtime.pending_frames, 0); // no write
    assert!(runtime.last_error.is_some());
}

/// Test close still resets ring buffer and status.
#[test]
fn close_still_resets_ring_buffer_and_status() {
    use super::super::sink::WasapiOutputSink;
    use crate::playback::output::{OutputSettings, OutputSink};

    let mut sink = WasapiOutputSink::new();
    let _ = sink.open(&OutputSettings::default());

    // Prepare and submit to create ring buffer state
    sink.prepare_ring_buffer_for_stream(&float32_stream(2, 44100), 1024)
        .unwrap();
    let frame = make_frame(2, 44100, vec![0.1, 0.2]);
    let _ = sink.submit_frame(frame);

    // Close should reset everything
    sink.close().unwrap();
    let s = sink.status();
    assert!(!s.is_open && !s.is_active && s.last_error.is_none() && s.pending_frames == 0);
    assert!(!sink.has_ring_buffer());
}
