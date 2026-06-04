// output_wasapi_root_tests/sink_tests.rs
//
// Tests for WasapiOutputSink typed unsupported behavior.

use super::support::{assert_unsupported, test_frame};
use crate::playback::output::{OutputSettings, OutputSink};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

#[test]
fn wasapi_scaffold_open_returns_ok() {
    let mut sink = WasapiOutputSink::new();

    let result = sink.open(&OutputSettings::default());
    assert!(result.is_ok(), "scaffold open should succeed");

    let status = result.unwrap();
    assert!(status.is_open, "scaffold should report is_open after open");
    assert!(status.is_active, "scaffold should report is_active after open");
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
fn wasapi_scaffold_state_transitions_succeed() {
    let mut sink = WasapiOutputSink::new();
    sink.open(&OutputSettings::default()).unwrap();

    // Scaffold state transitions should all succeed
    assert!(sink.pause().is_ok());
    assert!(sink.resume().is_ok());
    assert!(sink.flush().is_ok());
    assert!(sink.set_volume(0.5).is_ok());
    assert!(sink.set_muted(true).is_ok());

    // stop sets inactive
    let status = sink.stop().unwrap();
    assert!(!status.is_active, "stop should set inactive");
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
