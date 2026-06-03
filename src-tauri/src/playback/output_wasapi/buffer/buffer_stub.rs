// buffer_stub.rs
//
// Non-Windows fallback for buffer smoke.
//
// On non-Windows platforms, WASAPI is not available.
// This stub returns a skipped report without attempting any COM operations.

use super::WasapiBufferSmokeReport;

/// Probe IAudioRenderClient::GetBuffer + ReleaseBuffer on non-Windows platforms.
///
/// Always returns a skipped report indicating the platform is unsupported.
/// Does not reference any Windows types or call any COM APIs.
pub fn probe_buffer() -> WasapiBufferSmokeReport {
    WasapiBufferSmokeReport::skipped_non_windows()
}
