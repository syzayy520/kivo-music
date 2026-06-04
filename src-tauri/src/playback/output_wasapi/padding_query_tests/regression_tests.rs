// regression_tests.rs
//
// Regression tests ensuring existing functionality is not broken by padding query smoke.

use crate::playback::engine::PlaybackEngine;
use crate::playback::output::OutputSink;
use crate::playback::output_wasapi::sink::WasapiOutputSink;

#[test]
fn wasapi_output_sink_scaffold_succeeds() {
    let mut sink = WasapiOutputSink::new();
    // Scaffold open should succeed
    let settings = crate::playback::output::OutputSettings::default();
    assert!(sink.open(&settings).is_ok());
}

#[test]
fn native_output_still_uses_null_sink_by_default() {
    // Verify that native output still uses null sink
    // This is a regression test to ensure we didn't accidentally enable real output
    let mut engine = crate::playback::backends::native::KivoNativeEngine::new();
    // NativeEngine should still return UnsupportedOperation for play
    assert!(engine.play().is_err());
}

#[test]
fn capabilities_remain_default() {
    // Verify that PlaybackCapabilities are still all false/default
    let caps = crate::playback::capabilities::PlaybackCapabilities::default();
    assert!(!caps.can_seek);
    assert!(!caps.can_select_output_device);
    assert!(!caps.can_use_exclusive_output);
    assert!(!caps.can_probe_metadata);
    assert!(!caps.can_gapless);
    assert!(!caps.can_replaygain);
}

#[test]
fn public_native_engine_remains_typed_unsupported() {
    // Verify that public NativeEngine still returns typed UnsupportedOperation
    let mut engine = crate::playback::backends::native::KivoNativeEngine::new();
    let result = engine.play();
    assert!(result.is_err());
    // Should be a typed UnsupportedOperation error
    match result {
        Err(crate::playback::errors::PlaybackError::UnsupportedOperation(_)) => {}
        _ => panic!("Expected UnsupportedOperation error"),
    }
}

#[test]
fn start_stop_smoke_boundary_still_does_not_query_padding_by_default() {
    // Verify that start_stop smoke boundary doesn't call GetCurrentPadding
    // This is a regression test to ensure we didn't accidentally add GetCurrentPadding to start_stop
    let report = crate::playback::output_wasapi::start_stop::WasapiStartStopSmokeReport::base_report_for_windows();
    assert!(!report.get_current_padding_called);
}
