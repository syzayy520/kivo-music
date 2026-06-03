// reset_boundary_windows.rs
//
// Windows-only reset boundary smoke implementation.
//
// This module is the ONLY place in the reset_boundary boundary that performs
// the full WASAPI probe with Stop + Reset after a single GetCurrentPadding.
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::IsFormatSupported
//   - Non-silent audio data
//   - Silent loop
//   - Audio thread / async runtime / callback
//   - Ring buffer / decoder / pipeline
//   - OutputSink / PlaybackCapabilities

use super::env::is_opt_in_enabled;
use super::format_fields::extract_format_fields;
use super::guards::{ComApartment, StartedClientGuard};
use super::report::WasapiResetBoundarySmokeReport;
use super::reset_boundary_steps;

/// Probe reset boundary on Windows.
///
/// Returns a `WasapiResetBoundarySmokeReport` describing the outcome.
pub fn probe_reset_boundary() -> WasapiResetBoundarySmokeReport {
    if !is_opt_in_enabled() {
        return WasapiResetBoundarySmokeReport::skipped_env_missing();
    }

    let _com = match ComApartment::initialize() {
        Ok(com) => com,
        Err(e) => {
            return WasapiResetBoundarySmokeReport::skipped_with_error("COM init failed", e);
        }
    };

    let enumerator = match reset_boundary_steps::create_device_enumerator() {
        Ok(e) => e,
        Err(report) => return report,
    };

    let endpoint = match reset_boundary_steps::get_default_render_endpoint(&enumerator) {
        Ok(ep) => ep,
        Err(report) => return report,
    };

    let audio_client = match reset_boundary_steps::activate_audio_client(&endpoint) {
        Ok(client) => client,
        Err(report) => return report,
    };

    let guard = match reset_boundary_steps::get_mix_format(&audio_client) {
        Ok(g) => g,
        Err(report) => return report,
    };

    let fields = unsafe { extract_format_fields(guard.ptr) };

    if let Err(e) = reset_boundary_steps::initialize_shared_client(&audio_client, guard.ptr) {
        return WasapiResetBoundarySmokeReport::mix_format_obtained_but_initialize_failed(
            fields, e,
        );
    }

    let render_client = match reset_boundary_steps::get_render_client_service(&audio_client) {
        Ok(client) => client,
        Err(e) => {
            return WasapiResetBoundarySmokeReport::initialized_but_get_service_failed(fields, e);
        }
    };

    let buffer_size_frames = match reset_boundary_steps::get_buffer_size(&audio_client) {
        Ok(size) => size,
        Err(e) => {
            return WasapiResetBoundarySmokeReport::render_client_obtained_but_get_buffer_size_failed(
                fields, e,
            );
        }
    };

    if buffer_size_frames == 0 {
        return WasapiResetBoundarySmokeReport::buffer_size_zero(fields);
    }

    // Prefill: GetBuffer(1) + ReleaseBuffer(1, SILENT)
    let mut buffer_guard =
        match reset_boundary_steps::get_buffer(&render_client, 1, buffer_size_frames, fields) {
            Ok(g) => g,
            Err(report) => return report,
        };

    if let Err(report) =
        reset_boundary_steps::release_buffer_silent(&mut buffer_guard, buffer_size_frames, fields)
    {
        return report;
    }

    // Start
    if let Err(e) = reset_boundary_steps::start_audio_client(&audio_client) {
        return WasapiResetBoundarySmokeReport::start_failed(fields, buffer_size_frames, e);
    }

    let mut stop_guard = StartedClientGuard::new(audio_client.clone());

    // GetCurrentPadding once
    let padding_frames = match reset_boundary_steps::get_current_padding(&audio_client) {
        Ok(p) => p,
        Err(e) => {
            return WasapiResetBoundarySmokeReport::get_current_padding_failed(
                fields,
                buffer_size_frames,
                e,
            );
        }
    };

    // Stop
    if let Err(e) = stop_guard.stop() {
        return WasapiResetBoundarySmokeReport::stop_failed(fields, buffer_size_frames, e);
    }

    // Reset (only if Stop succeeded)
    match reset_boundary_steps::reset_audio_client(&audio_client) {
        Ok(()) => {
            // Success
        }
        Err((hresult, error)) => {
            return WasapiResetBoundarySmokeReport::reset_failed(
                fields,
                buffer_size_frames,
                padding_frames,
                hresult,
                error,
            );
        }
    }

    WasapiResetBoundarySmokeReport::success(fields, buffer_size_frames, padding_frames, 0)
}
