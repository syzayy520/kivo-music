use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::errors::PlaybackError;
use crate::playback::output::{AudioOutputFrame, OutputSink};

use super::frame_bridge::FrameBridgeError;
use super::ring_buffer::errors::RingBufferError;
use super::sink::{WasapiOutputSink, WasapiRingBufferPrepareError};

fn default_settings() -> crate::playback::output::OutputSettings {
    crate::playback::output::OutputSettings::default()
}

fn dummy_frame() -> AudioOutputFrame {
    AudioOutputFrame {
        stream: AudioStreamInfo {
            sample_rate_hz: 44100,
            channels: 2,
            sample_format: AudioSampleFormat::Float32,
        },
        position_ms: 0,
        samples: vec![0.0; 256],
    }
}

fn float32_stream(channels: u16, sample_rate_hz: u32) -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz,
        channels,
        sample_format: AudioSampleFormat::Float32,
    }
}

#[test]
fn open_sets_open_and_active() {
    let mut sink = WasapiOutputSink::new();
    let result = sink.open(&default_settings());
    assert!(result.is_ok());
    let status = result.unwrap();
    assert!(status.is_open);
    assert!(status.is_active);
    assert!(status.last_error.is_none());
}

#[test]
fn submit_frame_returns_unsupported_operation() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&default_settings()).unwrap();
    let result = sink.submit_frame(dummy_frame());
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(ref msg)) if msg.contains("submit_frame")
    ));
}

#[test]
fn pause_returns_ok() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&default_settings()).unwrap();
    let result = sink.pause();
    assert!(result.is_ok());
    let status = result.unwrap();
    assert!(status.is_open);
    assert!(status.is_active);
}

#[test]
fn resume_returns_ok() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&default_settings()).unwrap();
    let result = sink.resume();
    assert!(result.is_ok());
    let status = result.unwrap();
    assert!(status.is_open);
    assert!(status.is_active);
}

#[test]
fn flush_resets_pending_frames() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&default_settings()).unwrap();
    let result = sink.flush();
    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.pending_frames, 0);
}

#[test]
fn stop_sets_inactive() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&default_settings()).unwrap();
    let result = sink.stop();
    assert!(result.is_ok());
    let status = result.unwrap();
    assert!(!status.is_active);
    assert_eq!(status.pending_frames, 0);
}

#[test]
fn set_volume_updates_controls() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&default_settings()).unwrap();
    let result = sink.set_volume(0.5);
    assert!(result.is_ok());
    let status = result.unwrap();
    assert!((status.controls.volume_level - 0.5).abs() < f32::EPSILON);
}

#[test]
fn close_resets_all_state() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&default_settings()).unwrap();
    let close_result = sink.close();
    assert!(close_result.is_ok());
    let status = sink.status();
    assert!(!status.is_open);
    assert!(!status.is_active);
    assert!(status.last_error.is_none());
    assert_eq!(status.pending_frames, 0);
}
fn assert_bridge_err(r: Result<(), WasapiRingBufferPrepareError>, expected: FrameBridgeError) {
    assert!(matches!(
        r,
        Err(WasapiRingBufferPrepareError::Bridge(e)) if e == expected
    ));
}
fn assert_buffer_err(r: Result<(), WasapiRingBufferPrepareError>, expected: RingBufferError) {
    assert!(matches!(
        r,
        Err(WasapiRingBufferPrepareError::Buffer(e)) if e == expected
    ));
}
#[test]
fn new_sink_has_no_ring_buffer() {
    let sink = WasapiOutputSink::new();
    assert!(!sink.has_ring_buffer());
    assert!(sink.ring_buffer_available_frames().is_none());
}
#[test]
fn prepare_ring_buffer_succeeds() {
    let mut sink = WasapiOutputSink::new();
    let stream = float32_stream(2, 44100);
    assert!(sink.prepare_ring_buffer_for_stream(&stream, 1024).is_ok());
    assert!(sink.has_ring_buffer());
    assert_eq!(sink.ring_buffer_available_frames(), Some(0));
}
#[test]
fn prepare_ring_buffer_replaces_existing() {
    let mut sink = WasapiOutputSink::new();
    sink.prepare_ring_buffer_for_stream(&float32_stream(2, 44100), 512)
        .unwrap();
    sink.prepare_ring_buffer_for_stream(&float32_stream(2, 44100), 1024)
        .unwrap();
    assert!(sink.has_ring_buffer());
}
#[test]
fn prepare_ring_buffer_fails_zero_channels() {
    let mut sink = WasapiOutputSink::new();
    let r = sink.prepare_ring_buffer_for_stream(&float32_stream(0, 44100), 1024);
    assert_bridge_err(r, FrameBridgeError::ZeroChannels);
}
#[test]
fn prepare_ring_buffer_fails_zero_sample_rate() {
    let mut sink = WasapiOutputSink::new();
    let r = sink.prepare_ring_buffer_for_stream(&float32_stream(2, 0), 1024);
    assert_bridge_err(r, FrameBridgeError::ZeroSampleRate);
}
#[test]
fn prepare_ring_buffer_fails_non_float32() {
    let mut sink = WasapiOutputSink::new();
    let stream = AudioStreamInfo {
        sample_rate_hz: 44100,
        channels: 2,
        sample_format: AudioSampleFormat::Signed16,
    };
    let r = sink.prepare_ring_buffer_for_stream(&stream, 1024);
    assert_bridge_err(r, FrameBridgeError::UnsupportedSampleFormat);
}
#[test]
fn prepare_ring_buffer_fails_zero_capacity() {
    let mut sink = WasapiOutputSink::new();
    let r = sink.prepare_ring_buffer_for_stream(&float32_stream(2, 44100), 0);
    assert_buffer_err(r, RingBufferError::InvalidCapacity);
}
#[test]
fn flush_resets_ring_buffer_data() {
    let mut sink = WasapiOutputSink::new();
    let stream = float32_stream(2, 44100);
    sink.prepare_ring_buffer_for_stream(&stream, 1024).unwrap();
    sink.flush().unwrap();
    assert!(sink.has_ring_buffer());
    assert_eq!(sink.ring_buffer_available_frames(), Some(0));
}
#[test]
fn stop_preserves_ring_buffer() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&default_settings()).unwrap();
    let stream = float32_stream(2, 44100);
    sink.prepare_ring_buffer_for_stream(&stream, 1024).unwrap();
    sink.stop().unwrap();
    assert!(sink.has_ring_buffer());
    assert_eq!(sink.ring_buffer_available_frames(), Some(0));
}
#[test]
fn close_drops_and_prepare_recreates() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&default_settings()).unwrap();
    let stream = float32_stream(2, 44100);
    sink.prepare_ring_buffer_for_stream(&stream, 1024).unwrap();
    sink.close().unwrap();
    assert!(!sink.has_ring_buffer());
    assert!(sink.prepare_ring_buffer_for_stream(&stream, 1024).is_ok());
    assert!(sink.has_ring_buffer());
}
#[test]
fn prepare_ring_buffer_mono_format() {
    let mut sink = WasapiOutputSink::new();
    let stream = float32_stream(1, 48000);
    assert!(sink.prepare_ring_buffer_for_stream(&stream, 2048).is_ok());
}
