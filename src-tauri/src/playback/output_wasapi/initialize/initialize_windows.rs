// initialize_windows.rs
//
// Windows-only IAudioClient::Initialize shared-mode smoke implementation.
//
// This module is the ONLY place in the codebase that calls
// IAudioClient::Initialize. It performs a minimal smoke probe:
// get the default audio render endpoint, activate IAudioClient,
// call GetMixFormat, call Initialize in shared mode, then
// immediately drop everything.
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::IsFormatSupported
//   - IAudioClient::GetService
//   - IAudioRenderClient
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

use windows::Win32::Media::Audio::{
    eConsole, eRender, IAudioClient, IMMDeviceEnumerator, MMDeviceEnumerator,
    AUDCLNT_SHAREMODE_SHARED,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};

use super::env::is_opt_in_enabled;
use super::format_fields::extract_format_fields;
use super::guards::{ComApartment, MixFormatGuard};
use super::report::WasapiClientInitializeSmokeReport;

/// Probe IAudioClient::Initialize in shared mode on Windows.
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
/// 9. Drops IAudioClient (RAII)
/// 10. Releases format pointer via MixFormatGuard (RAII)
/// 11. Cleans up COM via ComApartment (RAII)
///
/// It does NOT:
/// - Call IsFormatSupported
/// - Call GetService
/// - Get IAudioRenderClient
/// - Call GetBuffer / ReleaseBuffer
/// - Call Start / Stop / Reset
/// - Produce sound
/// - Do format conversion
/// - Open threads
/// - Do async operations
///
/// Returns a `WasapiClientInitializeSmokeReport` describing the outcome.
pub fn probe_initialize() -> WasapiClientInitializeSmokeReport {
    // Step 1: Check opt-in
    if !is_opt_in_enabled() {
        return WasapiClientInitializeSmokeReport::skipped_env_missing();
    }

    // Step 2: Initialize COM
    let _com = match ComApartment::initialize() {
        Ok(com) => com,
        Err(e) => {
            return WasapiClientInitializeSmokeReport::skipped_with_error("COM init failed", e);
        }
    };

    // Step 3: Create IMMDeviceEnumerator
    let enumerator: IMMDeviceEnumerator =
        // Windows COM FFI boundary
        match unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) } {
            Ok(enumerator) => enumerator,
            Err(e) => {
                return WasapiClientInitializeSmokeReport::skipped_with_error(
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
                return WasapiClientInitializeSmokeReport::skipped_with_error(
                    "default endpoint unavailable",
                    format!("GetDefaultAudioEndpoint failed: {e}"),
                );
            }
        };

    // Step 5: Activate IAudioClient from endpoint
    // This is the same as P0-022 client activate smoke
    // We do NOT Initialize yet, just get the client
    let audio_client: IAudioClient =
        // Windows COM FFI boundary
        // no audio client initialization yet
        // no render client
        // no playback
        match unsafe { endpoint.Activate(CLSCTX_ALL, None) } {
            Ok(client) => client,
            Err(e) => {
                return WasapiClientInitializeSmokeReport::endpoint_available_but_activate_failed(
                    format!("Activate IAudioClient failed: {e}"),
                );
            }
        };

    // Step 6: Call GetMixFormat
    // Returns a COM-allocated WAVEFORMATEX pointer that must be freed with CoTaskMemFree.
    // This is the same as P0-024 mix format smoke
    // We do NOT Initialize yet, just get the format
    let format_ptr =
        // Windows COM FFI boundary
        // no audio client initialization yet
        // no render client
        // no playback
        match unsafe { audio_client.GetMixFormat() } {
            Ok(ptr) => ptr,
            Err(e) => {
                return WasapiClientInitializeSmokeReport::client_activated_but_mix_format_failed(
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

    // Step 9: Call IAudioClient::Initialize in shared mode
    // This is the main purpose of this smoke test.
    // We use shared mode with minimal parameters:
    // - AUDCLNT_SHAREMODE_SHARED: shared mode, best compatibility
    // - stream_flags = 0: no special flags
    // - hnsBufferDuration = 0: use default buffer size (typically 10ms)
    // - hnsPeriodicity = 0: shared mode must be 0, engine decides
    // - pFormat = mix format pointer: device native format
    // - AudioSessionGuid = None: use default audio session
    //
    // We do NOT:
    // - Call IsFormatSupported (not needed in shared mode)
    // - Call GetService (not getting render client)
    // - Get IAudioRenderClient (not getting render client)
    // - Call GetBuffer / ReleaseBuffer (not getting buffer)
    // - Call Start / Stop / Reset (not starting playback)
    // - Produce sound (not starting playback)
    let hr = unsafe {
        audio_client.Initialize(
            AUDCLNT_SHAREMODE_SHARED,
            0,         // stream_flags
            0,         // hnsBufferDuration (0 = default)
            0,         // hnsPeriodicity (0 = default for shared mode)
            guard.ptr, // pFormat
            None,      // AudioSessionGuid
        )
    };

    // Step 10: Check if Initialize succeeded
    if hr.is_err() {
        return WasapiClientInitializeSmokeReport::mix_format_obtained_but_initialize_failed(
            sample_rate_hz,
            channels,
            bits_per_sample,
            block_align,
            avg_bytes_per_sec,
            format_tag,
            cb_size,
            format!("IAudioClient::Initialize failed: {hr:?}"),
        );
    }

    // Step 11: Initialize succeeded. Report success.
    // audio_client will be dropped here.
    // MixFormatGuard will release format pointer.
    // COM cleanup happens via ComApartment::drop.
    //
    // We do NOT:
    // - Call GetService (not getting render client)
    // - Get IAudioRenderClient (not getting render client)
    // - Call GetBuffer / ReleaseBuffer (not getting buffer)
    // - Call Start / Stop / Reset (not starting playback)
    // - Produce sound (not starting playback)

    WasapiClientInitializeSmokeReport::success(
        sample_rate_hz,
        channels,
        bits_per_sample,
        block_align,
        avg_bytes_per_sec,
        format_tag,
        cb_size,
    )
}
