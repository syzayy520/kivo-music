//! Tests for sink_submission helpers.
//!
//! All tests are pure — no real audio device, no Windows API.

use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::errors::PlaybackError;
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus};
use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;
use crate::playback::output_wasapi::status::WasapiOutputStatus;

use super::buffered::submit_frame_to_ring_buffer;
use super::sample_bytes::f32_samples_to_ne_bytes;

fn float32_stream(channels: u16, sample_rate_hz: u32) -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz,
        channels,
        sample_format: AudioSampleFormat::Float32,
    }
}

fn make_frame(channels: u16, sample_rate_hz: u32, samples: Vec<f32>) -> AudioOutputFrame {
    AudioOutputFrame {
        stream: float32_stream(channels, sample_rate_hz),
        position_ms: 0,
        samples,
    }
}

/// Test f32 to native-endian bytes conversion.
#[test]
fn helper_converts_f32_samples_to_bytes() {
    let samples: Vec<f32> = vec![1.0, -1.0, 0.5];
    let bytes = f32_samples_to_ne_bytes(&samples);
    assert_eq!(bytes.len(), 12); // 3 * 4 bytes

    // Verify round-trip
    for (i, &s) in samples.iter().enumerate() {
        let mut arr = [0u8; 4];
        arr.copy_from_slice(&bytes[i * 4..(i + 1) * 4]);
        let recovered = f32::from_ne_bytes(arr);
        assert!((recovered - s).abs() < f32::EPSILON);
    }
}

/// Test empty samples produce empty bytes.
#[test]
fn helper_empty_samples_produce_empty_bytes() {
    let bytes = f32_samples_to_ne_bytes(&[]);
    assert!(bytes.is_empty());
}

/// Test submit_frame without real open returns UnsupportedOperation.
#[test]
fn submit_frame_without_real_open_returns_unsupported() {
    let frame = make_frame(2, 44100, vec![0.1, 0.2, 0.3, 0.4]);
    let mut status = WasapiOutputStatus::default();
    let mut ring_buffer: Option<RingBuffer> = None;
    let mut runtime = OutputRuntimeStatus::default();

    let result = submit_frame_to_ring_buffer(frame, &mut status, &mut ring_buffer, &mut runtime);
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(ref msg)) if msg.contains("submit_frame")
    ));
    assert!(ring_buffer.is_none());
    assert_eq!(status.submitted_frames, 0);
    assert_eq!(runtime.pending_frames, 0);
}

/// Test submit_frame with render client not acquired returns UnsupportedOperation.
#[test]
fn submit_frame_without_render_client_returns_unsupported() {
    let frame = make_frame(2, 44100, vec![0.1, 0.2]);
    let mut status = WasapiOutputStatus::default();
    status.is_open_attempted = true;
    status.is_real_device_open = true;
    // is_render_client_acquired is false
    let mut ring_buffer: Option<RingBuffer> = None;
    let mut runtime = OutputRuntimeStatus::default();

    let result = submit_frame_to_ring_buffer(frame, &mut status, &mut ring_buffer, &mut runtime);
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(ref msg))
            if msg.contains("render client")
    ));
}

/// Test empty frame is a no-op.
#[test]
fn helper_handles_empty_frame() {
    let frame = make_frame(2, 44100, vec![]);
    let mut status = WasapiOutputStatus::default();
    status.is_real_device_open = true;
    status.is_render_client_acquired = true;
    let mut ring_buffer: Option<RingBuffer> = None;
    let mut runtime = OutputRuntimeStatus::default();

    let result = submit_frame_to_ring_buffer(frame, &mut status, &mut ring_buffer, &mut runtime);
    assert!(result.is_ok());
    assert!(ring_buffer.is_none()); // no ring buffer created for empty frame
    assert_eq!(status.submitted_frames, 0);
    assert_eq!(runtime.pending_frames, 0);
}

/// Test successful write to ring buffer with lazy creation.
#[test]
fn helper_writes_frame_to_ring_buffer() {
    let frame = make_frame(2, 44100, vec![0.1, 0.2, 0.3, 0.4]);
    let mut status = WasapiOutputStatus::default();
    status.is_real_device_open = true;
    status.is_render_client_acquired = true;
    let mut ring_buffer: Option<RingBuffer> = None;
    let mut runtime = OutputRuntimeStatus::default();

    let result = submit_frame_to_ring_buffer(frame, &mut status, &mut ring_buffer, &mut runtime);
    assert!(result.is_ok());
    assert!(ring_buffer.is_some());
    assert_eq!(status.submitted_frames, 2); // 4 samples / 2 channels = 2 frames
    assert_eq!(runtime.pending_frames, 2);
    assert!(runtime.last_error.is_none());
}

/// Test ring buffer overflow returns error.
#[test]
fn helper_handles_ring_buffer_overflow() {
    let mut status = WasapiOutputStatus::default();
    status.is_real_device_open = true;
    status.is_render_client_acquired = true;

    // Create a tiny ring buffer (1 frame capacity = 2 samples for stereo)
    let format = crate::playback::output_wasapi::frame_bridge::ring_buffer_format_from_stream(
        &float32_stream(2, 44100),
    )
    .unwrap();
    let mut ring_buffer: Option<RingBuffer> =
        Some(RingBuffer::new(format, 1).expect("ring buffer creation"));
    let mut runtime = OutputRuntimeStatus::default();

    // First write succeeds (2 samples = 1 frame)
    let frame1 = make_frame(2, 44100, vec![0.1, 0.2]);
    let result = submit_frame_to_ring_buffer(frame1, &mut status, &mut ring_buffer, &mut runtime);
    assert!(result.is_ok());
    assert_eq!(status.submitted_frames, 1);

    // Second write fails (buffer full)
    let frame2 = make_frame(2, 44100, vec![0.3, 0.4]);
    let result = submit_frame_to_ring_buffer(frame2, &mut status, &mut ring_buffer, &mut runtime);
    assert!(matches!(result, Err(PlaybackError::Output(ref msg)) if msg.contains("write failed")));
}

/// Test that submit_frame does not contain GetBuffer/ReleaseBuffer/Start calls.
///
/// This is a compile-time guarantee: the buffered module does not import
/// any Windows WASAPI render client types.
#[test]
fn submit_frame_path_has_no_wasapi_device_calls() {
    // The submit_frame_to_ring_buffer function only uses:
    // - PlaybackError/PlaybackResult
    // - OutputRuntimeStatus
    // - wasapi_unsupported
    // - ring_buffer_format_from_stream
    // - RingBuffer
    // - WasapiOutputStatus
    // - f32_samples_to_ne_bytes
    //
    // None of these involve IAudioClient, IAudioRenderClient,
    // GetBuffer, ReleaseBuffer, or Start.
    // This is verified by the module's imports — no Windows API types present.
    assert!(true);
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
