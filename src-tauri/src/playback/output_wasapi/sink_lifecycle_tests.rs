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

/// Test open behavior on non-Windows platforms.
///
/// On non-Windows: open() should fail with unsupported platform error.
/// On Windows: open() should succeed (if audio device available).
#[test]
fn open_attempts_real_device() {
    let mut sink = WasapiOutputSink::new();
    let result = sink.open(&OutputSettings::default());

    #[cfg(target_os = "windows")]
    {
        // On Windows, open may succeed or fail depending on device availability
        // We just verify it doesn't panic
        let _ = result;
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On non-Windows, open should fail with unsupported platform
        let status = result.unwrap();
        assert!(!status.is_open);
        assert!(!status.is_active);
        assert!(status.last_error.is_some());
        assert!(status.last_error.unwrap().contains("not supported"));
    }
}

/// Test that open populates status correctly.
#[test]
fn open_populates_status() {
    let mut sink = WasapiOutputSink::new();
    let _ = sink.open(&OutputSettings::default());

    let wasapi_status = sink.wasapi_status();
    assert!(wasapi_status.is_open_attempted);

    #[cfg(target_os = "windows")]
    {
        // On Windows, device may or may not be open depending on hardware
    }

    #[cfg(not(target_os = "windows"))]
    {
        assert!(!wasapi_status.is_real_device_open);
        assert!(!wasapi_status.is_render_client_acquired);
    }
}
/// Test that submit_frame still returns UnsupportedOperation.
///
/// This ticket does NOT implement PCM writes.
/// submit_frame should continue to return UnsupportedOperation.
#[test]
fn submit_frame_returns_unsupported_operation() {
    let mut sink = WasapiOutputSink::new();
    let _ = sink.open(&OutputSettings::default());
    assert!(matches!(
        sink.submit_frame(dummy_frame()),
        Err(PlaybackError::UnsupportedOperation(ref msg)) if msg.contains("submit_frame")
    ));
}
#[test]
fn pause_and_resume_ok() {
    let mut sink = WasapiOutputSink::new();
    let _ = sink.open(&OutputSettings::default());
    let s = sink.pause().unwrap();
    // On non-Windows, open fails so is_open/is_active are false
    #[cfg(target_os = "windows")]
    assert!(s.is_open && s.is_active);
    let s = sink.resume().unwrap();
    #[cfg(target_os = "windows")]
    assert!(s.is_open && s.is_active);
}
#[test]
fn flush_and_stop_lifecycle() {
    let mut sink = WasapiOutputSink::new();
    let _ = sink.open(&OutputSettings::default());
    assert_eq!(sink.flush().unwrap().pending_frames, 0);
    let s = sink.stop().unwrap();
    assert!(!s.is_active);
    assert_eq!(s.pending_frames, 0);
}
#[test]
fn set_volume_updates_controls() {
    let mut sink = WasapiOutputSink::new();
    let _ = sink.open(&OutputSettings::default());
    let s = sink.set_volume(0.5).unwrap();
    assert!((s.controls.volume_level - 0.5).abs() < f32::EPSILON);
}
/// Test that close resets all state.
#[test]
fn close_resets_all_state() {
    let mut sink = WasapiOutputSink::new();
    let _ = sink.open(&OutputSettings::default());
    sink.close().unwrap();
    let s = sink.status();
    assert!(!s.is_open && !s.is_active && s.last_error.is_none() && s.pending_frames == 0);
}

/// Test close idempotency: close before open.
#[test]
fn close_before_open_is_noop() {
    let mut sink = WasapiOutputSink::new();
    sink.close().unwrap();
    let s = sink.status();
    assert!(!s.is_open && !s.is_active && s.last_error.is_none());
}

/// Test close idempotency: close twice.
#[test]
fn close_twice_is_idempotent() {
    let mut sink = WasapiOutputSink::new();
    let _ = sink.open(&OutputSettings::default());
    sink.close().unwrap();
    sink.close().unwrap();
    let s = sink.status();
    assert!(!s.is_open && !s.is_active && s.last_error.is_none());
}

/// Test close after failed open.
#[test]
fn close_after_failed_open() {
    let mut sink = WasapiOutputSink::new();
    let _ = sink.open(&OutputSettings::default());
    // On non-Windows, open fails - close should still work
    sink.close().unwrap();
    let s = sink.status();
    assert!(!s.is_open && !s.is_active && s.last_error.is_none());
}

/// Test status transitions.
#[test]
fn status_transitions() {
    let mut sink = WasapiOutputSink::new();

    // Initial state
    let s = sink.status();
    assert!(!s.is_open && !s.is_active);

    // After open attempt
    let _ = sink.open(&OutputSettings::default());
    let wasapi_status = sink.wasapi_status();
    assert!(wasapi_status.is_open_attempted);

    // After close
    sink.close().unwrap();
    let s = sink.status();
    assert!(!s.is_open && !s.is_active && s.last_error.is_none());

    let wasapi_status = sink.wasapi_status();
    assert!(!wasapi_status.is_real_device_open);
    assert!(!wasapi_status.is_render_client_acquired);
}
