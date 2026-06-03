// smoke_ignored_tests.rs
//
// Windows-only ignored smoke test for the WASAPI reset boundary probe.
// This test only runs with `cargo test -- --ignored` and only performs
// real COM operations when KIVO_WASAPI_RESET_SMOKE=1.

use super::smoke_assertions;
use crate::playback::output_wasapi::reset_boundary::probe_reset_boundary;

#[cfg(windows)]
#[test]
#[ignore]
fn reset_boundary_smoke_windows_ignored_opt_in() {
    // This test is #[ignore] and only runs with:
    //   cargo test -- --ignored
    //
    // Even then, it only performs real COM operations if:
    //   KIVO_WASAPI_RESET_SMOKE=1
    //
    // If the env is not set, it passes with a skip report.
    // It never panics or hard-fails.
    //
    // It does NOT:
    // - Call IsFormatSupported
    // - Call Reset before Stop
    // - Produce sound
    // - Connect OutputSink
    // - Expose PlaybackCapabilities
    // - Create threads or async runtime
    // - Register callbacks
    // - Create ring buffers
    // - Connect decoder
    // - Connect pipeline
    // - Perform silent loop

    let report = probe_reset_boundary();

    if !report.opt_in_enabled {
        smoke_assertions::assert_skipped_when_env_not_set(&report);
        eprintln!("reset_boundary smoke skipped: env not set");
        return;
    }

    assert!(report.attempted, "should have attempted when env is set");

    if report.skipped {
        smoke_assertions::assert_skipped_when_attempted_but_skipped(&report);
        eprintln!(
            "reset_boundary smoke skipped: {:?} - {:?}",
            report.skipped_reason, report.error_message
        );
        return;
    }

    // Probe succeeded
    smoke_assertions::assert_success_prereq_steps(&report);
    smoke_assertions::assert_success_buffer_and_prefill(&report);
    smoke_assertions::assert_success_start_stop_reset(&report);
    smoke_assertions::assert_prohibited_always_false(&report);

    eprintln!(
        "reset_boundary smoke passed: reset succeeded with hresult={}, \
         padding={} frames, buffer_size={} frames",
        report.reset_hresult.unwrap(),
        report.current_padding_frames.unwrap(),
        report.buffer_size_frames.unwrap()
    );
}

#[cfg(not(windows))]
#[test]
#[ignore]
fn reset_boundary_smoke_non_windows_returns_skipped() {
    // On non-Windows, probe_reset_boundary should always return skipped.
    let report = probe_reset_boundary();
    smoke_assertions::assert_skipped_when_env_not_set(&report);
    eprintln!("reset_boundary smoke skipped: non-windows platform");
}
