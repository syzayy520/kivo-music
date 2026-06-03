use crate::playback::backends::native::KivoNativeEngine;
use crate::playback::capabilities::PlaybackCapabilities;
use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};
use crate::playback::engine::PlaybackEngine;
use crate::playback::errors::PlaybackError;
use crate::playback::native_output::KivoNativeOutputSink;
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings, OutputSink};
use crate::playback::output_wasapi::platform::{wasapi_compile_boundary, WasapiCompileBoundary};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

fn test_frame() -> AudioOutputFrame {
    AudioOutputFrame {
        stream: AudioStreamInfo {
            sample_rate_hz: 48_000,
            channels: 2,
            sample_format: AudioSampleFormat::Float32,
        },
        position_ms: 0,
        samples: vec![0.0, 0.1],
    }
}

fn assert_unsupported(result: Result<OutputRuntimeStatus, PlaybackError>, operation: &str) {
    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert!(
                message.contains("WASAPI output is not implemented yet"),
                "Expected WASAPI not implemented message, got: {message}"
            );
            assert!(
                message.contains(operation),
                "Expected operation '{operation}' in message, got: {message}"
            );
        }
        Err(other) => panic!("Expected UnsupportedOperation, got: {other}"),
        Ok(_) => panic!("Expected UnsupportedOperation, got success"),
    }
}

#[test]
fn wasapi_stub_open_returns_typed_unsupported() {
    let mut sink = WasapiOutputSink::new();

    let result = sink.open(&OutputSettings::default());
    assert_unsupported(result, "open");

    let status = sink.status();
    assert!(!status.is_open, "stub should not report is_open");
    assert!(!status.is_active, "stub should not report is_active");
}

#[test]
fn wasapi_stub_submit_frame_returns_typed_unsupported() {
    let mut sink = WasapiOutputSink::new();

    let result = sink.submit_frame(test_frame());
    assert_unsupported(result, "submit_frame");

    // Stub should not increment pending frames
    let status = sink.status();
    assert_eq!(
        status.pending_frames, 0,
        "stub should not track pending frames"
    );
}

#[test]
fn wasapi_stub_pause_resume_flush_stop_are_typed_unsupported() {
    let mut sink = WasapiOutputSink::new();

    assert_unsupported(sink.pause(), "pause");
    assert_unsupported(sink.resume(), "resume");
    assert_unsupported(sink.flush(), "flush");
    assert_unsupported(sink.stop(), "stop");
    assert_unsupported(sink.set_volume(0.5), "set_volume");
    assert_unsupported(sink.set_muted(true), "set_muted");

    // Status should remain not-open after all operations
    let status = sink.status();
    assert!(
        !status.is_open,
        "stub should not report is_open after operations"
    );
    assert!(
        !status.is_active,
        "stub should not report is_active after operations"
    );
}

#[test]
fn wasapi_stub_close_is_idempotent() {
    let mut sink = WasapiOutputSink::new();

    // Close should succeed
    sink.close().expect("first close should succeed");

    // Close again should also succeed (idempotent)
    sink.close().expect("second close should succeed");

    // Close should not pretend device is open
    let status = sink.status();
    assert!(
        !status.is_open,
        "stub should not report is_open after close"
    );
    assert!(
        !status.is_active,
        "stub should not report is_active after close"
    );
}

#[test]
fn wasapi_stub_status_never_reports_real_device_open() {
    let mut sink = WasapiOutputSink::new();

    // Initial status
    let status = sink.status();
    assert!(!status.is_open, "initial status should not be open");
    assert!(!status.is_active, "initial status should not be active");

    // After open attempt
    let _ = sink.open(&OutputSettings::default());
    let status = sink.status();
    assert!(!status.is_open, "status after open should not be open");
    assert!(!status.is_active, "status after open should not be active");

    // With device ID set
    let mut settings = OutputSettings::default();
    settings.selected_device_id = Some("fake-device".to_string());
    let _ = sink.open(&settings);
    let status = sink.status();
    assert!(!status.is_open, "status with device ID should not be open");
    assert_eq!(
        status.active_device_id.as_deref(),
        Some("fake-device"),
        "active_device_id should reflect config"
    );
}

#[test]
fn wasapi_config_is_created_from_output_settings() {
    let mut settings = OutputSettings::default();
    settings.selected_device_id = Some("test-device".to_string());
    settings.exclusive_mode = true;
    settings.bit_perfect_mode = true;

    let mut sink = WasapiOutputSink::new();
    let _ = sink.open(&settings);

    let config = sink.config();
    assert_eq!(config.selected_device_id.as_deref(), Some("test-device"));
    assert!(config.exclusive_mode);
    assert!(config.bit_perfect_mode);
}

#[test]
fn native_output_still_uses_null_sink_by_default() {
    let mut sink = KivoNativeOutputSink::new();

    // Should open Null Sink successfully
    let status = sink
        .open(&OutputSettings::default())
        .expect("KivoNativeOutputSink should open Null Sink");

    assert!(status.is_open, "Null Sink should report is_open");
    assert!(status.is_active, "Null Sink should report is_active");

    // Submit frame should succeed
    let status = sink
        .submit_frame(test_frame())
        .expect("Null Sink should accept frames");
    assert_eq!(
        status.pending_frames, 1,
        "Null Sink should track pending frames"
    );
}

#[test]
fn null_sink_regression_still_passes() {
    // This is a duplicate of the existing null_sink_open_succeeds_and_sets_boundary_active test
    // to ensure Null Sink behavior is unchanged.
    let mut sink = KivoNativeOutputSink::new();

    let status = sink
        .open(&OutputSettings::default())
        .expect("null sink open should succeed");

    assert!(status.is_open);
    assert!(status.is_active);
    assert!(status.last_error.is_none());
}

#[test]
fn public_native_engine_remains_typed_unsupported() {
    let mut engine = KivoNativeEngine::new();

    // All public methods should return UnsupportedOperation
    let track = crate::playback::types::PlaybackTrack {
        id: crate::playback::types::TrackId("test".to_string()),
        title: "Test".to_string(),
        artist: "Test".to_string(),
        source_path: "test.wav".to_string(),
    };

    assert!(matches!(
        engine.load(track),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.play(),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.pause(),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.resume(),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.stop(),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.seek(0),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.set_volume(1.0),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
    assert!(matches!(
        engine.set_muted(false),
        Err(PlaybackError::UnsupportedOperation(_))
    ));
}

#[test]
fn capabilities_remain_default() {
    let caps = PlaybackCapabilities::default();

    assert!(!caps.can_seek, "can_seek should be false");
    assert!(
        !caps.can_select_output_device,
        "can_select_output_device should be false"
    );
    assert!(
        !caps.can_use_exclusive_output,
        "can_use_exclusive_output should be false"
    );
    assert!(
        !caps.can_probe_metadata,
        "can_probe_metadata should be false"
    );
    assert!(!caps.can_gapless, "can_gapless should be false");
    assert!(!caps.can_replaygain, "can_replaygain should be false");
}

// ── P0-018: WASAPI Compile Boundary Tests ──────────────────────────────────

#[test]
fn compile_boundary_probe_returns_valid_descriptor() {
    let boundary = wasapi_compile_boundary();

    // The boundary descriptor should always be constructible
    let description = boundary.describe();
    assert!(
        !description.is_empty(),
        "compile boundary description should not be empty"
    );
    assert!(
        description.contains("WASAPI compile boundary"),
        "description should mention WASAPI compile boundary, got: {description}"
    );
}

#[test]
fn compile_boundary_is_windows_target_matches_platform() {
    let boundary = WasapiCompileBoundary::new();

    #[cfg(windows)]
    {
        assert!(
            boundary.is_windows_target(),
            "on Windows, is_windows_target() should return true"
        );
    }

    #[cfg(not(windows))]
    {
        assert!(
            !boundary.is_windows_target(),
            "on non-Windows, is_windows_target() should return false"
        );
    }
}

#[test]
fn wasapi_output_sink_exposes_compile_boundary() {
    let sink = WasapiOutputSink::new();
    let boundary = sink.compile_boundary();

    let description = boundary.describe();
    assert!(
        !description.is_empty(),
        "sink compile boundary description should not be empty"
    );
}

#[test]
fn compile_boundary_default_matches_new() {
    let from_new = WasapiCompileBoundary::new();
    let from_default = WasapiCompileBoundary::default();

    assert_eq!(
        from_new.is_windows_target(),
        from_default.is_windows_target(),
        "new() and default() should agree on is_windows_target"
    );
    assert_eq!(
        from_new.describe(),
        from_default.describe(),
        "new() and default() should produce same description"
    );
}

#[cfg(windows)]
#[test]
fn compile_boundary_windows_holds_wave_format() {
    let boundary = WasapiCompileBoundary::new();

    let wave_format = boundary.wave_format();
    // Copy packed struct fields to local variables (packed struct fields
    // cannot be directly referenced due to alignment requirements).
    let format_tag = wave_format.wFormatTag;
    let channels = wave_format.nChannels;
    let sample_rate = wave_format.nSamplesPerSec;
    // Default WAVEFORMATEX should have zeroed fields
    assert_eq!(format_tag, 0, "default WAVEFORMATEX wFormatTag should be 0");
    assert_eq!(channels, 0, "default WAVEFORMATEX nChannels should be 0");
    assert_eq!(
        sample_rate, 0,
        "default WAVEFORMATEX nSamplesPerSec should be 0"
    );
}

#[cfg(windows)]
#[test]
fn compile_boundary_windows_holds_share_mode() {
    let boundary = WasapiCompileBoundary::new();

    let share_mode = boundary.share_mode();
    // Copy packed struct field to local variable
    let mode_value = share_mode.0;
    // Default AUDCLNT_SHAREMODE should be zeroed (shared mode = 0)
    assert_eq!(mode_value, 0, "default AUDCLNT_SHAREMODE value should be 0");
}
