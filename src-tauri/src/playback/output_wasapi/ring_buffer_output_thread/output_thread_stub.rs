// ring_buffer_output_thread/output_thread_stub.rs
//
// Non-Windows stub for ring buffer output thread smoke.

use super::report::WasapiRingBufferOutputThreadSmokeReport;

/// Probe ring buffer output thread on non-Windows platforms.
///
/// Always returns a "skipped_non_windows" report since WASAPI is Windows-only.
pub fn probe_ring_buffer_output_thread_smoke() -> WasapiRingBufferOutputThreadSmokeReport {
    WasapiRingBufferOutputThreadSmokeReport::skipped_non_windows()
}
