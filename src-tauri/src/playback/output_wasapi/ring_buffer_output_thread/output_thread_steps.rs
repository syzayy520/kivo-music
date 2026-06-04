// ring_buffer_output_thread/output_thread_steps.rs
//
// Step helper functions for ring buffer output thread smoke probe.
//
// Each function wraps a single WASAPI call, returning simple String errors.
// The caller (output_thread_flow) is responsible for building the report.
//
// **PROHIBITED** (not implemented here):
//   - Non-silent audio data
//   - OutputSink / PlaybackCapabilities
//   - Decoder / pipeline / manager

use windows::Win32::Media::Audio::{
    eConsole, eRender, IAudioClient, IAudioRenderClient, IMMDeviceEnumerator, MMDeviceEnumerator,
    AUDCLNT_SHAREMODE_SHARED,
};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};

use super::guards::{BufferGuard, MixFormatGuard};

/// Create an IMMDeviceEnumerator via CoCreateInstance.
pub(super) fn create_device_enumerator() -> Result<IMMDeviceEnumerator, String> {
    unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) }
        .map_err(|e| format!("CoCreateInstance failed: {e}"))
}

/// Get the default audio render endpoint (eRender, eConsole).
pub(super) fn get_default_render_endpoint(
    enumerator: &IMMDeviceEnumerator,
) -> Result<windows::Win32::Media::Audio::IMMDevice, String> {
    unsafe { enumerator.GetDefaultAudioEndpoint(eRender, eConsole) }
        .map_err(|e| format!("GetDefaultAudioEndpoint failed: {e}"))
}

/// Activate IAudioClient from an endpoint.
pub(super) fn activate_audio_client(
    endpoint: &windows::Win32::Media::Audio::IMMDevice,
) -> Result<IAudioClient, String> {
    unsafe { endpoint.Activate(CLSCTX_ALL, None) }
        .map_err(|e| format!("Activate IAudioClient failed: {e}"))
}

/// Get the mix format pointer and wrap in RAII guard.
pub(super) fn get_mix_format(audio_client: &IAudioClient) -> Result<MixFormatGuard, String> {
    let format_ptr =
        unsafe { audio_client.GetMixFormat() }.map_err(|e| format!("GetMixFormat failed: {e}"))?;
    Ok(MixFormatGuard { ptr: format_ptr })
}

/// Call IAudioClient::Initialize in shared mode.
pub(super) fn initialize_shared_client(
    audio_client: &IAudioClient,
    format_ptr: *mut windows::Win32::Media::Audio::WAVEFORMATEX,
) -> Result<(), String> {
    let hr =
        unsafe { audio_client.Initialize(AUDCLNT_SHAREMODE_SHARED, 0, 0, 0, format_ptr, None) };
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

/// Call IAudioRenderClient::GetBuffer to get a buffer of the given size.
pub(super) fn get_buffer_sized(
    render_client: &IAudioRenderClient,
    frames: u32,
) -> Result<BufferGuard, String> {
    let hr = unsafe { render_client.GetBuffer(frames) };
    if hr.is_ok() {
        Ok(BufferGuard::new(render_client.clone(), frames))
    } else {
        Err(format!("IAudioRenderClient::GetBuffer failed: {hr:?}"))
    }
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

/// Call IAudioClient::GetCurrentPadding.
pub(super) fn get_current_padding(audio_client: &IAudioClient) -> Result<u32, String> {
    let hr = unsafe { audio_client.GetCurrentPadding() };
    hr.map_err(|e| format!("IAudioClient::GetCurrentPadding failed: {e}"))
}

/// Call IAudioClient::Reset.
pub(super) fn reset_audio_client(audio_client: &IAudioClient) -> Result<(), (i32, String)> {
    let hr = unsafe { audio_client.Reset() };
    if let Err(e) = hr {
        let code = e.code().0;
        Err((code, format!("IAudioClient::Reset failed: {e}")))
    } else {
        Ok(())
    }
}
