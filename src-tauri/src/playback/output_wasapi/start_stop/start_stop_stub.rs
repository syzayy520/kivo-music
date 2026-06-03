// start_stop_stub.rs
//
// Non-Windows fallback for start/stop smoke.
//
// On non-Windows platforms, WASAPI is not available.
// This stub returns a skipped report without attempting any COM operations.

use super::WasapiStartStopSmokeReport;

/// Probe IAudioClient::Start + Stop on non-Windows platforms.
///
/// Always returns a skipped report indicating the platform is unsupported.
pub fn probe_start_stop() -> WasapiStartStopSmokeReport {
    WasapiStartStopSmokeReport::skipped_non_windows()
}
