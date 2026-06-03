// buffer_windows.rs
//
// Windows-only IAudioRenderClient::GetBuffer + ReleaseBuffer silent smoke implementation.
//
// This module is the ONLY place in the codebase that calls
// IAudioRenderClient::GetBuffer and ReleaseBuffer. It performs a
// minimal smoke probe: get the default audio render endpoint, activate
// IAudioClient, call GetMixFormat, call Initialize in shared mode,
// call GetService to obtain IAudioRenderClient, call GetBufferSize,
// call GetBuffer(1 frame), call ReleaseBuffer(1 frame) with
// AUDCLNT_BUFFERFLAGS_SILENT, then drop everything.
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::IsFormatSupported
//   - IAudioClient::GetCurrentPadding
//   - IAudioClient::Start / Stop / Reset
//   - Audio playback
//   - Non-silent audio data

use super::buffer_steps;
use super::env::is_opt_in_enabled;
use super::format_fields::extract_format_fields;
use super::guards::ComApartment;
use super::report::WasapiBufferSmokeReport;

/// Probe IAudioRenderClient::GetBuffer + ReleaseBuffer on Windows.
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
/// 10. Calls IAudioClient::GetBufferSize
/// 11. Calls IAudioRenderClient::GetBuffer with requested_frames = 1
/// 12. Calls IAudioRenderClient::ReleaseBuffer(1, AUDCLNT_BUFFERFLAGS_SILENT)
/// 13. Drops BufferGuard (RAII safety net)
/// 14. Drops IAudioRenderClient (RAII)
/// 15. Drops IAudioClient (RAII)
/// 16. Releases format pointer via MixFormatGuard (RAII)
/// 17. Cleans up COM via ComApartment (RAII)
///
/// It does NOT:
/// - Call IsFormatSupported
/// - Call GetCurrentPadding
/// - Call Start / Stop / Reset
/// - Produce sound
///
/// Returns a `WasapiBufferSmokeReport` describing the outcome.
pub fn probe_buffer() -> WasapiBufferSmokeReport {
    // Step 1: Check opt-in
    if !is_opt_in_enabled() {
        return WasapiBufferSmokeReport::skipped_env_missing();
    }

    // Step 2: Initialize COM
    let _com = match ComApartment::initialize() {
        Ok(com) => com,
        Err(e) => {
            return WasapiBufferSmokeReport::skipped_with_error("COM init failed", e);
        }
    };

    // Step 3: Create IMMDeviceEnumerator
    let enumerator = match buffer_steps::create_device_enumerator() {
        Ok(e) => e,
        Err(report) => return report,
    };

    // Step 4: Get default audio render endpoint
    let endpoint = match buffer_steps::get_default_render_endpoint(&enumerator) {
        Ok(ep) => ep,
        Err(report) => return report,
    };

    // Step 5: Activate IAudioClient from endpoint
    let audio_client = match buffer_steps::activate_audio_client(&endpoint) {
        Ok(client) => client,
        Err(report) => return report,
    };

    // Step 6: Get mix format and wrap in RAII guard
    let guard = match buffer_steps::get_mix_format(&audio_client) {
        Ok(g) => g,
        Err(report) => return report,
    };

    // Step 7: Read format fields into local struct
    // WAVEFORMATEX is packed, so we copy fields to avoid unaligned reference issues.
    let fields = unsafe { extract_format_fields(guard.ptr) };

    // Step 8: Call IAudioClient::Initialize in shared mode
    if let Err(e) = buffer_steps::initialize_shared_client(&audio_client, guard.ptr) {
        return WasapiBufferSmokeReport::mix_format_obtained_but_initialize_failed(fields, e);
    }

    // Step 9: Call IAudioClient::GetService to obtain IAudioRenderClient
    let render_client = match buffer_steps::get_render_client_service(&audio_client) {
        Ok(client) => client,
        Err(e) => {
            return WasapiBufferSmokeReport::initialized_but_get_service_failed(fields, e);
        }
    };

    // Step 10: Call IAudioClient::GetBufferSize
    let buffer_size_frames = match buffer_steps::get_buffer_size(&audio_client) {
        Ok(size) => size,
        Err(e) => {
            return WasapiBufferSmokeReport::render_client_obtained_but_get_buffer_size_failed(
                fields, e,
            );
        }
    };

    if buffer_size_frames == 0 {
        return WasapiBufferSmokeReport::buffer_size_zero(fields);
    }

    // Step 11: Call IAudioRenderClient::GetBuffer with requested_frames = 1
    let requested_frames = 1;
    let mut buffer_guard = match buffer_steps::get_buffer(
        &render_client,
        requested_frames,
        buffer_size_frames,
        fields,
    ) {
        Ok(g) => g,
        Err(report) => return report,
    };

    // Step 12: Call IAudioRenderClient::ReleaseBuffer with AUDCLNT_BUFFERFLAGS_SILENT
    if let Err(report) = buffer_steps::release_buffer_silent(
        &mut buffer_guard,
        buffer_size_frames,
        requested_frames,
        fields,
    ) {
        return report;
    }

    // Step 13: Success
    // buffer_guard dropped here (ReleaseBuffer safety net).
    // render_client dropped here (IAudioRenderClient).
    // audio_client dropped here (IAudioClient).
    // MixFormatGuard releases format pointer.
    // COM cleanup via ComApartment::drop.
    WasapiBufferSmokeReport::success(fields, buffer_size_frames, requested_frames)
}
