// ring_buffer_output_thread/output_thread_windows.rs
//
// Windows-only ring buffer output thread smoke implementation.
//
// This module is the ONLY place in the ring_buffer_output_thread boundary that performs
// the full WASAPI probe with ring buffer integration in an independent output thread.
//
// **PROHIBITED** (not implemented here):
//   - Non-silent audio data
//   - OutputSink / PlaybackCapabilities
//   - Decoder / pipeline / manager

use std::time::Duration;

use super::env::is_opt_in_enabled;
use super::output_thread_outcome::apply_thread_outcome;
use super::report::WasapiRingBufferOutputThreadSmokeReport;
use super::thread_runner::{recv_timeout_and_join, spawn_output_thread_probe};

/// Timeout for waiting for the output thread to complete.
const OUTPUT_THREAD_TIMEOUT: Duration = Duration::from_secs(15);

/// Probe ring buffer output thread on Windows.
pub fn probe_ring_buffer_output_thread_smoke() -> WasapiRingBufferOutputThreadSmokeReport {
    if !is_opt_in_enabled() {
        return WasapiRingBufferOutputThreadSmokeReport::skipped_env_missing();
    }

    let mut report = WasapiRingBufferOutputThreadSmokeReport {
        attempted: true,
        output_thread_spawn_attempted: true,
        ..Default::default()
    };

    // Spawn the output thread
    let (rx, handle) = spawn_output_thread_probe();
    report.output_thread_spawned = true;

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
