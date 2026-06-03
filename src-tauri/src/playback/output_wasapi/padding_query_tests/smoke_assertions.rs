// smoke_assertions.rs
//
// Assertion helpers for the padding query smoke test.
// Each helper asserts the same semantics as the original inline assertions;
// no behavior is added or changed.

#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::playback::output_wasapi::padding_query::report::WasapiPaddingQuerySmokeReport;

/// Assert that the report shows a skip when the opt-in env is not set.
pub fn assert_skipped_when_env_not_set(report: &WasapiPaddingQuerySmokeReport) {
    assert!(report.skipped, "should be skipped when env not set");
    assert!(!report.attempted, "should not attempt when env not set");
    assert!(!report.start_attempted, "should not attempt start when env not set");
    assert!(!report.stop_attempted, "should not attempt stop when env not set");
    assert!(!report.get_current_padding_attempted, "should not attempt padding when env not set");
}

/// Assert that the report shows a skip when probe was attempted but skipped.
pub fn assert_skipped_when_attempted_but_skipped(report: &WasapiPaddingQuerySmokeReport) {
    assert!(report.skipped_reason.is_some(), "should have skip reason when skipped");
    assert!(!report.stopped_audio_client, "should not stop when skipped");
    assert!(!report.get_current_padding_attempted, "should not attempt padding when skipped");
}

/// Assert success prerequisites: endpoint through buffer_size.
pub fn assert_success_prereq_steps(report: &WasapiPaddingQuerySmokeReport) {
    assert!(report.endpoint_available, "endpoint should be available");
    assert!(report.client_activated, "client should be activated");
    assert!(report.mix_format_available, "mix format should be available");
    assert!(report.initialize_attempted, "initialize should have been attempted");
    assert!(report.initialized_audio_client, "initialize should have succeeded");
    assert!(report.get_service_attempted, "get_service should have been attempted");
    assert!(report.render_client_obtained, "render_client should have been obtained");
    assert!(report.get_buffer_size_attempted, "get_buffer_size should have been attempted");
    assert!(report.buffer_size_frames.is_some(), "buffer_size_frames should be Some");
    assert!(report.buffer_size_frames.unwrap() >= 1, "buffer_size_frames should be >= 1");
}

/// Assert success buffer steps, prefill frames, and silent flag.
pub fn assert_success_buffer_and_prefill(report: &WasapiPaddingQuerySmokeReport) {
    assert!(report.get_buffer_attempted, "get_buffer should have been attempted");
    assert!(report.buffer_obtained, "buffer should have been obtained");
    assert!(report.release_buffer_attempted, "release_buffer should have been attempted");
    assert!(report.buffer_released, "buffer should have been released");
    assert!(report.prefill_used_silent_flag, "should have used silent flag");
    assert_eq!(report.prefill_requested_frames, Some(1), "prefill_requested_frames should be Some(1)");
    assert_eq!(report.prefill_released_frames, Some(1), "prefill_released_frames should be Some(1)");
}

/// Assert success Start/Stop and padding query result.
pub fn assert_success_start_stop_and_padding(report: &WasapiPaddingQuerySmokeReport) {
    assert!(report.start_attempted, "start should have been attempted");
    assert!(report.started_audio_client, "start should have succeeded");
    assert!(report.stop_attempted, "stop should have been attempted");
    assert!(report.stopped_audio_client, "stop should have succeeded");
    assert_eq!(report.wait_duration_ms, Some(0), "wait_duration_ms should be Some(0)");
    assert!(report.get_current_padding_attempted, "padding query should have been attempted");
    assert!(report.padding_frames.is_some(), "padding_frames should be Some");
    assert_eq!(report.query_mode, "started", "query_mode should be 'started'");
}

/// Assert all prohibited operations are false.
pub fn assert_prohibited_always_false(report: &WasapiPaddingQuerySmokeReport) {
    assert!(!report.is_format_supported_called, "is_format_supported_called should be false");
    assert!(!report.reset_audio_client, "reset_audio_client should be false");
    assert!(!report.audio_produced, "audio_produced should be false");
    assert!(!report.output_sink_connected, "output_sink_connected should be false");
    assert!(!report.capability_exposed, "capability_exposed should be false");
    assert!(!report.thread_created, "thread_created should be false");
    assert!(!report.async_runtime_created, "async_runtime_created should be false");
    assert!(!report.callback_registered, "callback_registered should be false");
    assert!(report.skipped_reason.is_none(), "should not have skip reason on success");
    assert!(report.error_message.is_none(), "should not have error message on success");
}
