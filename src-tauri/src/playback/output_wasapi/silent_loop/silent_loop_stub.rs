// silent_loop_stub.rs
//
// Non-Windows stub for silent loop smoke.
//
// This module provides a safe stub implementation for non-Windows
// platforms that returns a skipped report.

use super::report::WasapiSilentLoopSmokeReport;

/// Probe silent loop on non-Windows platforms.
///
/// Returns a skipped report since silent loop is Windows-only.
pub fn probe_silent_loop() -> WasapiSilentLoopSmokeReport {
    WasapiSilentLoopSmokeReport::skipped_non_windows()
}
