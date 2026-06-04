use crate::playback::output_wasapi::ring_buffer_output_thread::WasapiRingBufferOutputThreadSmokeReport;

fn assert_prohibited_false(r: &WasapiRingBufferOutputThreadSmokeReport) {
    assert!(!r.output_sink_connected);
    assert!(!r.capability_exposed);
    assert!(!r.decoder_connected);
    assert!(!r.pipeline_connected);
    assert!(!r.manager_connected);
    assert!(!r.real_pcm_produced);
    assert!(!r.non_silent_data_written);
    assert!(!r.audio_produced);
    assert!(!r.playback_capability_enabled);
}

#[test]
fn default_report_prohibited_fields_false() {
    let r = WasapiRingBufferOutputThreadSmokeReport::default();
    assert_prohibited_false(&r);
}

#[test]
fn default_report_thread_lifecycle_false() {
    let r = WasapiRingBufferOutputThreadSmokeReport::default();
    assert!(!r.output_thread_spawn_attempted);
    assert!(!r.output_thread_spawned);
    assert!(!r.thread_report_recv_attempted);
    assert!(!r.thread_report_received);
    assert!(r.thread_recv_timeout_ms.is_none());
    assert!(!r.output_thread_join_attempted);
    assert!(!r.output_thread_joined);
    assert!(!r.output_thread_join_failed);
    assert!(!r.thread_panic_caught);
    assert!(r.thread_panic_message.is_none());
    assert!(!r.thread_recv_timed_out);
    assert!(r.thread_duration_ms.is_none());
}

#[test]
fn default_report_ring_buffer_fields_zero() {
    let r = WasapiRingBufferOutputThreadSmokeReport::default();
    assert!(!r.ring_buffer_created);
    assert!(r.ring_buffer_capacity_frames.is_none());
    assert!(r.ring_buffer_available_frames.is_none());
    assert!(!r.ring_buffer_closed);
    assert_eq!(r.ring_buffer_underrun_count, 0);
    assert_eq!(r.ring_buffer_overrun_count, 0);
    assert_eq!(r.total_frames_written, 0);
    assert_eq!(r.total_frames_read, 0);
    assert_eq!(r.total_silence_frames_filled, 0);
}

#[test]
fn default_report_wasapi_lifecycle_false() {
    let r = WasapiRingBufferOutputThreadSmokeReport::default();
    assert!(!r.com_initialized);
    assert!(!r.endpoint_available);
    assert!(!r.client_activated);
    assert!(!r.mix_format_available);
    assert!(r.sample_rate_hz.is_none());
    assert!(r.channels.is_none());
    assert!(r.bits_per_sample.is_none());
    assert!(r.block_align.is_none());
    assert!(!r.initialize_attempted);
    assert!(!r.initialized_audio_client);
    assert!(!r.get_service_attempted);
    assert!(!r.render_client_obtained);
    assert!(!r.get_buffer_size_attempted);
    assert!(r.buffer_size_frames.is_none());
    assert!(!r.wasapi_buffer_obtained);
    assert!(!r.wasapi_buffer_released);
    assert!(!r.used_silent_flag);
    assert!(!r.start_attempted);
    assert!(!r.started_audio_client);
    assert!(!r.get_current_padding_attempted);
    assert!(r.current_padding_frames.is_none());
    assert!(!r.stop_attempted);
    assert!(!r.stopped_audio_client);
    assert!(!r.reset_attempted);
    assert!(!r.reset_succeeded);
}

#[test]
fn skipped_env_missing_fields() {
    let r = WasapiRingBufferOutputThreadSmokeReport::skipped_env_missing();
    assert!(!r.attempted);
    assert!(r.skipped);
    assert_eq!(r.skipped_reason, Some("env opt-in not enabled"));
    assert_prohibited_false(&r);
}

#[test]
fn skipped_non_windows_fields() {
    let r = WasapiRingBufferOutputThreadSmokeReport::skipped_non_windows();
    assert!(!r.attempted);
    assert!(r.skipped);
    assert_eq!(r.skipped_reason, Some("non-windows platform"));
    assert_prohibited_false(&r);
}

#[test]
fn scaffold_ready_report_fields() {
    let r = WasapiRingBufferOutputThreadSmokeReport::scaffold_ready_report();
    assert!(r.attempted);
    assert!(!r.skipped);
    assert!(!r.ring_buffer_created);
    assert!(!r.output_thread_spawned);
    assert_prohibited_false(&r);
}

#[test]
fn scaffold_ready_report_no_thread() {
    let r = WasapiRingBufferOutputThreadSmokeReport::scaffold_ready_report();
    assert!(!r.output_thread_spawned);
    assert!(!r.output_thread_spawn_attempted);
    assert!(!r.output_thread_joined);
}

#[test]
fn scaffold_ready_report_no_real_pcm() {
    let r = WasapiRingBufferOutputThreadSmokeReport::scaffold_ready_report();
    assert!(!r.real_pcm_produced);
    assert!(!r.non_silent_data_written);
    assert!(!r.audio_produced);
}
