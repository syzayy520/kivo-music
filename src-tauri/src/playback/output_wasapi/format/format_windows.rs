// format_windows.rs
//
// Windows-only IAudioClient GetMixFormat smoke implementation.
//
// This module is the ONLY place in the codebase that calls
// IAudioClient::GetMixFormat and CoTaskMemFree. It performs a minimal
// smoke probe: get the default audio render endpoint, activate IAudioClient,
// call GetMixFormat, read basic format fields, then release the format pointer.
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::Initialize
//   - IAudioClient::IsFormatSupported
//   - IAudioClient::Start / Stop / Reset
//   - IAudioClient::GetCurrentPadding
//   - IAudioClient::GetService
//   - IAudioRenderClient
//   - IAudioRenderClient::GetBuffer / ReleaseBuffer
//   - Shared/exclusive mode initialization
//   - Buffer duration design
//   - Stream flags design
//   - Format conversion
//   - Sample rate conversion
//   - Audio playback
//   - Thread creation
//   - Async operations
//   - System volume changes

use windows::Win32::Media::Audio::{
    eConsole, eRender, IAudioClient, IMMDeviceEnumerator, MMDeviceEnumerator,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};

use super::env::is_opt_in_enabled;
use super::format_fields::extract_format_fields;
use super::guards::{ComApartment, MixFormatGuard};
use super::report::WasapiMixFormatSmokeReport;

/// Probe IAudioClient::GetMixFormat from the default audio render endpoint on Windows.
///
/// This function:
/// 1. Checks the opt-in environment variable
/// 2. Initializes COM (MTA)
/// 3. Creates an IMMDeviceEnumerator
/// 4. Calls GetDefaultAudioEndpoint(eRender, eConsole)
/// 5. Activates IAudioClient from the endpoint
/// 6. Calls GetMixFormat to obtain the mix format pointer
/// 7. Reads basic format fields into local variables
/// 8. Releases the format pointer via CoTaskMemFree (RAII)
/// 9. Drops IAudioClient
/// 10. Cleans up COM (RAII)
///
/// It does NOT:
/// - Initialize IAudioClient
/// - IsFormatSupported
/// - GetService
/// - Get IAudioRenderClient
/// - GetBuffer / ReleaseBuffer
/// - Start / Stop / Reset
/// - Produce sound
/// - Do format conversion
/// - Open threads
/// - Do async operations
///
/// Returns a `WasapiMixFormatSmokeReport` describing the outcome.
pub fn probe_mix_format() -> WasapiMixFormatSmokeReport {
    // Step 1: Check opt-in
    if !is_opt_in_enabled() {
        return WasapiMixFormatSmokeReport::skipped_env_missing();
    }

    // Step 2: Initialize COM
    let _com = match ComApartment::initialize() {
        Ok(com) => com,
        Err(e) => {
            return WasapiMixFormatSmokeReport::skipped_with_error("COM init failed", e);
        }
    };

    // Step 3: Create IMMDeviceEnumerator
    let enumerator: IMMDeviceEnumerator =
        // Windows COM FFI boundary
        match unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) } {
            Ok(enumerator) => enumerator,
            Err(e) => {
                return WasapiMixFormatSmokeReport::skipped_with_error(
                    "device enumerator creation failed",
                    format!("CoCreateInstance failed: {e}"),
                );
            }
        };

    // Step 4: Get default audio render endpoint
    // eRender = output device, eConsole = console/multimedia role
    let endpoint =
        // Windows COM FFI boundary
        match unsafe { enumerator.GetDefaultAudioEndpoint(eRender, eConsole) } {
            Ok(endpoint) => endpoint,
            Err(e) => {
                return WasapiMixFormatSmokeReport::skipped_with_error(
                    "default endpoint unavailable",
                    format!("GetDefaultAudioEndpoint failed: {e}"),
                );
            }
        };

    // Step 5: Activate IAudioClient from endpoint
    // no audio client initialization
    // no render client
    // no playback
    let audio_client: IAudioClient =
        // Windows COM FFI boundary
        match unsafe { endpoint.Activate(CLSCTX_ALL, None) } {
            Ok(client) => client,
            Err(e) => {
                return WasapiMixFormatSmokeReport::endpoint_available_but_activate_failed(
                    format!("Activate IAudioClient failed: {e}"),
                );
            }
        };

    // Step 6: Call GetMixFormat
    // Returns a COM-allocated WAVEFORMATEX pointer that must be freed with CoTaskMemFree.
    // We do NOT Initialize, IsFormatSupported, GetService, or any playback operations.
    // mix format pointer lifetime
    // no audio client initialization
    // no render client
    // no playback
    let format_ptr =
        // Windows COM FFI boundary
        match unsafe { audio_client.GetMixFormat() } {
            Ok(ptr) => ptr,
            Err(e) => {
                return WasapiMixFormatSmokeReport::client_activated_but_mix_format_failed(
                    format!("GetMixFormat failed: {e}"),
                );
            }
        };

    // Step 7: Hold the pointer in RAII guard for safe release
    let guard = MixFormatGuard { ptr: format_ptr };

    // Step 8: Read basic format fields into local variables
    // WAVEFORMATEX is packed, so we copy fields to local variables before using them.
    // This avoids unaligned reference issues with packed struct fields.
    let (
        sample_rate_hz,
        channels,
        bits_per_sample,
        block_align,
        avg_bytes_per_sec,
        format_tag,
        cb_size,
    ) = unsafe { extract_format_fields(guard.ptr) };

    // Step 9: Report success. Format pointer will be released by MixFormatGuard::drop.
    // audio_client will be dropped here.
    // COM cleanup happens via ComApartment::drop.

    WasapiMixFormatSmokeReport::success(
        sample_rate_hz,
        channels,
        bits_per_sample,
        block_align,
        avg_bytes_per_sec,
        format_tag,
        cb_size,
    )
}
