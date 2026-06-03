// output_thread_boundary_tests/report_tests.rs
//
// Tests for WasapiOutputThreadSmokeReport builders and defaults.

use crate::playback::output_wasapi::output_thread_boundary::report::WasapiOutputThreadSmokeReport;

#[test]
fn test_report_default_has_correct_values() {
    let report = WasapiOutputThreadSmokeReport::default();

    // Platform and opt-in
    assert_eq!(report.platform, "windows");
    assert_eq!(report.opt_in_env, "KIVO_WASAPI_OUTPUT_THREAD_SMOKE");
    assert!(!report.opt_in_enabled);
    assert!(!report.attempted);
    assert!(!report.skipped);
    assert!(report.skipped_reason.is_none());
    assert!(report.error_message.is_none());
    assert!(report.error_hresult.is_none());

    // Thread lifecycle
    assert!(!report.thread_spawn_attempted);
    assert!(!report.thread_spawned);
    assert!(!report.thread_report_recv_attempted);
    assert!(!report.thread_report_received);
    assert!(report.thread_recv_timeout_ms.is_none());
    assert!(!report.thread_recv_timed_out);
    assert!(!report.thread_join_attempted);
    assert!(!report.thread_joined);
    assert!(!report.thread_join_failed);
    assert!(!report.thread_panic_caught);
    assert!(report.thread_panic_message.is_none());
    assert!(report.thread_duration_ms.is_none());

    // Thread internal WASAPI fields
    assert!(!report.com_initialized);
    assert!(!report.endpoint_available);
    assert!(!report.client_activated);
    assert!(!report.mix_format_available);
    assert!(!report.initialize_attempted);
    assert!(!report.initialized_audio_client);
    assert!(!report.get_service_attempted);
    assert!(!report.render_client_obtained);
    assert!(!report.get_buffer_size_attempted);
    assert!(report.buffer_size_frames.is_none());

    // Prefill fields
    assert!(!report.prefill_get_buffer_attempted);
    assert!(!report.prefill_buffer_obtained);
    assert!(!report.prefill_release_buffer_attempted);
    assert!(!report.prefill_buffer_released);
    assert!(report.prefill_requested_frames.is_none());
    assert!(report.prefill_released_frames.is_none());
    assert!(!report.prefill_used_silent_flag);

    // Start/Stop/Reset fields
    assert!(!report.start_attempted);
    assert!(!report.started_audio_client);
    assert!(!report.get_current_padding_attempted);
    assert!(report.current_padding_frames.is_none());
    assert!(!report.stop_attempted);
    assert!(!report.stopped_audio_client);
    assert!(!report.reset_attempted);
    assert!(!report.reset_succeeded);
    assert!(report.reset_hresult.is_none());

    // Prohibited operations (must all be false)
    assert!(!report.output_sink_connected);
    assert!(!report.capability_exposed);
    assert!(!report.thread_callback_registered);
    assert!(!report.async_runtime_created);
    assert!(!report.ring_buffer_created);
    assert!(!report.decoder_connected);
    assert!(!report.pipeline_connected);
    assert!(!report.manager_connected);
    assert!(!report.real_pcm_produced);
    assert!(!report.non_silent_data_written);
    assert!(!report.audio_produced);
    assert!(!report.playback_capability_enabled);

    // Format information
    assert!(report.sample_rate_hz.is_none());
    assert!(report.channels.is_none());
    assert!(report.bits_per_sample.is_none());
    assert!(report.block_align.is_none());
    assert!(report.avg_bytes_per_sec.is_none());
    assert!(report.format_tag.is_none());
    assert!(report.cb_size.is_none());

    // Initialize parameters
    assert_eq!(report.share_mode, "shared");
    assert_eq!(report.stream_flags, 0);
    assert_eq!(report.buffer_duration_hns, 0);
    assert_eq!(report.periodicity_hns, 0);

    // Query mode and wait strategy
    assert_eq!(report.query_mode, "output_thread_boundary");
    assert_eq!(report.wait_duration_ms, Some(0));
}

#[test]
fn test_report_skipped_env_missing() {
    let report = WasapiOutputThreadSmokeReport::skipped_env_missing();

    assert!(report.skipped);
    assert_eq!(
        report.skipped_reason,
        Some("environment variable not set to \"1\"")
    );
    assert!(report.error_message.is_none());
    assert!(report.error_hresult.is_none());
}

#[test]
fn test_report_skipped_non_windows() {
    let report = WasapiOutputThreadSmokeReport::skipped_non_windows();

    assert!(report.skipped);
    assert_eq!(report.skipped_reason, Some("not a Windows platform"));
    assert!(report.error_message.is_none());
    assert!(report.error_hresult.is_none());
}

#[test]
fn test_report_skipped_with_error() {
    let report =
        WasapiOutputThreadSmokeReport::skipped_with_error("test reason", "test error".to_string());

    assert!(report.skipped);
    assert_eq!(report.skipped_reason, Some("test reason"));
    assert_eq!(report.error_message, Some("test error".to_string()));
    assert!(report.error_hresult.is_none());
}

#[test]
fn test_report_success_has_correct_values() {
    let fields =
        crate::playback::output_wasapi::output_thread_boundary::format_fields::FormatFields {
            sample_rate_hz: 44100,
            channels: 2,
            bits_per_sample: 16,
            block_align: 4,
            avg_bytes_per_sec: 176400,
            format_tag: 1,
            cb_size: 0,
        };
    let report = WasapiOutputThreadSmokeReport::success(fields, 1024, 512, 100);

    assert!(!report.skipped);
    assert!(report.error_message.is_none());
    assert!(report.error_hresult.is_none());

    // Format fields
    assert_eq!(report.sample_rate_hz, Some(44100));
    assert_eq!(report.channels, Some(2));
    assert_eq!(report.bits_per_sample, Some(16));
    assert_eq!(report.block_align, Some(4));
    assert_eq!(report.avg_bytes_per_sec, Some(176400));
    assert_eq!(report.format_tag, Some(1));
    assert_eq!(report.cb_size, Some(0));

    // Buffer and padding
    assert_eq!(report.buffer_size_frames, Some(1024));
    assert_eq!(report.current_padding_frames, Some(512));
    assert_eq!(report.thread_duration_ms, Some(100));

    // All prohibited operations must be false
    assert!(!report.output_sink_connected);
    assert!(!report.capability_exposed);
    assert!(!report.thread_callback_registered);
    assert!(!report.async_runtime_created);
    assert!(!report.ring_buffer_created);
    assert!(!report.decoder_connected);
    assert!(!report.pipeline_connected);
    assert!(!report.manager_connected);
    assert!(!report.real_pcm_produced);
    assert!(!report.non_silent_data_written);
    assert!(!report.audio_produced);
    assert!(!report.playback_capability_enabled);
}
