use crate::playback::output_wasapi::ring_buffer_output_thread::env;

#[test]
fn env_opt_in_serial() {
    // Clean up first
    std::env::remove_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV);
    assert!(!env::is_opt_in_enabled());

    // "0" => false
    std::env::set_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV, "0");
    assert!(!env::is_opt_in_enabled());

    // "true" => false
    std::env::set_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV, "true");
    assert!(!env::is_opt_in_enabled());

    // "1" => true
    std::env::set_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV, "1");
    assert!(env::is_opt_in_enabled());

    // Clean up
    std::env::remove_var(env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV);
    assert!(!env::is_opt_in_enabled());
}
