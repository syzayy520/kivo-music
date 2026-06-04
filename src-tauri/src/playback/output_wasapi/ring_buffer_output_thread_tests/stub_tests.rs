use crate::playback::output_wasapi::ring_buffer_output_thread::{
    env, probe_ring_buffer_output_thread_smoke,
};

fn assert_prohibited_false(
    r: &crate::playback::output_wasapi::ring_buffer_output_thread::WasapiRingBufferOutputThreadSmokeReport,
) {
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
fn env_missing_returns_skipped() {
    std::env::remove_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV);
    let r = probe_ring_buffer_output_thread_smoke();
    assert!(!r.attempted);
    assert!(r.skipped);
    assert_eq!(r.skipped_reason, Some("env opt-in not enabled"));
    assert_prohibited_false(&r);
}

#[test]
fn env_enabled_returns_scaffold_ready() {
    std::env::set_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV, "1");
    let r = probe_ring_buffer_output_thread_smoke();
    assert!(r.attempted);
    assert!(!r.skipped);
    assert!(!r.ring_buffer_created);
    assert!(!r.output_thread_spawned);
    assert_prohibited_false(&r);
    // Clean up
    std::env::remove_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV);
}

#[test]
fn probe_does_not_create_output_thread() {
    std::env::set_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV, "1");
    let r = probe_ring_buffer_output_thread_smoke();
    assert!(!r.output_thread_spawned);
    assert!(!r.output_thread_spawn_attempted);
    assert!(!r.output_thread_joined);
    std::env::remove_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV);
}

#[test]
fn probe_does_not_create_ring_buffer() {
    std::env::set_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV, "1");
    let r = probe_ring_buffer_output_thread_smoke();
    assert!(!r.ring_buffer_created);
    assert!(r.ring_buffer_capacity_frames.is_none());
    std::env::remove_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV);
}

#[test]
fn probe_does_not_write_pcm() {
    std::env::set_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV, "1");
    let r = probe_ring_buffer_output_thread_smoke();
    assert!(!r.real_pcm_produced);
    assert!(!r.non_silent_data_written);
    assert!(!r.audio_produced);
    std::env::remove_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV);
}

#[test]
fn probe_prohibited_fields_false() {
    std::env::set_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV, "1");
    let r = probe_ring_buffer_output_thread_smoke();
    assert_prohibited_false(&r);
    std::env::remove_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV);
}
