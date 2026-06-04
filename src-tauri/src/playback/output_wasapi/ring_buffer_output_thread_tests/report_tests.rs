use crate::playback::output_wasapi::ring_buffer::RingBufferStats;
use crate::playback::output_wasapi::ring_buffer_output_thread::WasapiRingBufferOutputThreadSmokeReport;

use super::smoke_assertions;

#[test]
fn default_report_prohibited_fields_false() {
    let r = WasapiRingBufferOutputThreadSmokeReport::default();
    smoke_assertions::assert_prohibited_always_false(&r);
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
    assert!(!r.initialize_attempted);
    assert!(!r.initialized_audio_client);
    assert!(!r.render_client_obtained);
    assert!(r.buffer_size_frames.is_none());
    assert!(!r.wasapi_buffer_obtained);
    assert!(!r.wasapi_buffer_released);
    assert!(!r.used_silent_flag);
    assert!(!r.started_audio_client);
    assert!(!r.stopped_audio_client);
    assert!(!r.reset_succeeded);
}

#[test]
fn skipped_env_missing_fields() {
    let r = WasapiRingBufferOutputThreadSmokeReport::skipped_env_missing();
    assert!(!r.attempted);
    assert!(r.skipped);
    assert_eq!(r.skipped_reason, Some("env opt-in not enabled"));
    smoke_assertions::assert_prohibited_always_false(&r);
}

#[test]
fn skipped_non_windows_fields() {
    let r = WasapiRingBufferOutputThreadSmokeReport::skipped_non_windows();
    assert!(!r.attempted);
    assert!(r.skipped);
    assert_eq!(r.skipped_reason, Some("non-windows platform"));
    smoke_assertions::assert_prohibited_always_false(&r);
}

#[test]
fn scaffold_ready_report_fields() {
    let r = WasapiRingBufferOutputThreadSmokeReport::scaffold_ready_report();
    assert!(r.attempted);
    assert!(!r.skipped);
    assert!(!r.ring_buffer_created);
    assert!(!r.output_thread_spawned);
    smoke_assertions::assert_prohibited_always_false(&r);
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

#[test]
fn apply_ring_buffer_stats_populates_fields() {
    let mut r = WasapiRingBufferOutputThreadSmokeReport::default();
    let stats = RingBufferStats {
        underrun_count: 3,
        overrun_count: 1,
        total_frames_written: 100,
        total_frames_read: 50,
        total_silence_frames_filled: 200,
    };
    r.apply_ring_buffer_stats(&stats);
    assert_eq!(r.ring_buffer_underrun_count, 3);
    assert_eq!(r.ring_buffer_overrun_count, 1);
    assert_eq!(r.total_frames_written, 100);
    assert_eq!(r.total_frames_read, 50);
    assert_eq!(r.total_silence_frames_filled, 200);
    smoke_assertions::assert_prohibited_always_false(&r);
}

#[test]
fn merge_from_copies_thread_lifecycle() {
    let mut target = WasapiRingBufferOutputThreadSmokeReport::default();
    let mut source = WasapiRingBufferOutputThreadSmokeReport::default();
    source.thread_report_received = true;
    source.output_thread_join_attempted = true;
    source.output_thread_joined = true;
    source.thread_duration_ms = Some(42);

    target.merge_from(source);

    assert!(target.thread_report_recv_attempted, "recv_attempted set by merge");
    assert!(target.thread_report_received);
    assert!(target.output_thread_join_attempted);
    assert!(target.output_thread_joined);
    assert!(!target.thread_panic_caught);
    assert!(!target.thread_recv_timed_out);
    assert_eq!(target.thread_duration_ms, Some(42));
    smoke_assertions::assert_prohibited_always_false(&target);
}

#[test]
fn merge_from_copies_ring_buffer_fields() {
    let mut target = WasapiRingBufferOutputThreadSmokeReport::default();
    let mut source = WasapiRingBufferOutputThreadSmokeReport::default();
    source.ring_buffer_created = true;
    source.ring_buffer_capacity_frames = Some(4800);
    source.ring_buffer_available_frames = Some(4800);
    source.ring_buffer_closed = true;
    source.ring_buffer_underrun_count = 5;
    source.ring_buffer_overrun_count = 2;
    source.total_frames_written = 1000;
    source.total_frames_read = 500;
    source.total_silence_frames_filled = 4800;

    target.merge_from(source);

    assert!(target.ring_buffer_created);
    assert_eq!(target.ring_buffer_capacity_frames, Some(4800));
    assert_eq!(target.ring_buffer_available_frames, Some(4800));
    assert!(target.ring_buffer_closed);
    assert_eq!(target.ring_buffer_underrun_count, 5);
    assert_eq!(target.ring_buffer_overrun_count, 2);
    assert_eq!(target.total_frames_written, 1000);
    assert_eq!(target.total_frames_read, 500);
    assert_eq!(target.total_silence_frames_filled, 4800);
}

#[test]
fn merge_from_copies_wasapi_lifecycle_fields() {
    let mut target = WasapiRingBufferOutputThreadSmokeReport::default();
    let mut source = WasapiRingBufferOutputThreadSmokeReport::default();
    source.com_initialized = true;
    source.endpoint_available = true;
    source.client_activated = true;
    source.mix_format_available = true;
    source.initialized_audio_client = true;
    source.render_client_obtained = true;
    source.buffer_size_frames = Some(960);
    source.wasapi_buffer_obtained = true;
    source.wasapi_buffer_released = true;
    source.used_silent_flag = true;
    source.started_audio_client = true;
    source.stopped_audio_client = true;
    source.reset_succeeded = true;

    target.merge_from(source);

    assert!(target.com_initialized);
    assert!(target.endpoint_available);
    assert!(target.mix_format_available);
    assert!(target.initialized_audio_client);
    assert_eq!(target.buffer_size_frames, Some(960));
    assert!(target.used_silent_flag);
    assert!(target.started_audio_client);
    assert!(target.reset_succeeded);
}

#[test]
fn merge_from_preserves_existing_optional_when_source_none() {
    let mut target = WasapiRingBufferOutputThreadSmokeReport::default();
    target.error_message = Some("existing error".to_string());
    target.skipped_reason = Some("existing reason");

    let source = WasapiRingBufferOutputThreadSmokeReport::default();
    target.merge_from(source);

    assert_eq!(target.error_message, Some("existing error".to_string()));
    assert_eq!(target.skipped_reason, Some("existing reason"));
}
