// output_wasapi_root_tests/sink_tests.rs
//
// Tests for WasapiOutputSink typed unsupported behavior.

use super::support::{assert_unsupported, test_frame};
use crate::playback::output::{OutputSettings, OutputSink};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

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
