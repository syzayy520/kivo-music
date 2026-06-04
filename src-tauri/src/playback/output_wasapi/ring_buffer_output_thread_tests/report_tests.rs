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
fn scaffold_ready_report_no_ring_buffer() {
    let r = WasapiRingBufferOutputThreadSmokeReport::scaffold_ready_report();
    assert!(!r.ring_buffer_created);
    assert!(r.ring_buffer_capacity_frames.is_none());
    assert!(r.ring_buffer_available_frames.is_none());
}

#[test]
fn scaffold_ready_report_no_thread() {
    let r = WasapiRingBufferOutputThreadSmokeReport::scaffold_ready_report();
    assert!(!r.output_thread_spawned);
    assert!(!r.output_thread_spawn_attempted);
    assert!(!r.output_thread_joined);
}

#[test]
fn scaffold_ready_report_no_output_sink() {
    let r = WasapiRingBufferOutputThreadSmokeReport::scaffold_ready_report();
    assert!(!r.output_sink_connected);
    assert!(!r.capability_exposed);
}

#[test]
fn scaffold_ready_report_no_decoder_pipeline_manager() {
    let r = WasapiRingBufferOutputThreadSmokeReport::scaffold_ready_report();
    assert!(!r.decoder_connected);
    assert!(!r.pipeline_connected);
    assert!(!r.manager_connected);
}

#[test]
fn scaffold_ready_report_no_real_pcm() {
    let r = WasapiRingBufferOutputThreadSmokeReport::scaffold_ready_report();
    assert!(!r.real_pcm_produced);
    assert!(!r.non_silent_data_written);
    assert!(!r.audio_produced);
}
