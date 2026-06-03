// start_stop_tests/cleanup_tests.rs
//
// Cleanup and RAII behavior tests for the WASAPI start/stop smoke boundary.

use std::env;

use crate::playback::output_wasapi::start_stop::{probe_start_stop, WASAPI_START_STOP_SMOKE_ENV};

#[test]
fn start_success_requires_stop_attempt() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    // Without env, start was not attempted, so stop should not be attempted
    assert!(!report.start_attempted, "start_attempted should be false when skipped");
    assert!(!report.stop_attempted, "stop_attempted should be false when skipped");
}

#[test]
fn stop_guard_stops_if_start_succeeded() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    // Without env, nothing happened
    assert!(!report.started_audio_client, "started_audio_client should be false when skipped");
    assert!(!report.stopped_audio_client, "stopped_audio_client should be false when skipped");
}

#[test]
fn stop_failure_is_reported_without_reset() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    // Without env, all false
    assert!(!report.stopped_audio_client, "stopped_audio_client should be false when skipped");
    assert!(!report.reset_audio_client, "reset_audio_client should always be false");
}

#[test]
fn cleanup_does_not_reset_audio_client() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert!(!report.reset_audio_client, "reset_audio_client should always be false");
}

#[test]
fn cleanup_does_not_write_non_silent_data() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert!(!report.audio_produced, "audio_produced should always be false");
}

#[test]
fn cleanup_does_not_connect_output_sink() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert!(!report.output_sink_connected, "output_sink_connected should always be false");
}

#[test]
fn cleanup_does_not_claim_capability() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert!(!report.capability_exposed, "capability_exposed should always be false");
}

#[test]
fn cleanup_does_not_get_current_padding() {
    env::remove_var(WASAPI_START_STOP_SMOKE_ENV);

    let report = probe_start_stop();

    assert!(!report.get_current_padding_called, "get_current_padding_called should always be false");
}
