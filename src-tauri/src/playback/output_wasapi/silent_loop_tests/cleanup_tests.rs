// silent_loop_tests/cleanup_tests.rs
//
// Tests verifying RAII guard cleanup behavior and prohibited operations.

use crate::playback::output_wasapi::silent_loop::report::WasapiSilentLoopSmokeReport;

#[test]
fn report_has_no_reset_field_true() {
    // After any report path, reset_audio_client must be false
    let r = WasapiSilentLoopSmokeReport::skipped_non_windows();
    assert!(!r.reset_audio_client, "no Reset in skipped report");

    let r2 = WasapiSilentLoopSmokeReport::skipped_env_missing();
    assert!(!r2.reset_audio_client, "no Reset in env_missing report");

    let r3 = WasapiSilentLoopSmokeReport::skipped_with_error("test", "err".into());
    assert!(!r3.reset_audio_client, "no Reset in error report");
}

#[test]
fn report_has_no_real_pcm() {
    // audio_produced must always be false
    let r = WasapiSilentLoopSmokeReport::base_report_for_windows();
    assert!(!r.audio_produced);

    let r2 = WasapiSilentLoopSmokeReport::base_report_for_non_windows();
    assert!(!r2.audio_produced);
}

#[test]
fn report_has_no_output_sink() {
    let r = WasapiSilentLoopSmokeReport::base_report_for_windows();
    assert!(!r.output_sink_connected);
}

#[test]
fn report_has_no_capability() {
    let r = WasapiSilentLoopSmokeReport::base_report_for_windows();
    assert!(!r.capability_exposed);
}

#[test]
fn report_has_no_thread_async_callback() {
    let r = WasapiSilentLoopSmokeReport::base_report_for_windows();
    assert!(!r.thread_created);
    assert!(!r.async_runtime_created);
    assert!(!r.callback_registered);
}

#[test]
fn report_has_no_ring_buffer_decoder_pipeline() {
    let r = WasapiSilentLoopSmokeReport::base_report_for_windows();
    assert!(!r.ring_buffer_created);
    assert!(!r.decoder_connected);
    assert!(!r.pipeline_connected);
}

#[test]
fn skipped_reports_preserve_prohibited_defaults() {
    // All skipped builders should inherit base defaults for prohibited fields
    let reports = vec![
        WasapiSilentLoopSmokeReport::skipped_non_windows(),
        WasapiSilentLoopSmokeReport::skipped_env_missing(),
        WasapiSilentLoopSmokeReport::skipped_with_error("test", "err".into()),
        WasapiSilentLoopSmokeReport::endpoint_available_but_activate_failed("err".into()),
        WasapiSilentLoopSmokeReport::client_activated_but_mix_format_failed("err".into()),
    ];

    for r in &reports {
        assert!(!r.reset_audio_client, "reset_audio_client should be false");
        assert!(!r.audio_produced, "audio_produced should be false");
        assert!(
            !r.output_sink_connected,
            "output_sink_connected should be false"
        );
        assert!(!r.capability_exposed, "capability_exposed should be false");
        assert!(!r.thread_created, "thread_created should be false");
        assert!(
            !r.async_runtime_created,
            "async_runtime_created should be false"
        );
        assert!(
            !r.callback_registered,
            "callback_registered should be false"
        );
        assert!(
            !r.ring_buffer_created,
            "ring_buffer_created should be false"
        );
        assert!(!r.decoder_connected, "decoder_connected should be false");
        assert!(!r.pipeline_connected, "pipeline_connected should be false");
    }
}
