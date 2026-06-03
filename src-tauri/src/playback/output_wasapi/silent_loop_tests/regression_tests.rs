// silent_loop_tests/regression_tests.rs
//
// Regression tests ensuring existing WASAPI boundaries are not broken
// by the silent_loop module.

use crate::playback::output::OutputSink;

#[test]
fn wasapi_output_sink_still_stub() {
    // WasapiOutputSink should remain a stub (no real audio)
    let sink = crate::playback::output_wasapi::sink::WasapiOutputSink::new();
    // Verify it's still a stub by checking status returns default
    let status = sink.status();
    // The stub should not have opened any real device
    assert!(!status.is_open, "WasapiOutputSink status should not be open");
    assert!(!status.is_active, "WasapiOutputSink status should not be active");
}

#[test]
fn padding_query_boundary_unchanged() {
    // Verify padding_query module still exists and has the expected function
    let report = crate::playback::output_wasapi::padding_query::probe_padding_query();
    // Without env set, it should return a skipped report
    assert!(report.skipped, "padding_query should skip without env");
    assert_eq!(report.query_mode, "started");
}

#[test]
fn start_stop_boundary_unchanged() {
    // Verify start_stop module still exists and has the expected function
    let report = crate::playback::output_wasapi::start_stop::probe_start_stop();
    // Without env set, it should return a skipped report
    assert!(report.skipped, "start_stop should skip without env");
}

#[test]
fn silent_loop_stub_returns_skipped_on_non_windows() {
    // On any platform (including Windows without env), probe_silent_loop
    // should return a report. On non-Windows it's always skipped.
    #[cfg(not(windows))]
    {
        let report = crate::playback::output_wasapi::silent_loop::probe_silent_loop();
        assert!(report.skipped);
        assert_eq!(report.skipped_reason, Some("non-windows platform"));
    }
}
