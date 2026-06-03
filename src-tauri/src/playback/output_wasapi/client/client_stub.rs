// client_stub.rs
//
// Non-Windows fallback for client activate smoke.
//
// On non-Windows platforms, WASAPI is not available.
// This stub returns a skipped report without attempting any COM operations.

use super::WasapiClientActivateSmokeReport;

/// Probe IAudioClient activation on non-Windows platforms.
///
/// Always returns a skipped report indicating the platform is unsupported.
/// Does not reference any Windows types or call any COM APIs.
pub fn probe_client_activate() -> WasapiClientActivateSmokeReport {
    WasapiClientActivateSmokeReport::skipped_non_windows()
}
