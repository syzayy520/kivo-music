// smoke_assertions.rs
//
// Assertion helpers for the ring buffer output thread smoke test.

use crate::playback::output_wasapi::ring_buffer_output_thread::WasapiRingBufferOutputThreadSmokeReport;

/// Assert that the report shows a skip when the opt-in env is not set.
pub fn assert_skipped_when_env_not_set(report: &WasapiRingBufferOutputThreadSmokeReport) {
    assert!(report.skipped, "should be skipped when env not set");
    assert!(!report.attempted, "should not attempt when env not set");
    assert!(
        !report.output_thread_spawn_attempted,
        "should not attempt thread spawn when env not set"
    );
    assert!(
        !report.output_thread_spawned,
        "should not spawn thread when env not set"
    );
}

/// Assert that the report shows a skip when probe was attempted but skipped.
pub fn assert_skipped_when_attempted_but_skipped(report: &WasapiRingBufferOutputThreadSmokeReport) {
    assert!(
        report.skipped_reason.is_some(),
        "should have skip reason when skipped"
    );
}

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
