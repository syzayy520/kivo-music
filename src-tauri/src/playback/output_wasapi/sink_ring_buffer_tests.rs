use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::errors::PlaybackError;
use crate::playback::output::{AudioOutputFrame, OutputSettings, OutputSink};

use super::frame_bridge::FrameBridgeError;
use super::ring_buffer::errors::RingBufferError;
use super::sink::{WasapiOutputSink, WasapiRingBufferPrepareError};

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
fn submit_frame_still_unsupported_after_prepare() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&OutputSettings::default()).unwrap();
    sink.prepare_ring_buffer_for_stream(&float32_stream(2, 44100), 1024)
        .unwrap();
    assert!(matches!(
        sink.submit_frame(dummy_frame()),
        Err(PlaybackError::UnsupportedOperation(ref msg)) if msg.contains("submit_frame")
    ));
    assert!(sink.has_ring_buffer());
    assert_eq!(sink.ring_buffer_available_frames(), Some(0));
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
    assert!(sink
        .prepare_ring_buffer_for_stream(&float32_stream(2, 44100), 1024)
        .is_ok());
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
fn prepare_ring_buffer_bridge_errors() {
    let mut sink = WasapiOutputSink::new();
    assert_bridge_err(
        sink.prepare_ring_buffer_for_stream(&float32_stream(0, 44100), 1024),
        FrameBridgeError::ZeroChannels,
    );
    assert_bridge_err(
        sink.prepare_ring_buffer_for_stream(&float32_stream(2, 0), 1024),
        FrameBridgeError::ZeroSampleRate,
    );
    let bad = AudioStreamInfo {
        sample_rate_hz: 44100,
        channels: 2,
        sample_format: AudioSampleFormat::Signed16,
    };
    assert_bridge_err(
        sink.prepare_ring_buffer_for_stream(&bad, 1024),
        FrameBridgeError::UnsupportedSampleFormat,
    );
}
#[test]
fn prepare_ring_buffer_fails_zero_capacity() {
    let mut sink = WasapiOutputSink::new();
    assert_buffer_err(
        sink.prepare_ring_buffer_for_stream(&float32_stream(2, 44100), 0),
        RingBufferError::InvalidCapacity,
    );
}
#[test]
fn prepare_ring_buffer_does_not_change_runtime_state() {
    let mut sink = WasapiOutputSink::new();
    let before = sink.status();
    assert!(!before.is_open && !before.is_active && before.pending_frames == 0);
    sink.prepare_ring_buffer_for_stream(&float32_stream(2, 44100), 1024)
        .unwrap();
    let after = sink.status();
    assert!(!after.is_open && !after.is_active && after.pending_frames == 0);
    assert!(sink.has_ring_buffer());
}
#[test]
fn open_after_prepare_preserves_ring_buffer() {
    let mut sink = WasapiOutputSink::new();
    sink.prepare_ring_buffer_for_stream(&float32_stream(2, 44100), 1024)
        .unwrap();
    sink.open(&OutputSettings::default()).unwrap();
    assert!(sink.has_ring_buffer());
    let s = sink.status();
    assert!(s.is_open && s.is_active);
}
#[test]
fn ring_buffer_flush_and_stop() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&OutputSettings::default()).unwrap();
    sink.prepare_ring_buffer_for_stream(&float32_stream(2, 44100), 1024)
        .unwrap();
    sink.flush().unwrap();
    assert!(sink.has_ring_buffer());
    assert_eq!(sink.ring_buffer_available_frames(), Some(0));
    sink.stop().unwrap();
    assert!(sink.has_ring_buffer());
    assert_eq!(sink.ring_buffer_available_frames(), Some(0));
}
#[test]
fn close_drops_and_prepare_recreates() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&OutputSettings::default()).unwrap();
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
    assert!(sink
        .prepare_ring_buffer_for_stream(&float32_stream(1, 48000), 2048)
        .is_ok());
}
