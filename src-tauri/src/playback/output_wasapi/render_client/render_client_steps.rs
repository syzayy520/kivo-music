// render_client_steps.rs
//
// Step helper functions for IAudioClient::GetService(IAudioRenderClient) smoke.
//
// This file extracts individual WASAPI probe steps from probe_render_client()
// to keep render_client_windows.rs within the 220-line file limit.
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::IsFormatSupported
//   - IAudioRenderClient::GetBuffer / ReleaseBuffer
//   - IAudioClient::Start / Stop / Reset
//   - Audio playback
//   - Buffer submission

use windows::Win32::Media::Audio::{
    eConsole, eRender, IAudioClient, IAudioRenderClient, IMMDeviceEnumerator, MMDeviceEnumerator,
    AUDCLNT_SHAREMODE_SHARED,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};

use super::guards::MixFormatGuard;
use super::report::WasapiRenderClientSmokeReport;

/// Create an IMMDeviceEnumerator via CoCreateInstance.
#[allow(clippy::result_large_err)]
pub(super) fn create_device_enumerator(
) -> Result<IMMDeviceEnumerator, WasapiRenderClientSmokeReport> {
    // Windows COM FFI boundary
    unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) }.map_err(|e| {
        WasapiRenderClientSmokeReport::skipped_with_error(
            "device enumerator creation failed",
            format!("CoCreateInstance failed: {e}"),
        )
    })
}

/// Get the default audio render endpoint (eRender, eConsole).
#[allow(clippy::result_large_err)]
pub(super) fn get_default_render_endpoint(
    enumerator: &IMMDeviceEnumerator,
) -> Result<windows::Win32::Media::Audio::IMMDevice, WasapiRenderClientSmokeReport> {
    // Windows COM FFI boundary
    unsafe { enumerator.GetDefaultAudioEndpoint(eRender, eConsole) }.map_err(|e| {
        WasapiRenderClientSmokeReport::skipped_with_error(
            "default endpoint unavailable",
            format!("GetDefaultAudioEndpoint failed: {e}"),
        )
    })
}

/// Activate IAudioClient from an endpoint.
#[allow(clippy::result_large_err)]
pub(super) fn activate_audio_client(
    endpoint: &windows::Win32::Media::Audio::IMMDevice,
) -> Result<IAudioClient, WasapiRenderClientSmokeReport> {
    // Windows COM FFI boundary
    unsafe { endpoint.Activate(CLSCTX_ALL, None) }.map_err(|e| {
        WasapiRenderClientSmokeReport::endpoint_available_but_activate_failed(format!(
            "Activate IAudioClient failed: {e}"
        ))
    })
}

/// Get the mix format pointer and wrap in RAII guard.
#[allow(clippy::result_large_err)]
pub(super) fn get_mix_format(
    audio_client: &IAudioClient,
) -> Result<MixFormatGuard, WasapiRenderClientSmokeReport> {
    // Windows COM FFI boundary
    let format_ptr = unsafe { audio_client.GetMixFormat() }.map_err(|e| {
        WasapiRenderClientSmokeReport::client_activated_but_mix_format_failed(format!(
            "GetMixFormat failed: {e}"
        ))
    })?;
    Ok(MixFormatGuard { ptr: format_ptr })
}

/// Call IAudioClient::Initialize in shared mode.
///
/// Parameters: shared, flags=0, duration=0, periodicity=0, pFormat, AudioSessionGuid=None.
pub(super) fn initialize_shared_client(
    audio_client: &IAudioClient,
    format_ptr: *mut windows::Win32::Media::Audio::WAVEFORMATEX,
) -> Result<(), String> {
    // Windows COM FFI boundary
    let hr = unsafe {
        audio_client.Initialize(
            AUDCLNT_SHAREMODE_SHARED,
            0,          // stream_flags
            0,          // hnsBufferDuration (0 = default)
            0,          // hnsPeriodicity (0 = default for shared mode)
            format_ptr, // pFormat
            None,       // AudioSessionGuid
        )
    };
    if hr.is_err() {
        Err(format!("IAudioClient::Initialize failed: {hr:?}"))
    } else {
        Ok(())
    }
}

/// Call IAudioClient::GetService to obtain IAudioRenderClient.
///
/// This is the ONLY place in the codebase that calls GetService.
/// It does NOT call GetBuffer, ReleaseBuffer, Start, Stop, or Reset.
pub(super) fn get_render_client_service(
    audio_client: &IAudioClient,
) -> Result<IAudioRenderClient, String> {
    // Windows COM FFI boundary
    // no buffer operations
    // no playback
    let hr = unsafe { audio_client.GetService::<IAudioRenderClient>() };
    hr.map_err(|e| format!("IAudioClient::GetService failed: {e}"))
}
