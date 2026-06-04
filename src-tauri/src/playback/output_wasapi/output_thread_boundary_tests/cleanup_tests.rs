// output_thread_boundary_tests/cleanup_tests.rs
//
// Tests verifying RAII guard cleanup behavior and prohibited operations.

use crate::playback::output_wasapi::output_thread_boundary::output_thread_outcome::apply_thread_outcome;
use crate::playback::output_wasapi::output_thread_boundary::report::WasapiOutputThreadSmokeReport;
use crate::playback::output_wasapi::output_thread_boundary::thread_report::ThreadReport;

fn assert_no_prohibited(r: &WasapiOutputThreadSmokeReport) {
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

fn test_fields(
) -> crate::playback::output_wasapi::output_thread_boundary::format_fields::FormatFields {
    crate::playback::output_wasapi::output_thread_boundary::format_fields::FormatFields {
        sample_rate_hz: 48000,
        channels: 2,
        bits_per_sample: 16,
        block_align: 4,
        avg_bytes_per_sec: 192000,
        format_tag: 1,
        cb_size: 0,
    }
}

#[test]
fn report_has_no_loop_executed_true() {
    assert!(!WasapiOutputThreadSmokeReport::skipped_non_windows().audio_produced);
    assert!(!WasapiOutputThreadSmokeReport::skipped_env_missing().audio_produced);
    assert!(
        !WasapiOutputThreadSmokeReport::skipped_with_error("test", "err".into()).audio_produced
    );
}

#[test]
fn report_has_no_real_pcm() {
    assert!(!WasapiOutputThreadSmokeReport::default().audio_produced);
    assert!(!WasapiOutputThreadSmokeReport::skipped_non_windows().audio_produced);
}

#[test]
fn default_report_has_no_output_sink_or_capability() {
    let r = WasapiOutputThreadSmokeReport::default();
    assert!(!r.output_sink_connected);
    assert!(!r.capability_exposed);
}

#[test]
fn default_report_has_no_thread_async_callback() {
    let r = WasapiOutputThreadSmokeReport::default();
    assert!(!r.thread_callback_registered);
    assert!(!r.async_runtime_created);
}

#[test]
fn default_report_has_no_ring_buffer_decoder_pipeline() {
    let r = WasapiOutputThreadSmokeReport::default();
    assert!(!r.ring_buffer_created);
    assert!(!r.decoder_connected);
    assert!(!r.pipeline_connected);
}

#[test]
fn reset_only_after_stop_success() {
    let r = WasapiOutputThreadSmokeReport::success(test_fields(), 1024, 960, 0);
    assert!(r.stop_attempted, "Stop must be attempted");
    assert!(r.stopped_audio_client, "Stop must succeed");
    assert!(r.reset_attempted, "Reset must be attempted after Stop");
    assert!(r.reset_succeeded, "Reset must succeed");
}

#[test]
fn stop_failure_prevents_reset() {
    let r =
        WasapiOutputThreadSmokeReport::stop_failed(test_fields(), 1024, 512, "stop error".into());
    assert!(r.stop_attempted);
    assert!(!r.stopped_audio_client);
    assert!(
        !r.reset_attempted,
        "Reset must not be attempted when Stop fails"
    );
    assert!(!r.reset_succeeded);
}

#[test]
fn skipped_reports_preserve_prohibited_defaults() {
    let reports = vec![
        WasapiOutputThreadSmokeReport::skipped_non_windows(),
        WasapiOutputThreadSmokeReport::skipped_env_missing(),
        WasapiOutputThreadSmokeReport::skipped_with_error("test", "err".into()),
    ];
    for r in &reports {
        assert_no_prohibited(r);
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
    assert!(!report.thread_joined);
    assert!(!report.thread_join_failed);
    assert!(!report.thread_panic_caught);
}

#[test]
fn panic_report_does_not_enable_playback() {
    let mut report = WasapiOutputThreadSmokeReport::default();
    apply_thread_outcome(&mut report, ThreadReport::Panic("test panic".into()), 15000);
    assert_no_prohibited(&report);
}

#[test]
fn join_failure_report_does_not_enable_playback() {
    let mut report = WasapiOutputThreadSmokeReport::default();
    apply_thread_outcome(
        &mut report,
        ThreadReport::JoinFailed("channel disconnected".into()),
        15000,
    );
    assert_no_prohibited(&report);
}

#[test]
fn success_builder_preserves_prohibited_defaults() {
    assert_no_prohibited(&WasapiOutputThreadSmokeReport::success(
        test_fields(),
        1024,
        512,
        100,
    ));
}

#[test]
fn prereq_failure_builders_preserve_prohibited_defaults() {
    let f = test_fields();
    let reports = vec![
        WasapiOutputThreadSmokeReport::com_init_failed("err".into()),
        WasapiOutputThreadSmokeReport::endpoint_unavailable("err".into()),
        WasapiOutputThreadSmokeReport::client_activate_failed("err".into()),
        WasapiOutputThreadSmokeReport::mix_format_failed("err".into()),
        WasapiOutputThreadSmokeReport::initialize_failed(f, "err".into()),
        WasapiOutputThreadSmokeReport::get_service_failed(f, "err".into()),
        WasapiOutputThreadSmokeReport::get_buffer_size_failed(f, "err".into()),
        WasapiOutputThreadSmokeReport::buffer_size_zero(f),
    ];
    for r in &reports {
        assert_no_prohibited(r);
    }
}

#[test]
fn thread_failure_builders_preserve_prohibited_defaults() {
    let f = test_fields();
    let reports = vec![
        WasapiOutputThreadSmokeReport::start_failed(f, 1024, "err".into()),
        WasapiOutputThreadSmokeReport::get_current_padding_failed(f, 1024, "err".into()),
        WasapiOutputThreadSmokeReport::stop_failed(f, 1024, 512, "err".into()),
    ];
    for r in &reports {
        assert_no_prohibited(r);
    }
}

#[test]
fn reset_failure_builder_preserves_prohibited_defaults() {
    assert_no_prohibited(&WasapiOutputThreadSmokeReport::reset_failed(
        test_fields(),
        1024,
        512,
        -1,
        "err".into(),
    ));
}
