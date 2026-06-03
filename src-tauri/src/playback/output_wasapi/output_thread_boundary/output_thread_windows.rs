// output_thread_boundary/output_thread_windows.rs
//
// Windows-only output thread boundary smoke implementation.
//
// This module is the ONLY place in the output_thread_boundary boundary that performs
// the full WASAPI probe with Stop + Reset after a single GetCurrentPadding
// in an independent output thread.
//
// **PROHIBITED** (not implemented here):
//   - IAudioClient::IsFormatSupported
//   - Non-silent audio data
//   - Silent loop
//   - Ring buffer / decoder / pipeline
//   - OutputSink / PlaybackCapabilities
//   - Async runtime

use std::time::Duration;

use super::env::is_opt_in_enabled;
use super::format_fields::extract_format_fields;
use super::guards::{ComApartment, StartedClientGuard};
use super::output_thread_steps;
use super::report::WasapiOutputThreadSmokeReport;
use super::thread_report::ThreadReport;
use super::thread_runner::{recv_timeout_and_join, spawn_output_thread_probe};

/// Timeout for waiting for the output thread to complete.
const OUTPUT_THREAD_TIMEOUT: Duration = Duration::from_secs(15);

/// Probe output thread boundary on Windows.
///
/// Returns a `WasapiOutputThreadSmokeReport` describing the outcome.
pub fn probe_output_thread_boundary() -> WasapiOutputThreadSmokeReport {
    if !is_opt_in_enabled() {
        return WasapiOutputThreadSmokeReport::skipped_env_missing();
    }

    let mut report = WasapiOutputThreadSmokeReport {
        attempted: true,
        thread_spawn_attempted: true,
        ..Default::default()
    };

    // Spawn the output thread
    let rx = spawn_output_thread_probe();
    report.thread_spawned = true;

    // Wait for the report with timeout
    let thread_report = recv_timeout_and_join(rx, OUTPUT_THREAD_TIMEOUT);

    match thread_report {
        ThreadReport::Success(success_report) => {
            // Merge the success report into our report
            report.merge_from(*success_report);
        }
        ThreadReport::Timeout => {
            report.thread_recv_timed_out = true;
            report.thread_recv_timeout_ms = Some(OUTPUT_THREAD_TIMEOUT.as_millis() as u64);
            report.error_message = Some("Output thread timed out".to_string());
        }
        ThreadReport::Panic(panic_msg) => {
            report.thread_panic_caught = true;
            report.thread_panic_message = Some(panic_msg);
            report.error_message = Some("Output thread panicked".to_string());
        }
        ThreadReport::JoinFailed(error) => {
            report.thread_join_failed = true;
            report.error_message = Some(format!("Output thread join failed: {error}"));
        }
    }

    report
}

/// Internal probe that runs inside the output thread.
///
/// This function runs the full WASAPI lifecycle probe:
/// COM init → endpoint → activate → GetMixFormat → Initialize
/// → GetService → GetBufferSize → GetBuffer(1) → ReleaseBuffer(1, SILENT)
/// → Start → GetCurrentPadding → Stop → Reset
///
/// Returns a `WasapiOutputThreadSmokeReport` describing the outcome.
pub fn probe_output_thread_boundary_internal() -> WasapiOutputThreadSmokeReport {
    let mut report = WasapiOutputThreadSmokeReport {
        query_mode: "output_thread_boundary",
        wait_duration_ms: Some(0),
        ..Default::default()
    };

    // COM initialization
    let _com = match ComApartment::initialize() {
        Ok(com) => {
            report.com_initialized = true;
            com
        }
        Err(e) => {
            return WasapiOutputThreadSmokeReport::com_init_failed(e);
        }
    };

    // Device enumerator
    let enumerator = match output_thread_steps::create_device_enumerator() {
        Ok(e) => e,
        Err(report) => return report,
    };

    // Default render endpoint
    let endpoint = match output_thread_steps::get_default_render_endpoint(&enumerator) {
        Ok(ep) => {
            report.endpoint_available = true;
            ep
        }
        Err(report) => return report,
    };

    // Activate audio client
    let audio_client = match output_thread_steps::activate_audio_client(&endpoint) {
        Ok(client) => {
            report.client_activated = true;
            client
        }
        Err(report) => return report,
    };

    // Get mix format
    let guard = match output_thread_steps::get_mix_format(&audio_client) {
        Ok(g) => {
            report.mix_format_available = true;
            g
        }
        Err(report) => return report,
    };

    // Extract format fields
    let fields = unsafe { extract_format_fields(guard.ptr) };
    report.sample_rate_hz = Some(fields.sample_rate_hz);
    report.channels = Some(fields.channels);
    report.bits_per_sample = Some(fields.bits_per_sample);
    report.block_align = Some(fields.block_align);
    report.avg_bytes_per_sec = Some(fields.avg_bytes_per_sec);
    report.format_tag = Some(fields.format_tag);
    report.cb_size = Some(fields.cb_size);

    // Initialize shared client
    report.initialize_attempted = true;
    if let Err(e) = output_thread_steps::initialize_shared_client(&audio_client, guard.ptr) {
        return WasapiOutputThreadSmokeReport::initialize_failed(fields, e);
    }
    report.initialized_audio_client = true;

    // Get render client service
    report.get_service_attempted = true;
    let render_client = match output_thread_steps::get_render_client_service(&audio_client) {
        Ok(client) => {
            report.render_client_obtained = true;
            client
        }
        Err(e) => {
            return WasapiOutputThreadSmokeReport::get_service_failed(fields, e);
        }
    };

    // Get buffer size
    report.get_buffer_size_attempted = true;
    let buffer_size_frames = match output_thread_steps::get_buffer_size(&audio_client) {
        Ok(size) => {
            report.buffer_size_frames = Some(size);
            size
        }
        Err(e) => {
            return WasapiOutputThreadSmokeReport::get_buffer_size_failed(fields, e);
        }
    };

    if buffer_size_frames == 0 {
        return WasapiOutputThreadSmokeReport::buffer_size_zero(fields);
    }

    // Prefill: GetBuffer(1) + ReleaseBuffer(1, SILENT)
    report.prefill_get_buffer_attempted = true;
    report.prefill_requested_frames = Some(1);
    let mut buffer_guard =
        match output_thread_steps::get_buffer(&render_client, 1, buffer_size_frames, fields) {
            Ok(g) => {
                report.prefill_buffer_obtained = true;
                g
            }
            Err(report) => return report,
        };

    report.prefill_release_buffer_attempted = true;
    report.prefill_used_silent_flag = true;
    if let Err(report) =
        output_thread_steps::release_buffer_silent(&mut buffer_guard, buffer_size_frames, fields)
    {
        return report;
    }
    report.prefill_buffer_released = true;
    report.prefill_released_frames = Some(1);

    // Start
    report.start_attempted = true;
    if let Err(e) = output_thread_steps::start_audio_client(&audio_client) {
        return WasapiOutputThreadSmokeReport::start_failed(fields, buffer_size_frames, e);
    }
    report.started_audio_client = true;

    let mut stop_guard = StartedClientGuard::new(audio_client.clone());

    // GetCurrentPadding once
    report.get_current_padding_attempted = true;
    let padding_frames = match output_thread_steps::get_current_padding(&audio_client) {
        Ok(p) => {
            report.current_padding_frames = Some(p);
            p
        }
        Err(e) => {
            return WasapiOutputThreadSmokeReport::get_current_padding_failed(
                fields,
                buffer_size_frames,
                e,
            );
        }
    };

    // Stop
    report.stop_attempted = true;
    if let Err(e) = stop_guard.stop() {
        return WasapiOutputThreadSmokeReport::stop_failed(fields, buffer_size_frames, padding_frames, e);
    }
    report.stopped_audio_client = true;

    // Reset (only if Stop succeeded)
    report.reset_attempted = true;
    match output_thread_steps::reset_audio_client(&audio_client) {
        Ok(()) => {
            report.reset_succeeded = true;
        }
        Err((hresult, error)) => {
            report.reset_hresult = Some(hresult);
            return WasapiOutputThreadSmokeReport::reset_failed(
                fields,
                buffer_size_frames,
                padding_frames,
                hresult,
                error,
            );
        }
    }

    WasapiOutputThreadSmokeReport::success(fields, buffer_size_frames, padding_frames, 0)
}
