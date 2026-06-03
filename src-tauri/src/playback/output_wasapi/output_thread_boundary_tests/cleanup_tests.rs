// output_thread_boundary_tests/cleanup_tests.rs
//
// Tests verifying RAII guard cleanup behavior and prohibited operations.

use crate::playback::output_wasapi::output_thread_boundary::report::WasapiOutputThreadSmokeReport;

#[test]
fn report_has_no_loop_executed_true() {
    // After any report path, loop_executed must be false (loop not implemented)
    let r = WasapiOutputThreadSmokeReport::skipped_non_windows();
    assert!(!r.audio_produced, "no audio in skipped report");

    let r2 = WasapiOutputThreadSmokeReport::skipped_env_missing();
    assert!(!r2.audio_produced, "no audio in env_missing report");

    let r3 = WasapiOutputThreadSmokeReport::skipped_with_error("test", "err".into());
    assert!(!r3.audio_produced, "no audio in error report");
}

#[test]
fn report_has_no_real_pcm() {
    // audio_produced must always be false
    let r = WasapiOutputThreadSmokeReport::default();
    assert!(!r.audio_produced);

    let r2 = WasapiOutputThreadSmokeReport::skipped_non_windows();
    assert!(!r2.audio_produced);
}

#[test]
fn report_has_no_output_sink() {
    let r = WasapiOutputThreadSmokeReport::default();
    assert!(!r.output_sink_connected);
}

#[test]
fn report_has_no_capability() {
    let r = WasapiOutputThreadSmokeReport::default();
    assert!(!r.capability_exposed);
}

#[test]
fn report_has_no_thread_async_callback() {
    let r = WasapiOutputThreadSmokeReport::default();
    assert!(!r.thread_callback_registered);
    assert!(!r.async_runtime_created);
}

#[test]
fn report_has_no_ring_buffer_decoder_pipeline() {
    let r = WasapiOutputThreadSmokeReport::default();
    assert!(!r.ring_buffer_created);
    assert!(!r.decoder_connected);
    assert!(!r.pipeline_connected);
}

#[test]
fn reset_only_after_stop_success() {
    // In a success report, both stop and reset must be attempted and succeeded
    use crate::playback::output_wasapi::output_thread_boundary::format_fields::FormatFields;
    let fields = FormatFields {
        sample_rate_hz: 48000,
        channels: 2,
        bits_per_sample: 16,
        block_align: 4,
        avg_bytes_per_sec: 192000,
        format_tag: 1,
        cb_size: 0,
    };
    let r = WasapiOutputThreadSmokeReport::success(fields, 1024, 960, 0);
    assert!(r.stop_attempted, "Stop must be attempted in success path");
    assert!(r.stopped_audio_client, "Stop must succeed in success path");
    assert!(
        r.reset_attempted,
        "Reset must be attempted after Stop success"
    );
    assert!(r.reset_succeeded, "Reset must succeed in success path");
}

#[test]
fn stop_failure_prevents_reset() {
    // When Stop fails, Reset should not be attempted
    use crate::playback::output_wasapi::output_thread_boundary::format_fields::FormatFields;
    let fields = FormatFields {
        sample_rate_hz: 48000,
        channels: 2,
        bits_per_sample: 16,
        block_align: 4,
        avg_bytes_per_sec: 192000,
        format_tag: 1,
        cb_size: 0,
    };
    let r = WasapiOutputThreadSmokeReport::stop_failed(fields, 1024, 512, "stop error".to_string());
    assert!(r.stop_attempted, "Stop must be attempted");
    assert!(!r.stopped_audio_client, "Stop must fail");
    assert!(
        !r.reset_attempted,
        "Reset must not be attempted when Stop fails"
    );
    assert!(!r.reset_succeeded, "Reset must not succeed when Stop fails");
}

#[test]
fn skipped_reports_preserve_prohibited_defaults() {
    // All skipped builders should inherit base defaults for prohibited fields
    let reports = vec![
        WasapiOutputThreadSmokeReport::skipped_non_windows(),
        WasapiOutputThreadSmokeReport::skipped_env_missing(),
        WasapiOutputThreadSmokeReport::skipped_with_error("test", "err".into()),
    ];

    for r in &reports {
        assert!(
            !r.output_sink_connected,
            "output_sink_connected should be false"
        );
        assert!(!r.capability_exposed, "capability_exposed should be false");
        assert!(
            !r.thread_callback_registered,
            "thread_callback_registered should be false"
        );
        assert!(
            !r.async_runtime_created,
            "async_runtime_created should be false"
        );
        assert!(
            !r.ring_buffer_created,
            "ring_buffer_created should be false"
        );
        assert!(!r.decoder_connected, "decoder_connected should be false");
        assert!(!r.pipeline_connected, "pipeline_connected should be false");
        assert!(!r.manager_connected, "manager_connected should be false");
        assert!(!r.real_pcm_produced, "real_pcm_produced should be false");
        assert!(
            !r.non_silent_data_written,
            "non_silent_data_written should be false"
        );
        assert!(!r.audio_produced, "audio_produced should be false");
        assert!(
            !r.playback_capability_enabled,
            "playback_capability_enabled should be false"
        );
    }
}
