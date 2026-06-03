// smoke_ignored_tests.rs
//
// Windows-only ignored smoke test for the WASAPI output thread boundary probe.
// This test only runs with `cargo test -- --ignored` and only performs
// real COM operations when KIVO_WASAPI_OUTPUT_THREAD_SMOKE=1.

use super::smoke_assertions;
use crate::playback::output_wasapi::output_thread_boundary::probe_output_thread_boundary;

#[cfg(windows)]
#[test]
#[ignore]
fn output_thread_boundary_smoke_windows_ignored_opt_in() {
    // This test is #[ignore] and only runs with:
    //   cargo test -- --ignored
    //
    // Even then, it only performs real COM operations if:
    //   KIVO_WASAPI_OUTPUT_THREAD_SMOKE=1
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
    // - Register callbacks
    // - Create ring buffers
    // - Connect decoder
    // - Connect pipeline
    // - Perform silent loop
    // - Create async runtime

    let report = probe_output_thread_boundary();

    if !report.opt_in_enabled {
        smoke_assertions::assert_skipped_when_env_not_set(&report);
        eprintln!("output_thread_boundary smoke skipped: env not set");
        return;
    }

    assert!(report.attempted, "should have attempted when env is set");

    if report.skipped {
        smoke_assertions::assert_skipped_when_attempted_but_skipped(&report);
        eprintln!(
            "output_thread_boundary smoke skipped: {:?} - {:?}",
            report.skipped_reason, report.error_message
        );
        return;
    }

    // Probe succeeded
    smoke_assertions::assert_success_prereq_steps(&report);
    smoke_assertions::assert_success_buffer_and_prefill(&report);
    smoke_assertions::assert_success_start_stop_reset(&report);
    smoke_assertions::assert_prohibited_always_false(&report);

    // Thread lifecycle assertions
    assert!(report.thread_spawn_attempted, "thread spawn attempted");
    assert!(report.thread_spawned, "thread spawned");
    assert!(report.thread_report_received, "thread report received");
    assert!(
        report.thread_recv_timeout_ms.is_some(),
        "recv timeout present"
    );
    assert!(!report.thread_recv_timed_out, "recv not timed out");
    assert!(
        report.thread_duration_ms.is_some(),
        "thread duration present"
    );
    assert!(!report.thread_panic_caught, "no thread panic");
    assert!(!report.thread_join_failed, "thread join not failed");

    eprintln!(
        "output_thread_boundary smoke passed: reset succeeded with padding={} frames, \
         buffer_size={} frames, thread_duration={}ms",
        report.current_padding_frames.unwrap(),
        report.buffer_size_frames.unwrap(),
        report.thread_duration_ms.unwrap()
    );
}

#[cfg(not(windows))]
#[test]
#[ignore]
fn output_thread_boundary_smoke_non_windows_returns_skipped() {
    // On non-Windows, probe_output_thread_boundary should always return skipped.
    let report = probe_output_thread_boundary();
    smoke_assertions::assert_skipped_when_env_not_set(&report);
    eprintln!("output_thread_boundary smoke skipped: non-windows platform");
}
