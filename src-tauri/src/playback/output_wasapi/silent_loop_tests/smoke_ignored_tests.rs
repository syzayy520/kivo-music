// silent_loop_tests/smoke_ignored_tests.rs
//
// Windows-only ignored smoke test for the WASAPI silent loop probe.
// This test only runs with `cargo test -- --ignored` and only performs
// real COM operations when KIVO_WASAPI_SILENT_LOOP_SMOKE=1.

use super::smoke_assertions;
use crate::playback::output_wasapi::silent_loop::probe_silent_loop;

#[cfg(windows)]
#[test]
#[ignore]
fn silent_loop_smoke_windows_ignored_opt_in() {
    // This test is #[ignore] and only runs with:
    //   cargo test -- --ignored
    //
    // Even then, it only performs real COM operations if:
    //   KIVO_WASAPI_SILENT_LOOP_SMOKE=1
    //
    // If the env is not set, it passes with a skip report.
    // It never panics or hard-fails.
    //
    // It does NOT:
    // - Call IsFormatSupported
    // - Call Reset
    // - Produce sound
    // - Connect OutputSink
    // - Expose PlaybackCapabilities
    // - Create threads or async runtime
    // - Register callbacks
    // - Create ring buffers
    // - Connect decoder
    // - Connect pipeline

    let report = probe_silent_loop();

    if !report.opt_in_enabled {
        smoke_assertions::assert_skipped_when_env_not_set(&report);
        eprintln!("silent_loop smoke skipped: env not set");
        return;
    }

    assert!(report.attempted, "should have attempted when env is set");

    if report.skipped {
        smoke_assertions::assert_skipped_when_attempted_but_skipped(&report);
        eprintln!(
            "silent_loop smoke skipped: {:?} - {:?}",
            report.skipped_reason, report.error_message
        );
        return;
    }

    // Probe succeeded
    smoke_assertions::assert_success_prereq_steps(&report);
    smoke_assertions::assert_success_buffer_and_prefill(&report);
    smoke_assertions::assert_success_start_stop_and_loop(&report);
    smoke_assertions::assert_prohibited_always_false(&report);

    eprintln!(
        "silent_loop smoke passed: loop completed {} iterations, \
         zero_available={}, buffer_size={} frames",
        report.loop_iterations_completed.unwrap(),
        report.zero_available_count.unwrap(),
        report.buffer_size_frames.unwrap()
    );
}

#[cfg(not(windows))]
#[test]
#[ignore]
fn silent_loop_smoke_non_windows_returns_skipped() {
    // On non-Windows, probe_silent_loop should always return skipped.
    let report = probe_silent_loop();
    smoke_assertions::assert_skipped_when_env_not_set(&report);
    eprintln!("silent_loop smoke skipped: non-windows platform");
}
