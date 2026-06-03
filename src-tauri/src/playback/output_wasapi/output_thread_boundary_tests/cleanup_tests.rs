// output_thread_boundary_tests/cleanup_tests.rs
//
// Tests verifying RAII guard cleanup behavior and prohibited operations.

use crate::playback::output_wasapi::output_thread_boundary::output_thread_outcome::apply_thread_outcome;
use crate::playback::output_wasapi::output_thread_boundary::report::WasapiOutputThreadSmokeReport;
use crate::playback::output_wasapi::output_thread_boundary::thread_report::ThreadReport;

#[test]
fn report_has_no_loop_executed_true() {
    let r = WasapiOutputThreadSmokeReport::skipped_non_windows();
    assert!(!r.audio_produced, "no audio in skipped report");
    let r2 = WasapiOutputThreadSmokeReport::skipped_env_missing();
    assert!(!r2.audio_produced, "no audio in env_missing report");
    let r3 = WasapiOutputThreadSmokeReport::skipped_with_error("test", "err".into());
    assert!(!r3.audio_produced, "no audio in error report");
}

#[test]
fn report_has_no_real_pcm() {
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
    assert!(r.stop_attempted, "Stop must be attempted");
    assert!(r.stopped_audio_client, "Stop must succeed");
    assert!(r.reset_attempted, "Reset must be attempted after Stop");
    assert!(r.reset_succeeded, "Reset must succeed");
}

#[test]
fn stop_failure_prevents_reset() {
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
    let reports = vec![
        WasapiOutputThreadSmokeReport::skipped_non_windows(),
        WasapiOutputThreadSmokeReport::skipped_env_missing(),
        WasapiOutputThreadSmokeReport::skipped_with_error("test", "err".into()),
    ];
    for r in &reports {
        assert!(!r.output_sink_connected);
        assert!(!r.capability_exposed);
        assert!(!r.thread_callback_registered);
        assert!(!r.async_runtime_created);
        assert!(!r.ring_buffer_created);
        assert!(!r.decoder_connected);
        assert!(!r.pipeline_connected);
        assert!(!r.manager_connected);
        assert!(!r.real_pcm_produced);
        assert!(!r.non_silent_data_written);
        assert!(!r.audio_produced);
        assert!(!r.playback_capability_enabled);
    }
}

#[test]
fn timeout_report_has_no_join_attempted() {
    let mut report = WasapiOutputThreadSmokeReport::default();
    apply_thread_outcome(&mut report, ThreadReport::Timeout, 15000);
    assert!(
        !report.thread_join_attempted,
        "Timeout should not attempt join"
    );
    assert!(!report.thread_joined, "Timeout should not join");
    assert!(
        !report.thread_join_failed,
        "Timeout should not have join failure"
    );
    assert!(
        !report.thread_panic_caught,
        "Timeout should not catch panic"
    );
}

#[test]
fn panic_report_does_not_enable_playback() {
    let mut report = WasapiOutputThreadSmokeReport::default();
    apply_thread_outcome(
        &mut report,
        ThreadReport::Panic("test panic".to_string()),
        15000,
    );
    assert!(!report.output_sink_connected);
    assert!(!report.capability_exposed);
    assert!(!report.ring_buffer_created);
    assert!(!report.decoder_connected);
    assert!(!report.pipeline_connected);
    assert!(!report.manager_connected);
    assert!(!report.real_pcm_produced);
    assert!(!report.non_silent_data_written);
    assert!(!report.audio_produced);
}

#[test]
fn join_failure_report_does_not_enable_playback() {
    let mut report = WasapiOutputThreadSmokeReport::default();
    apply_thread_outcome(
        &mut report,
        ThreadReport::JoinFailed("channel disconnected".to_string()),
        15000,
    );
    assert!(!report.output_sink_connected);
    assert!(!report.capability_exposed);
    assert!(!report.ring_buffer_created);
    assert!(!report.decoder_connected);
    assert!(!report.pipeline_connected);
    assert!(!report.manager_connected);
    assert!(!report.real_pcm_produced);
    assert!(!report.non_silent_data_written);
    assert!(!report.audio_produced);
}
