// start_stop_windows.rs
//
// Windows-only IAudioClient::Start + Stop silent smoke implementation.
//
// This module is the ONLY place in the codebase that calls
// IAudioClient::Start and Stop. It performs a minimal smoke probe:
// get the default audio render endpoint, activate IAudioClient,
// call GetMixFormat, call Initialize in shared mode, call GetService
// to obtain IAudioRenderClient, call GetBufferSize, call GetBuffer(1),
// call ReleaseBuffer(1, AUDCLNT_BUFFERFLAGS_SILENT), call Start,
// call Stop, then drop everything.
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::IsFormatSupported
//   - IAudioClient::GetCurrentPadding
//   - IAudioClient::Reset
//   - Non-silent audio data
//   - Decoded user audio data
//   - Audio thread
//   - Async runtime
//   - Queue integration
//   - OutputSink real implementation
//   - PlaybackCapabilities change
//   - Settings exposure
//   - UI exposure
//   - Manager integration
//   - Pipeline integration
//   - Decoder changes
//   - Format conversion
//   - Resampler
//   - Volume
//   - ReplayGain
//   - Clipping protection
//   - Device hotplug
//   - Exclusive mode

use super::env::is_opt_in_enabled;
use super::format_fields::extract_format_fields;
use super::guards::{ComApartment, StartedClientGuard};
use super::report::WasapiStartStopSmokeReport;
use super::start_stop_steps;

/// Probe IAudioClient::Start + Stop on Windows.
///
/// Returns a `WasapiStartStopSmokeReport` describing the outcome.
pub fn probe_start_stop() -> WasapiStartStopSmokeReport {
    // Step 1: Check opt-in
    if !is_opt_in_enabled() {
        return WasapiStartStopSmokeReport::skipped_env_missing();
    }

    // Step 2: Initialize COM
    let _com = match ComApartment::initialize() {
        Ok(com) => com,
        Err(e) => {
            return WasapiStartStopSmokeReport::skipped_with_error("COM init failed", e);
        }
    };

    // Step 3: Create IMMDeviceEnumerator
    let enumerator = match start_stop_steps::create_device_enumerator() {
        Ok(e) => e,
        Err(report) => return report,
    };

    // Step 4: Get default audio render endpoint
    let endpoint = match start_stop_steps::get_default_render_endpoint(&enumerator) {
        Ok(ep) => ep,
        Err(report) => return report,
    };

    // Step 5: Activate IAudioClient from endpoint
    let audio_client = match start_stop_steps::activate_audio_client(&endpoint) {
        Ok(client) => client,
        Err(report) => return report,
    };

    // Step 6: Get mix format and wrap in RAII guard
    let guard = match start_stop_steps::get_mix_format(&audio_client) {
        Ok(g) => g,
        Err(report) => return report,
    };

    // Step 7: Read format fields into local struct
    let fields = unsafe { extract_format_fields(guard.ptr) };

    // Step 8: Call IAudioClient::Initialize in shared mode
    if let Err(e) = start_stop_steps::initialize_shared_client(&audio_client, guard.ptr) {
        return WasapiStartStopSmokeReport::mix_format_obtained_but_initialize_failed(fields, e);
    }

    // Step 9: Call IAudioClient::GetService to obtain IAudioRenderClient
    let render_client = match start_stop_steps::get_render_client_service(&audio_client) {
        Ok(client) => client,
        Err(e) => {
            return WasapiStartStopSmokeReport::initialized_but_get_service_failed(fields, e);
        }
    };

    // Step 10: Call IAudioClient::GetBufferSize
    let buffer_size_frames = match start_stop_steps::get_buffer_size(&audio_client) {
        Ok(size) => size,
        Err(e) => {
            return WasapiStartStopSmokeReport::render_client_obtained_but_get_buffer_size_failed(
                fields, e,
            );
        }
    };

    if buffer_size_frames == 0 {
        return WasapiStartStopSmokeReport::buffer_size_zero(fields);
    }

    // Step 11: Call IAudioRenderClient::GetBuffer with requested_frames = 1
    let requested_frames = 1;
    let mut buffer_guard = match start_stop_steps::get_buffer(
        &render_client,
        requested_frames,
        buffer_size_frames,
        fields,
    ) {
        Ok(g) => g,
        Err(report) => return report,
    };

    // Step 12: Call IAudioRenderClient::ReleaseBuffer with AUDCLNT_BUFFERFLAGS_SILENT
    if let Err(report) = start_stop_steps::release_buffer_silent(
        &mut buffer_guard,
        buffer_size_frames,
        requested_frames,
        fields,
    ) {
        return report;
    }

    // Step 13: Call IAudioClient::Start
    if let Err(e) = start_stop_steps::start_audio_client(&audio_client) {
        return WasapiStartStopSmokeReport::start_failed(fields, buffer_size_frames, e);
    }

    // Step 14: Start succeeded, create StopGuard
    let mut stop_guard = StartedClientGuard::new(audio_client.clone());

    // Step 15: Explicitly call Stop
    if let Err(e) = stop_guard.stop() {
        return WasapiStartStopSmokeReport::stop_failed(fields, buffer_size_frames, e);
    }

    // Step 16: Success
    // stop_guard dropped here (Stop safety net, already stopped).
    // buffer_guard dropped here (ReleaseBuffer safety net, already released).
    // render_client dropped here (IAudioRenderClient).
    // audio_client dropped here (IAudioClient).
    // MixFormatGuard releases format pointer.
    // COM cleanup via ComApartment::drop.
    WasapiStartStopSmokeReport::success(fields, buffer_size_frames)
}
