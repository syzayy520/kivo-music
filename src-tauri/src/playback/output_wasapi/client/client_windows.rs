// client_windows.rs
//
// Windows-only IAudioClient Activate smoke implementation.
//
// This module is the ONLY place in the codebase that activates an IAudioClient
// from a WASAPI endpoint. It performs a minimal smoke probe: get the default
// audio render endpoint, activate IAudioClient, then immediately drop it.
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::Initialize
//   - IAudioClient::Start / Stop / Reset
//   - IAudioClient::GetCurrentPadding
//   - IAudioClient::GetService
//   - IAudioRenderClient
//   - IAudioRenderClient::GetBuffer / ReleaseBuffer
//   - GetMixFormat
//   - IsFormatSupported
//   - Format negotiation
//   - Shared/exclusive mode initialization
//   - Audio playback
//   - Thread creation
//   - Async operations
//   - System volume changes

use std::env;

use windows::Win32::Media::Audio::{
    eConsole, eRender, IAudioClient, IMMDeviceEnumerator, MMDeviceEnumerator,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
};

use super::{WasapiClientActivateSmokeReport, WASAPI_CLIENT_ACTIVATE_SMOKE_ENV};

/// RAII guard for COM apartment initialization.
///
/// Calls `CoInitializeEx` on creation and `CoUninitialize` on drop.
/// This ensures COM is properly cleaned up even if the smoke probe
/// encounters an error.
struct ComApartment {
    initialized: bool,
}

impl ComApartment {
    /// Initialize COM apartment with MTA (multi-threaded apartment).
    ///
    /// Returns `Ok(ComApartment)` if initialization succeeded,
    /// or `Err(String)` with the error description.
    fn initialize() -> Result<Self, String> {
        // Windows COM FFI boundary
        // no audio client initialization
        // no render client
        // no playback
        unsafe {
            let hr = CoInitializeEx(None, COINIT_MULTITHREADED);
            if hr.is_ok() {
                Ok(Self { initialized: true })
            } else {
                Err(format!("CoInitializeEx failed: {hr:?}"))
            }
        }
    }
}

impl Drop for ComApartment {
    fn drop(&mut self) {
        if self.initialized {
            // Windows COM FFI boundary
            unsafe {
                CoUninitialize();
            }
        }
    }
}

/// Check if the opt-in environment variable is set to "1".
fn is_opt_in_enabled() -> bool {
    env::var(WASAPI_CLIENT_ACTIVATE_SMOKE_ENV).ok().as_deref() == Some("1")
}

/// Probe IAudioClient activation from the default audio render endpoint on Windows.
///
/// This function:
/// 1. Checks the opt-in environment variable
/// 2. Initializes COM (MTA)
/// 3. Creates an IMMDeviceEnumerator
/// 4. Calls GetDefaultAudioEndpoint(eRender, eConsole)
/// 5. Activates IAudioClient from the endpoint
/// 6. Drops IAudioClient immediately
/// 7. Cleans up COM
///
/// It does NOT:
/// - Initialize IAudioClient
/// - Start / Stop / Reset IAudioClient
/// - Get IAudioRenderClient
/// - GetBuffer / ReleaseBuffer
/// - Produce sound
///
/// Returns a `WasapiClientActivateSmokeReport` describing the outcome.
pub fn probe_client_activate() -> WasapiClientActivateSmokeReport {
    // Step 1: Check opt-in
    if !is_opt_in_enabled() {
        return WasapiClientActivateSmokeReport::skipped_env_missing();
    }

    // Step 2: Initialize COM
    let _com = match ComApartment::initialize() {
        Ok(com) => com,
        Err(e) => {
            return WasapiClientActivateSmokeReport::skipped_with_error("COM init failed", e);
        }
    };

    // Step 3: Create IMMDeviceEnumerator
    let enumerator: IMMDeviceEnumerator =
        // Windows COM FFI boundary
        match unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) } {
            Ok(enumerator) => enumerator,
            Err(e) => {
                return WasapiClientActivateSmokeReport::skipped_with_error(
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
                return WasapiClientActivateSmokeReport::skipped_with_error(
                    "default endpoint unavailable",
                    format!("GetDefaultAudioEndpoint failed: {e}"),
                );
            }
        };

    // Step 5: Activate IAudioClient from endpoint
    // This is the only place we call Activate in the entire codebase.
    // We do NOT Initialize, Start, Stop, Reset, GetService, or GetBuffer.
    let _audio_client: IAudioClient =
        // Windows COM FFI boundary
        // no audio client initialization
        // no render client
        // no playback
        match unsafe { endpoint.Activate(CLSCTX_ALL, None) } {
            Ok(client) => client,
            Err(e) => {
                return WasapiClientActivateSmokeReport::endpoint_available_but_activate_failed(
                    format!("Activate IAudioClient failed: {e}"),
                );
            }
        };

    // Step 6: IAudioClient obtained successfully. It will be dropped here.
    // No Initialize, no Start, no Stop, no Reset, no GetService, no playback.

    // Step 7: COM cleanup happens via ComApartment::drop

    WasapiClientActivateSmokeReport::success()
}
