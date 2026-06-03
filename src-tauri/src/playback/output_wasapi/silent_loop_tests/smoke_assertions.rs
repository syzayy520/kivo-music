// smoke_assertions.rs
//
// Assertion helpers for the silent loop smoke test.
// Each helper asserts the same semantics as the original inline assertions;
// no behavior is added or changed.

use crate::playback::output_wasapi::silent_loop::report::WasapiSilentLoopSmokeReport;

/// Assert that the report shows a skip when the opt-in env is not set.
pub fn assert_skipped_when_env_not_set(report: &WasapiSilentLoopSmokeReport) {
    assert!(report.skipped, "should be skipped when env not set");
    assert!(!report.attempted, "should not attempt when env not set");
    assert!(
        !report.start_attempted,
        "should not attempt start when env not set"
    );
    assert!(
        !report.stop_attempted,
        "should not attempt stop when env not set"
    );
    assert!(
        !report.get_current_padding_attempted,
        "should not attempt padding when env not set"
    );
}

/// Assert that the report shows a skip when probe was attempted but skipped.
pub fn assert_skipped_when_attempted_but_skipped(report: &WasapiSilentLoopSmokeReport) {
    assert!(
        report.skipped_reason.is_some(),
        "should have skip reason when skipped"
    );
    assert!(!report.stopped_audio_client, "should not stop when skipped");
    assert!(
        !report.get_current_padding_attempted,
        "should not attempt padding when skipped"
    );
}

/// Assert success prerequisites: endpoint through buffer_size.
pub fn assert_success_prereq_steps(report: &WasapiSilentLoopSmokeReport) {
    assert!(report.endpoint_available, "endpoint available");
    assert!(report.client_activated, "client activated");
    assert!(report.mix_format_available, "mix format available");
    assert!(report.initialize_attempted, "initialize attempted");
    assert!(report.initialized_audio_client, "initialized");
    assert!(report.get_service_attempted, "get service attempted");
    assert!(report.render_client_obtained, "render client obtained");
    assert!(report.get_buffer_size_attempted, "buffer size attempted");
    assert!(report.buffer_size_frames.is_some(), "buffer size present");
    assert!(report.buffer_size_frames.unwrap() > 0, "buffer size > 0");
}

/// Assert buffer prefill and format fields.
pub fn assert_success_buffer_and_prefill(report: &WasapiSilentLoopSmokeReport) {
    assert!(report.prefill_get_buffer_attempted, "prefill get buffer");
    assert!(report.prefill_buffer_obtained, "prefill buffer obtained");
    assert!(report.prefill_release_buffer_attempted, "prefill release");
    assert!(report.prefill_buffer_released, "prefill released");
    assert_eq!(
        report.prefill_requested_frames,
        Some(1),
        "prefill request 1"
    );
    assert_eq!(report.prefill_released_frames, Some(1), "prefill release 1");
    assert!(report.prefill_used_silent_flag, "prefill SILENT flag");
    assert!(report.sample_rate_hz.is_some(), "sample rate present");
    assert!(report.channels.is_some(), "channels present");
    assert!(report.bits_per_sample.is_some(), "bits_per_sample present");
}

/// Assert start/stop and loop statistics.
pub fn assert_success_start_stop_and_loop(report: &WasapiSilentLoopSmokeReport) {
    assert!(report.start_attempted, "start attempted");
    assert!(report.started_audio_client, "started");
    assert!(report.stop_attempted, "stop attempted");
    assert!(report.stopped_audio_client, "stopped");
    assert_eq!(report.loop_iterations_configured, Some(3), "configured 3");
    assert_eq!(report.loop_iterations_completed, Some(3), "completed 3");
    assert_eq!(report.small_frame_count, Some(1), "small_frame_count=1");
    assert!(
        report.zero_available_count.is_some(),
        "zero_available present"
    );
    assert!(report.get_current_padding_attempted, "padding attempted");
    assert!(
        report.current_padding_success_count.is_some(),
        "padding success present"
    );
    assert_eq!(
        report.current_padding_success_count.unwrap(),
        3,
        "padding 3x"
    );
    assert!(
        report.first_padding_frames.is_some(),
        "first padding present"
    );
    assert!(report.last_padding_frames.is_some(), "last padding present");
    assert!(report.min_padding_observed.is_some(), "min padding present");
    assert!(report.max_padding_observed.is_some(), "max padding present");
    assert!(
        report.loop_get_buffer_attempted,
        "loop get buffer attempted"
    );
    assert!(
        report.loop_get_buffer_success_count.is_some(),
        "loop get buffer count present"
    );
    assert!(
        report.loop_release_buffer_attempted,
        "loop release attempted"
    );
    assert!(
        report.loop_release_buffer_success_count.is_some(),
        "loop release count present"
    );
    assert!(report.loop_all_releases_silent, "all releases silent");
    assert_eq!(report.query_mode, "silent_loop");
    assert_eq!(report.wait_duration_ms, Some(0));
}

/// Assert that all prohibited operations remain false.
pub fn assert_prohibited_always_false(report: &WasapiSilentLoopSmokeReport) {
    assert!(!report.is_format_supported_called, "no IsFormatSupported");
    assert!(!report.reset_audio_client, "no Reset");
    assert!(!report.audio_produced, "no audio produced");
    assert!(!report.output_sink_connected, "no OutputSink");
    assert!(!report.capability_exposed, "no PlaybackCapabilities");
    assert!(!report.thread_created, "no thread");
    assert!(!report.async_runtime_created, "no async runtime");
    assert!(!report.callback_registered, "no callback");
    assert!(!report.ring_buffer_created, "no ring buffer");
    assert!(!report.decoder_connected, "no decoder");
    assert!(!report.pipeline_connected, "no pipeline");
}
