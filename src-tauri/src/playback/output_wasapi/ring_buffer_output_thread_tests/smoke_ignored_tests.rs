// smoke_ignored_tests.rs
//
// Windows-only ignored smoke test for the ring buffer output thread probe.
// This test only runs with `cargo test -- --ignored` and only performs
// real COM operations when KIVO_WASAPI_RING_BUFFER_OUTPUT_THREAD_SMOKE=1.

use super::smoke_assertions;
use crate::playback::output_wasapi::ring_buffer_output_thread::probe_ring_buffer_output_thread_smoke;

#[cfg(windows)]
#[test]
#[ignore]
fn ring_buffer_output_thread_smoke_windows_ignored_opt_in() {
    // This test is #[ignore] and only runs with:
    //   cargo test -- --ignored
    //
    // Even then, it only performs real COM operations if:
    //   KIVO_WASAPI_RING_BUFFER_OUTPUT_THREAD_SMOKE=1
    //
    // If the env is not set, it passes with a skip report.
    // It never panics or hard-fails.
    //
    // It does NOT:
    // - Produce sound
    // - Connect OutputSink
    // - Expose PlaybackCapabilities
    // - Connect decoder
    // - Connect pipeline
    // - Perform silent loop

    let report = probe_ring_buffer_output_thread_smoke();

    if !report.opt_in_enabled {
        smoke_assertions::assert_skipped_or_silent_success(&report);
        eprintln!("ring_buffer_output_thread smoke skipped: env not set");
        return;
    }

    assert!(report.attempted, "should have attempted when env is set");

    if report.skipped {
        smoke_assertions::assert_skipped_or_silent_success(&report);
        eprintln!(
            "ring_buffer_output_thread smoke skipped: {:?} - {:?}",
            report.skipped_reason, report.error_message
        );
        return;
    }

    // Probe succeeded
    smoke_assertions::assert_success_wasapi_lifecycle(&report);
    smoke_assertions::assert_success_ring_buffer(&report);
    smoke_assertions::assert_success_buffer_start_stop_reset(&report);
    smoke_assertions::assert_success_ring_buffer_closed(&report);
    smoke_assertions::assert_no_real_playback(&report);
    smoke_assertions::assert_ring_buffer_silence_behavior(&report);
    smoke_assertions::assert_prohibited_always_false(&report);
    smoke_assertions::assert_thread_lifecycle_success(&report);

    eprintln!(
        "ring_buffer_output_thread smoke passed: \
         buffer_size={} frames, silence_filled={}, padding={} frames",
        report.buffer_size_frames.unwrap(),
        report.total_silence_frames_filled,
        report.current_padding_frames.unwrap()
    );
}

#[cfg(not(windows))]
#[test]
#[ignore]
fn ring_buffer_output_thread_smoke_non_windows_returns_skipped() {
    // On non-Windows, probe should always return skipped.
    let report = probe_ring_buffer_output_thread_smoke();
    smoke_assertions::assert_skipped_or_silent_success(&report);
    eprintln!("ring_buffer_output_thread smoke skipped: non-windows platform");
}
