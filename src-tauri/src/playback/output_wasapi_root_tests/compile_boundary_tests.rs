// output_wasapi_root_tests/compile_boundary_tests.rs
//
// Tests for WASAPI compile boundary behavior.

use crate::playback::output_wasapi::platform::{wasapi_compile_boundary, WasapiCompileBoundary};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

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
