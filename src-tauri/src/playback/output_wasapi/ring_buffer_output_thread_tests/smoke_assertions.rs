// smoke_assertions.rs
//
// Assertion helpers for the ring buffer output thread smoke test.

use crate::playback::output_wasapi::ring_buffer_output_thread::WasapiRingBufferOutputThreadSmokeReport;

/// Assert success WASAPI lifecycle through buffer size.
pub fn assert_success_wasapi_lifecycle(report: &WasapiRingBufferOutputThreadSmokeReport) {
    assert!(report.com_initialized, "COM initialized");
    assert!(report.endpoint_available, "endpoint available");
    assert!(report.client_activated, "client activated");
    assert!(report.mix_format_available, "mix format available");
    assert!(report.sample_rate_hz.is_some(), "sample rate present");
    assert!(report.channels.is_some(), "channels present");
    assert!(report.bits_per_sample.is_some(), "bits_per_sample present");
    assert!(report.block_align.is_some(), "block_align present");
    assert!(report.initialize_attempted, "initialize attempted");
    assert!(report.initialized_audio_client, "initialized");
    assert!(report.get_service_attempted, "get service attempted");
    assert!(report.render_client_obtained, "render client obtained");
    assert!(report.get_buffer_size_attempted, "buffer size attempted");
    assert!(report.buffer_size_frames.is_some(), "buffer size present");
    assert!(report.buffer_size_frames.unwrap() > 0, "buffer size > 0");
}

/// Assert ring buffer creation and silence read.
pub fn assert_success_ring_buffer(report: &WasapiRingBufferOutputThreadSmokeReport) {
    assert!(report.ring_buffer_created, "ring buffer created");
    assert!(
        report.ring_buffer_capacity_frames.is_some(),
        "ring buffer capacity present"
    );
    assert!(
        report.ring_buffer_capacity_frames.unwrap() > 0,
        "ring buffer capacity > 0"
    );
    assert!(report.total_silence_frames_filled > 0, "silence filled");
}

/// Assert WASAPI buffer, start/stop/reset.
pub fn assert_success_buffer_start_stop_reset(report: &WasapiRingBufferOutputThreadSmokeReport) {
    assert!(report.wasapi_buffer_obtained, "WASAPI buffer obtained");
    assert!(report.wasapi_buffer_released, "WASAPI buffer released");
    assert!(report.used_silent_flag, "used SILENT flag");
    assert!(report.start_attempted, "start attempted");
    assert!(report.started_audio_client, "started");
    assert!(report.get_current_padding_attempted, "padding attempted");
    assert!(
        report.current_padding_frames.is_some(),
        "padding frames present"
    );
    assert!(report.stop_attempted, "stop attempted");
    assert!(report.stopped_audio_client, "stopped");
    assert!(report.reset_attempted, "reset attempted");
    assert!(report.reset_succeeded, "reset succeeded");
}

/// Assert ring buffer is closed after success.
pub fn assert_success_ring_buffer_closed(report: &WasapiRingBufferOutputThreadSmokeReport) {
    assert!(report.ring_buffer_closed, "ring buffer closed");
}

/// Assert that all prohibited operations remain false.
pub fn assert_prohibited_always_false(report: &WasapiRingBufferOutputThreadSmokeReport) {
    assert!(!report.output_sink_connected, "no OutputSink");
    assert!(!report.capability_exposed, "no PlaybackCapabilities");
    assert!(!report.decoder_connected, "no decoder");
    assert!(!report.pipeline_connected, "no pipeline");
    assert!(!report.manager_connected, "no manager");
    assert!(!report.real_pcm_produced, "no real PCM");
    assert!(!report.non_silent_data_written, "no non-silent data");
    assert!(!report.audio_produced, "no audio produced");
    assert!(
        !report.playback_capability_enabled,
        "no playback capability"
    );
}

/// Assert thread lifecycle fields for success.
pub fn assert_thread_lifecycle_success(report: &WasapiRingBufferOutputThreadSmokeReport) {
    assert!(report.output_thread_spawn_attempted, "spawn attempted");
    assert!(report.output_thread_spawned, "spawned");
    assert!(report.thread_report_received, "report received");
    assert!(
        report.thread_recv_timeout_ms.is_some(),
        "recv timeout present"
    );
    assert!(!report.thread_recv_timed_out, "not timed out");
    assert!(report.thread_duration_ms.is_some(), "duration present");
    assert!(!report.thread_panic_caught, "no panic");
    assert!(!report.output_thread_join_failed, "join not failed");
}

/// Assert that no real audio playback occurred.
///
/// Validates that the smoke probe only produced silence,
/// never real PCM data or non-silent output.
pub fn assert_no_real_playback(report: &WasapiRingBufferOutputThreadSmokeReport) {
    assert!(!report.real_pcm_produced, "no real PCM produced");
    assert!(
        !report.non_silent_data_written,
        "no non-silent data written"
    );
    assert!(!report.audio_produced, "no audio produced");
    assert!(
        !report.playback_capability_enabled,
        "no playback capability"
    );
    assert!(!report.output_sink_connected, "no OutputSink connected");
    assert!(!report.capability_exposed, "no PlaybackCapabilities exposed");
}

/// Assert ring buffer silence behavior on success path.
///
/// Only call this on reports where the probe succeeded (not skipped).
/// Validates that the ring buffer was created, silence was filled,
/// and no real data was written.
pub fn assert_ring_buffer_silence_behavior(report: &WasapiRingBufferOutputThreadSmokeReport) {
    assert!(report.ring_buffer_created, "ring buffer created");
    assert!(report.used_silent_flag, "used SILENT flag");
    assert!(
        report.total_silence_frames_filled > 0,
        "silence frames filled"
    );
    assert!(report.ring_buffer_underrun_count > 0, "underrun count > 0");
    assert_eq!(report.total_frames_written, 0, "no frames written");
    assert!(
        !report.non_silent_data_written,
        "no non-silent data written"
    );
}

/// Assert that a report is either skipped or a silent success.
///
/// Combines skip validation and success validation into a single entry point.
/// - If skipped: validates skip fields and prohibited fields.
/// - If not skipped: validates attempted, silent success, and prohibited fields.
pub fn assert_skipped_or_silent_success(report: &WasapiRingBufferOutputThreadSmokeReport) {
    if report.skipped {
        assert!(
            report.skipped_reason.is_some(),
            "skip reason present when skipped"
        );
        assert_prohibited_always_false(report);
    } else {
        assert!(report.attempted, "attempted when not skipped");
        assert!(report.used_silent_flag, "used SILENT flag on success");
        assert_no_real_playback(report);
        assert_prohibited_always_false(report);
        assert_ring_buffer_silence_behavior(report);
    }
}
