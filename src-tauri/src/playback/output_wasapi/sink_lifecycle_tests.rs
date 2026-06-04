use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::errors::PlaybackError;
use crate::playback::output::{AudioOutputFrame, OutputSettings, OutputSink};

use super::sink::WasapiOutputSink;

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
#[test]
fn open_sets_state_no_ring_buffer() {
    let mut sink = WasapiOutputSink::new();
    let status = sink.open(&OutputSettings::default()).unwrap();
    assert!(status.is_open);
    assert!(status.is_active);
    assert!(status.last_error.is_none());
    assert!(!sink.has_ring_buffer());
}
#[test]
fn submit_frame_returns_unsupported_operation() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&OutputSettings::default()).unwrap();
    assert!(matches!(
        sink.submit_frame(dummy_frame()),
        Err(PlaybackError::UnsupportedOperation(ref msg)) if msg.contains("submit_frame")
    ));
}
#[test]
fn pause_and_resume_ok() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&OutputSettings::default()).unwrap();
    let s = sink.pause().unwrap();
    assert!(s.is_open && s.is_active);
    let s = sink.resume().unwrap();
    assert!(s.is_open && s.is_active);
}
#[test]
fn flush_and_stop_lifecycle() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&OutputSettings::default()).unwrap();
    assert_eq!(sink.flush().unwrap().pending_frames, 0);
    let s = sink.stop().unwrap();
    assert!(!s.is_active);
    assert_eq!(s.pending_frames, 0);
}
#[test]
fn set_volume_updates_controls() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&OutputSettings::default()).unwrap();
    let s = sink.set_volume(0.5).unwrap();
    assert!((s.controls.volume_level - 0.5).abs() < f32::EPSILON);
}
#[test]
fn close_resets_all_state() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&OutputSettings::default()).unwrap();
    sink.close().unwrap();
    let s = sink.status();
    assert!(!s.is_open && !s.is_active && s.last_error.is_none() && s.pending_frames == 0);
}
