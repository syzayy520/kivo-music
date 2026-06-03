// output_thread_boundary/output_thread_stub.rs
//
// Non-Windows stub for output thread boundary smoke.
//
// This module provides a stub implementation for non-Windows platforms
// that always returns a "skipped_non_windows" report.

use super::report::WasapiOutputThreadSmokeReport;

/// Probe output thread boundary on non-Windows platforms.
///
/// Always returns a "skipped_non_windows" report since WASAPI is Windows-only.
pub fn probe_output_thread_boundary() -> WasapiOutputThreadSmokeReport {
    WasapiOutputThreadSmokeReport::skipped_non_windows()
}
