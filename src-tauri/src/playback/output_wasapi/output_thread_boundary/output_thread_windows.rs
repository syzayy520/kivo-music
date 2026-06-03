// output_thread_boundary/output_thread_windows.rs
//
// Windows-only output thread boundary smoke implementation.
//
// This module is the ONLY place in the output_thread_boundary boundary that performs
// the full WASAPI probe with Stop + Reset after a single GetCurrentPadding
// in an independent output thread.
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::IsFormatSupported
//   - Non-silent audio data
//   - Silent loop
//   - Ring buffer / decoder / pipeline
//   - OutputSink / PlaybackCapabilities
//   - Async runtime

use std::time::Duration;

use super::env::is_opt_in_enabled;
use super::output_thread_flow::run_output_thread_flow;
use super::output_thread_outcome::apply_thread_outcome;
use super::report::WasapiOutputThreadSmokeReport;
use super::thread_runner::{recv_timeout_and_join, spawn_output_thread_probe};

/// Timeout for waiting for the output thread to complete.
const OUTPUT_THREAD_TIMEOUT: Duration = Duration::from_secs(15);

/// Probe output thread boundary on Windows.
///
/// Returns a `WasapiOutputThreadSmokeReport` describing the outcome.
pub fn probe_output_thread_boundary() -> WasapiOutputThreadSmokeReport {
    if !is_opt_in_enabled() {
        return WasapiOutputThreadSmokeReport::skipped_env_missing();
    }

    let mut report = WasapiOutputThreadSmokeReport {
        attempted: true,
        thread_spawn_attempted: true,
        ..Default::default()
    };

    // Spawn the output thread
    let (rx, handle) = spawn_output_thread_probe();
    report.thread_spawned = true;

    // Wait for the report with timeout
    let thread_report = recv_timeout_and_join(rx, handle, OUTPUT_THREAD_TIMEOUT);

    // Apply the thread outcome to the report
    apply_thread_outcome(
        &mut report,
        thread_report,
        OUTPUT_THREAD_TIMEOUT.as_millis() as u64,
    );

    report
}

/// Internal probe that runs inside the output thread.
///
/// This function runs the full WASAPI lifecycle probe:
/// COM init → endpoint → activate → GetMixFormat → Initialize
/// → GetService → GetBufferSize → GetBuffer(1) → ReleaseBuffer(1, SILENT)
/// → Start → GetCurrentPadding → Stop → Reset
///
/// Returns a `WasapiOutputThreadSmokeReport` describing the outcome.
pub fn probe_output_thread_boundary_internal() -> WasapiOutputThreadSmokeReport {
    run_output_thread_flow()
}
