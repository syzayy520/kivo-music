// reset_boundary_stub.rs
//
// Non-Windows stub for reset boundary smoke.
//
// This module provides a safe stub implementation for non-Windows
// platforms that returns a skipped report.

use super::report::WasapiResetBoundarySmokeReport;

/// Probe reset boundary on non-Windows platforms.
///
/// Returns a skipped report since reset boundary is Windows-only.
pub fn probe_reset_boundary() -> WasapiResetBoundarySmokeReport {
    WasapiResetBoundarySmokeReport::skipped_non_windows()
}
