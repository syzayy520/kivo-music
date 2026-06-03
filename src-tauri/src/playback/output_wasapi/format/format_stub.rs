// format_stub.rs
//
// Non-Windows fallback for mix format smoke.
//
// On non-Windows platforms, WASAPI is not available.
// This stub returns a skipped report without attempting any COM operations.

use super::WasapiMixFormatSmokeReport;

/// Probe IAudioClient::GetMixFormat on non-Windows platforms.
///
/// Always returns a skipped report indicating the platform is unsupported.
/// Does not reference any Windows types or call any COM APIs.
pub fn probe_mix_format() -> WasapiMixFormatSmokeReport {
    WasapiMixFormatSmokeReport::skipped_non_windows()
}
