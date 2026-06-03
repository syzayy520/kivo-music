// silent_loop_windows.rs
//
// Windows-only silent loop smoke implementation.
//
// This module is the ONLY place in the silent_loop boundary that performs
// the full WASAPI probe with a fixed-iteration silent write loop.
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::IsFormatSupported
//   - IAudioClient::Reset
//   - Non-silent audio data
//   - Audio thread / async runtime / callback
//   - Ring buffer / decoder / pipeline
//   - OutputSink / PlaybackCapabilities

use super::env::is_opt_in_enabled;
use super::format_fields::extract_format_fields;
use super::guards::{ComApartment, StartedClientGuard};
use super::report::WasapiSilentLoopSmokeReport;
use super::report_success_builders::LoopStats;
use super::silent_loop_steps;

const LOOP_ITERATIONS: u32 = 3;
const SMALL_FRAME_COUNT: u32 = 1;

/// Probe silent loop on Windows.
///
/// Returns a `WasapiSilentLoopSmokeReport` describing the outcome.
pub fn probe_silent_loop() -> WasapiSilentLoopSmokeReport {
    if !is_opt_in_enabled() {
        return WasapiSilentLoopSmokeReport::skipped_env_missing();
    }

    let _com = match ComApartment::initialize() {
        Ok(com) => com,
        Err(e) => {
            return WasapiSilentLoopSmokeReport::skipped_with_error("COM init failed", e);
        }
    };

    let enumerator = match silent_loop_steps::create_device_enumerator() {
        Ok(e) => e,
        Err(report) => return report,
    };

    let endpoint = match silent_loop_steps::get_default_render_endpoint(&enumerator) {
        Ok(ep) => ep,
        Err(report) => return report,
    };

    let audio_client = match silent_loop_steps::activate_audio_client(&endpoint) {
        Ok(client) => client,
        Err(report) => return report,
    };

    let guard = match silent_loop_steps::get_mix_format(&audio_client) {
        Ok(g) => g,
        Err(report) => return report,
    };

    let fields = unsafe { extract_format_fields(guard.ptr) };

    if let Err(e) = silent_loop_steps::initialize_shared_client(&audio_client, guard.ptr) {
        return WasapiSilentLoopSmokeReport::mix_format_obtained_but_initialize_failed(fields, e);
    }

    let render_client = match silent_loop_steps::get_render_client_service(&audio_client) {
        Ok(client) => client,
        Err(e) => {
            return WasapiSilentLoopSmokeReport::initialized_but_get_service_failed(fields, e);
        }
    };

    let buffer_size_frames = match silent_loop_steps::get_buffer_size(&audio_client) {
        Ok(size) => size,
        Err(e) => {
            return WasapiSilentLoopSmokeReport::render_client_obtained_but_get_buffer_size_failed(
                fields, e,
            );
        }
    };

    if buffer_size_frames == 0 {
        return WasapiSilentLoopSmokeReport::buffer_size_zero(fields);
    }

    // Prefill: GetBuffer(1) + ReleaseBuffer(1, SILENT)
    let mut buffer_guard =
        match silent_loop_steps::get_buffer(&render_client, 1, buffer_size_frames, fields, true) {
            Ok(g) => g,
            Err(report) => return report,
        };

    if let Err(report) = silent_loop_steps::release_buffer_silent(
        &mut buffer_guard,
        buffer_size_frames,
        fields,
        true,
    ) {
        return report;
    }

    // Start
    if let Err(e) = silent_loop_steps::start_audio_client(&audio_client) {
        return WasapiSilentLoopSmokeReport::start_failed(fields, buffer_size_frames, e);
    }

    let mut stop_guard = StartedClientGuard::new(audio_client.clone());

    // Silent loop
    let mut zero_available_count: u32 = 0;
    let mut current_padding_success_count: u32 = 0;
    let mut first_padding_frames: u32 = 0;
    let mut last_padding_frames: u32 = 0;
    let mut min_padding_observed: u32 = u32::MAX;
    let mut max_padding_observed: u32 = 0;
    let mut last_available_frames: u32 = 0;
    let mut last_writable_frames: u32 = 0;
    let mut loop_get_buffer_success_count: u32 = 0;
    let mut loop_release_buffer_success_count: u32 = 0;

    for i in 0..LOOP_ITERATIONS {
        let padding = match silent_loop_steps::get_current_padding(&audio_client) {
            Ok(p) => p,
            Err(e) => {
                return WasapiSilentLoopSmokeReport::loop_get_current_padding_failed(
                    fields,
                    buffer_size_frames,
                    i,
                    e,
                );
            }
        };

        current_padding_success_count += 1;
        if i == 0 {
            first_padding_frames = padding;
        }
        last_padding_frames = padding;
        min_padding_observed = min_padding_observed.min(padding);
        max_padding_observed = max_padding_observed.max(padding);

        let available = buffer_size_frames.saturating_sub(padding);
        last_available_frames = available;

        if available == 0 {
            zero_available_count += 1;
            continue;
        }

        let writable = available.min(SMALL_FRAME_COUNT);
        last_writable_frames = writable;

        let mut loop_buf = match silent_loop_steps::get_buffer(
            &render_client,
            writable,
            buffer_size_frames,
            fields,
            false,
        ) {
            Ok(g) => g,
            Err(report) => return report,
        };
        loop_get_buffer_success_count += 1;

        if let Err(report) = silent_loop_steps::release_buffer_silent(
            &mut loop_buf,
            buffer_size_frames,
            fields,
            false,
        ) {
            return report;
        }
        loop_release_buffer_success_count += 1;
    }

    // Stop
    if let Err(e) = stop_guard.stop() {
        return WasapiSilentLoopSmokeReport::stop_failed(fields, buffer_size_frames, e);
    }

    WasapiSilentLoopSmokeReport::success(
        fields,
        buffer_size_frames,
        LoopStats {
            loop_iterations_completed: LOOP_ITERATIONS,
            zero_available_count,
            current_padding_success_count,
            first_padding_frames,
            last_padding_frames,
            min_padding_observed,
            max_padding_observed,
            last_available_frames,
            last_writable_frames,
            loop_get_buffer_success_count,
            loop_release_buffer_success_count,
        },
    )
}
