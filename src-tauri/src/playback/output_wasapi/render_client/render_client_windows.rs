// render_client_windows.rs
//
// Windows-only IAudioClient::GetService(IAudioRenderClient) smoke implementation.
//
// This module is the ONLY place in the codebase that calls
// IAudioClient::GetService. It performs a minimal smoke probe:
// get the default audio render endpoint, activate IAudioClient,
// call GetMixFormat, call Initialize in shared mode, call
// GetService to obtain IAudioRenderClient, then immediately
// drop everything.
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::IsFormatSupported
//   - IAudioRenderClient::GetBuffer / ReleaseBuffer
//   - IAudioClient::Start / Stop / Reset
//   - Audio playback
//   - Audio thread creation
//   - Async operations
//   - Format conversion
//   - Resampler
//   - Volume
//   - ReplayGain
//   - Clipping protection
//   - Device hotplug
//   - Exclusive mode
//   - Queue integration
//   - OutputSink real implementation
//   - PlaybackCapabilities change
//   - Settings exposure
//   - UI exposure
//   - Manager integration
//   - Pipeline integration
//   - Decoder changes

use super::env::is_opt_in_enabled;
use super::format_fields::extract_format_fields;
use super::guards::ComApartment;
use super::render_client_steps;
use super::report::WasapiRenderClientSmokeReport;

/// Probe IAudioClient::GetService(IAudioRenderClient) on Windows.
///
/// This function:
/// 1. Checks the opt-in environment variable
/// 2. Initializes COM (MTA)
/// 3. Creates an IMMDeviceEnumerator
/// 4. Calls GetDefaultAudioEndpoint(eRender, eConsole)
/// 5. Activates IAudioClient from the endpoint
/// 6. Calls GetMixFormat to obtain the mix format pointer
/// 7. Reads basic format fields into local variables
/// 8. Calls IAudioClient::Initialize in shared mode
/// 9. Calls IAudioClient::GetService to obtain IAudioRenderClient
/// 10. Drops IAudioRenderClient (RAII)
/// 11. Drops IAudioClient (RAII)
/// 12. Releases format pointer via MixFormatGuard (RAII)
/// 13. Cleans up COM via ComApartment (RAII)
///
/// It does NOT:
/// - Call IsFormatSupported
/// - Call GetBuffer / ReleaseBuffer
/// - Call Start / Stop / Reset
/// - Produce sound
/// - Do format conversion
/// - Open threads
/// - Do async operations
///
/// Returns a `WasapiRenderClientSmokeReport` describing the outcome.
pub fn probe_render_client() -> WasapiRenderClientSmokeReport {
    // Step 1: Check opt-in
    if !is_opt_in_enabled() {
        return WasapiRenderClientSmokeReport::skipped_env_missing();
    }

    // Step 2: Initialize COM
    let _com = match ComApartment::initialize() {
        Ok(com) => com,
        Err(e) => {
            return WasapiRenderClientSmokeReport::skipped_with_error("COM init failed", e);
        }
    };

    // Step 3: Create IMMDeviceEnumerator
    let enumerator = match render_client_steps::create_device_enumerator() {
        Ok(e) => e,
        Err(report) => return report,
    };

    // Step 4: Get default audio render endpoint
    let endpoint = match render_client_steps::get_default_render_endpoint(&enumerator) {
        Ok(ep) => ep,
        Err(report) => return report,
    };

    // Step 5: Activate IAudioClient from endpoint
    let audio_client = match render_client_steps::activate_audio_client(&endpoint) {
        Ok(client) => client,
        Err(report) => return report,
    };

    // Step 6: Get mix format and wrap in RAII guard
    let guard = match render_client_steps::get_mix_format(&audio_client) {
        Ok(g) => g,
        Err(report) => return report,
    };

    // Step 7: Read format fields into local struct
    // WAVEFORMATEX is packed, so we copy fields to avoid unaligned reference issues.
    let fields = unsafe { extract_format_fields(guard.ptr) };

    // Step 8: Call IAudioClient::Initialize in shared mode
    if let Err(e) = render_client_steps::initialize_shared_client(&audio_client, guard.ptr) {
        return WasapiRenderClientSmokeReport::mix_format_obtained_but_initialize_failed(fields, e);
    }

    // Step 9: Call IAudioClient::GetService to obtain IAudioRenderClient
    // We do NOT: GetBuffer, ReleaseBuffer, Start, Stop, Reset
    let _render_client = match render_client_steps::get_render_client_service(&audio_client) {
        Ok(client) => client,
        Err(e) => {
            return WasapiRenderClientSmokeReport::initialized_but_get_service_failed(fields, e);
        }
    };

    // Step 10: GetService succeeded.
    // _render_client dropped here (IAudioRenderClient).
    // audio_client dropped here (IAudioClient).
    // MixFormatGuard releases format pointer.
    // COM cleanup via ComApartment::drop.
    WasapiRenderClientSmokeReport::success(fields)
}
