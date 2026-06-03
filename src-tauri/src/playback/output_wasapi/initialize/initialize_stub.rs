// initialize_stub.rs
//
// Non-Windows fallback for initialize smoke.
//
// On non-Windows platforms, WASAPI is not available.
// This stub returns a skipped report without attempting any COM operations.

use super::WasapiClientInitializeSmokeReport;

/// Probe IAudioClient::Initialize on non-Windows platforms.
///
/// Always returns a skipped report indicating the platform is unsupported.
/// Does not reference any Windows types or call any COM APIs.
pub fn probe_initialize() -> WasapiClientInitializeSmokeReport {
    WasapiClientInitializeSmokeReport::skipped_non_windows()
}
