// padding_query_tests/smoke_ignored_tests.rs
//
// Windows-only ignored smoke test for the WASAPI padding query probe.
// This test only runs with `cargo test -- --ignored` and only performs
// real COM operations when KIVO_WASAPI_PADDING_QUERY_SMOKE=1.

use super::smoke_assertions;
use crate::playback::output_wasapi::padding_query::probe_padding_query;

#[cfg(windows)]
#[test]
#[ignore]
fn padding_query_smoke_windows_ignored_opt_in() {
    // This test is #[ignore] and only runs with:
    //   cargo test -- --ignored
    //
    // Even then, it only performs real COM operations if:
    //   KIVO_WASAPI_PADDING_QUERY_SMOKE=1
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

    let report = probe_padding_query();

    if !report.opt_in_enabled {
        smoke_assertions::assert_skipped_when_env_not_set(&report);
        eprintln!("padding_query smoke skipped: env not set");
        return;
    }

    assert!(report.attempted, "should have attempted when env is set");

    if report.skipped {
        smoke_assertions::assert_skipped_when_attempted_but_skipped(&report);
        eprintln!(
            "padding_query smoke skipped: {:?} - {:?}",
            report.skipped_reason, report.error_message
        );
        return;
    }

    // Probe succeeded
    smoke_assertions::assert_success_prereq_steps(&report);
    smoke_assertions::assert_success_buffer_and_prefill(&report);
    smoke_assertions::assert_success_start_stop_and_padding(&report);
    smoke_assertions::assert_prohibited_always_false(&report);

    eprintln!(
        "padding_query smoke passed: GetCurrentPadding succeeded, padding={} frames, buffer_size={} frames",
        report.padding_frames.unwrap(),
        report.buffer_size_frames.unwrap()
    );
}
