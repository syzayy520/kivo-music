// device_windows.rs
//
// Windows-only endpoint smoke implementation.
//
// This module is the ONLY place in the codebase that calls real Windows COM
// audio APIs. It performs a minimal smoke probe: get the default audio
// render endpoint, then immediately drop it.
//
// **PROHIBITED** (not implemented here):
//   - IMMDevice::Activate
//   - IAudioClient::Initialize
//   - IAudioClient::Start / Stop / Reset
//   - IAudioRenderClient::GetBuffer / ReleaseBuffer
//   - Format negotiation
//   - Shared/exclusive mode initialization
//   - Audio playback
//   - Thread creation
//   - Async operations
//   - System volume changes

use std::env;

use windows::Win32::Media::Audio::{eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
};

use super::{WasapiEndpointSmokeReport, WASAPI_ENDPOINT_SMOKE_ENV};

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
            unsafe {
                CoUninitialize();
            }
        }
    }
}

/// Check if the opt-in environment variable is set to "1".
fn is_opt_in_enabled() -> bool {
    env::var(WASAPI_ENDPOINT_SMOKE_ENV).ok().as_deref() == Some("1")
}

/// Probe the default audio render endpoint on Windows.
///
/// This function:
/// 1. Checks the opt-in environment variable
/// 2. Initializes COM (MTA)
/// 3. Creates an IMMDeviceEnumerator
/// 4. Calls GetDefaultAudioEndpoint(eRender, eConsole)
/// 5. Drops the endpoint immediately
/// 6. Cleans up COM
///
/// It does NOT:
/// - Activate the endpoint for audio client access
/// - Initialize any audio client
/// - Open a render client
/// - Produce sound
///
/// Returns a `WasapiEndpointSmokeReport` describing the outcome.
pub fn probe_default_endpoint() -> WasapiEndpointSmokeReport {
    // Step 1: Check opt-in
    if !is_opt_in_enabled() {
        return WasapiEndpointSmokeReport::skipped_env_missing();
    }

    // Step 2: Initialize COM
    let _com = match ComApartment::initialize() {
        Ok(com) => com,
        Err(e) => {
            return WasapiEndpointSmokeReport::skipped_with_error("COM init failed", e);
        }
    };

    // Step 3: Create IMMDeviceEnumerator
    let enumerator: IMMDeviceEnumerator =
        match unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) } {
            Ok(enumerator) => enumerator,
            Err(e) => {
                return WasapiEndpointSmokeReport::skipped_with_error(
                    "device enumerator creation failed",
                    format!("CoCreateInstance failed: {e}"),
                );
            }
        };

    // Step 4: Get default audio render endpoint
    // eRender = output device, eConsole = console/multimedia role
    let _endpoint = match unsafe { enumerator.GetDefaultAudioEndpoint(eRender, eConsole) } {
        Ok(endpoint) => endpoint,
        Err(e) => {
            return WasapiEndpointSmokeReport::skipped_with_error(
                "default endpoint unavailable",
                format!("GetDefaultAudioEndpoint failed: {e}"),
            );
        }
    };

    // Step 5: Endpoint obtained successfully. It will be dropped here.
    // No Activate, no IAudioClient, no playback.

    // Step 6: COM cleanup happens via ComApartment::drop

    WasapiEndpointSmokeReport::success()
}
