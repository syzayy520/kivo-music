// device_stub.rs
//
// Non-Windows fallback for endpoint smoke.
//
// On non-Windows platforms, WASAPI is not available.
// This stub returns a skipped report without attempting any COM operations.

use super::WasapiEndpointSmokeReport;

/// Probe the default audio render endpoint on non-Windows platforms.
///
/// Always returns a skipped report indicating the platform is unsupported.
/// Does not reference any Windows types or call any COM APIs.
pub fn probe_default_endpoint() -> WasapiEndpointSmokeReport {
    WasapiEndpointSmokeReport::skipped_non_windows()
}
