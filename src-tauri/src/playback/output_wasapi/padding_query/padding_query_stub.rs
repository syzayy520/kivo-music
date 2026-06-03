// padding_query_stub.rs
//
// Non-Windows stub for IAudioClient::GetCurrentPadding smoke.
//
// This module provides a safe stub implementation for non-Windows
// platforms that returns a skipped report.

use super::report::WasapiPaddingQuerySmokeReport;

/// Probe IAudioClient::GetCurrentPadding on non-Windows platforms.
///
/// Returns a skipped report since GetCurrentPadding is Windows-only.
pub fn probe_padding_query() -> WasapiPaddingQuerySmokeReport {
    WasapiPaddingQuerySmokeReport::skipped_non_windows()
}
