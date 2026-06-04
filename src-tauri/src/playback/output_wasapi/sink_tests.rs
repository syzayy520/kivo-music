use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::errors::PlaybackError;
use crate::playback::output::{AudioOutputFrame, OutputSink};

use super::sink::WasapiOutputSink;

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
