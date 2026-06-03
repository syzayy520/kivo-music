// render_client_stub.rs
//
// Non-Windows fallback for render client smoke.
//
// On non-Windows platforms, WASAPI is not available.
// This stub returns a skipped report without attempting any COM operations.

use super::WasapiRenderClientSmokeReport;

/// Probe IAudioClient::GetService(IAudioRenderClient) on non-Windows platforms.
///
/// Always returns a skipped report indicating the platform is unsupported.
/// Does not reference any Windows types or call any COM APIs.
pub fn probe_render_client() -> WasapiRenderClientSmokeReport {
    WasapiRenderClientSmokeReport::skipped_non_windows()
}
