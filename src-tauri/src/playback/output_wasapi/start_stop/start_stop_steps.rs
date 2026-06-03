// start_stop_steps.rs
//
// Step helper functions for IAudioClient::Start + Stop silent smoke.
//
// This file extracts individual WASAPI probe steps from probe_start_stop()
// to keep start_stop_windows.rs within the 220-line file limit.
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::IsFormatSupported
//   - IAudioClient::GetCurrentPadding
//   - IAudioClient::Reset
//   - Audio playback
//   - Non-silent audio data

use windows::Win32::Media::Audio::{
    eConsole, eRender, IAudioClient, IAudioRenderClient, IMMDeviceEnumerator, MMDeviceEnumerator,
    AUDCLNT_SHAREMODE_SHARED,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};

use super::guards::{BufferGuard, MixFormatGuard};
use super::report::WasapiStartStopSmokeReport;

/// Create an IMMDeviceEnumerator via CoCreateInstance.
#[allow(clippy::result_large_err)]
pub(super) fn create_device_enumerator() -> Result<IMMDeviceEnumerator, WasapiStartStopSmokeReport> {
    unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) }.map_err(|e| {
        WasapiStartStopSmokeReport::skipped_with_error(
            "device enumerator creation failed",
            format!("CoCreateInstance failed: {e}"),
        )
    })
}

/// Get the default audio render endpoint (eRender, eConsole).
#[allow(clippy::result_large_err)]
pub(super) fn get_default_render_endpoint(
    enumerator: &IMMDeviceEnumerator,
) -> Result<windows::Win32::Media::Audio::IMMDevice, WasapiStartStopSmokeReport> {
    unsafe { enumerator.GetDefaultAudioEndpoint(eRender, eConsole) }.map_err(|e| {
        WasapiStartStopSmokeReport::skipped_with_error(
            "default endpoint unavailable",
            format!("GetDefaultAudioEndpoint failed: {e}"),
        )
    })
}

/// Activate IAudioClient from an endpoint.
#[allow(clippy::result_large_err)]
pub(super) fn activate_audio_client(
    endpoint: &windows::Win32::Media::Audio::IMMDevice,
) -> Result<IAudioClient, WasapiStartStopSmokeReport> {
    unsafe { endpoint.Activate(CLSCTX_ALL, None) }.map_err(|e| {
        WasapiStartStopSmokeReport::endpoint_available_but_activate_failed(format!(
            "Activate IAudioClient failed: {e}"
        ))
    })
}

/// Get the mix format pointer and wrap in RAII guard.
#[allow(clippy::result_large_err)]
pub(super) fn get_mix_format(
    audio_client: &IAudioClient,
) -> Result<MixFormatGuard, WasapiStartStopSmokeReport> {
    let format_ptr = unsafe { audio_client.GetMixFormat() }.map_err(|e| {
        WasapiStartStopSmokeReport::client_activated_but_mix_format_failed(format!(
            "GetMixFormat failed: {e}"
        ))
    })?;
    Ok(MixFormatGuard { ptr: format_ptr })
}

/// Call IAudioClient::Initialize in shared mode.
pub(super) fn initialize_shared_client(
    audio_client: &IAudioClient,
    format_ptr: *mut windows::Win32::Media::Audio::WAVEFORMATEX,
) -> Result<(), String> {
    let hr = unsafe {
        audio_client.Initialize(
            AUDCLNT_SHAREMODE_SHARED,
            0,
            0,
            0,
            format_ptr,
            None,
        )
    };
    if hr.is_err() {
        Err(format!("IAudioClient::Initialize failed: {hr:?}"))
    } else {
        Ok(())
    }
}

/// Call IAudioClient::GetService to obtain IAudioRenderClient.
pub(super) fn get_render_client_service(
    audio_client: &IAudioClient,
) -> Result<IAudioRenderClient, String> {
    let hr = unsafe { audio_client.GetService::<IAudioRenderClient>() };
    hr.map_err(|e| format!("IAudioClient::GetService failed: {e}"))
}

/// Call IAudioClient::GetBufferSize to get the buffer size in frames.
pub(super) fn get_buffer_size(audio_client: &IAudioClient) -> Result<u32, String> {
    let hr = unsafe { audio_client.GetBufferSize() };
    hr.map_err(|e| format!("IAudioClient::GetBufferSize failed: {e}"))
}

/// Call IAudioRenderClient::GetBuffer to get a buffer pointer.
#[allow(clippy::result_large_err)]
pub(super) fn get_buffer(
    render_client: &IAudioRenderClient,
    frames: u32,
    buffer_size_frames: u32,
    fields: super::format_fields::FormatFields,
) -> Result<BufferGuard, WasapiStartStopSmokeReport> {
    let hr = unsafe { render_client.GetBuffer(frames) };
    if hr.is_ok() {
        Ok(BufferGuard::new(render_client.clone(), frames))
    } else {
        Err(WasapiStartStopSmokeReport::get_buffer_failed(
            fields,
            buffer_size_frames,
            frames,
            format!("IAudioRenderClient::GetBuffer failed: {hr:?}"),
        ))
    }
}

/// Call IAudioRenderClient::ReleaseBuffer with AUDCLNT_BUFFERFLAGS_SILENT.
#[allow(clippy::result_large_err)]
pub(super) fn release_buffer_silent(
    buffer_guard: &mut BufferGuard,
    buffer_size_frames: u32,
    requested_frames: u32,
    fields: super::format_fields::FormatFields,
) -> Result<(), WasapiStartStopSmokeReport> {
    buffer_guard.release_silent().map_err(|e| {
        WasapiStartStopSmokeReport::release_buffer_failed(
            fields,
            buffer_size_frames,
            requested_frames,
            e,
        )
    })
}

/// Call IAudioClient::Start.
pub(super) fn start_audio_client(audio_client: &IAudioClient) -> Result<(), String> {
    let hr = unsafe { audio_client.Start() };
    if hr.is_ok() {
        Ok(())
    } else {
        Err(format!("IAudioClient::Start failed: {hr:?}"))
    }
}
