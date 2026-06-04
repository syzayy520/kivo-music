pub const RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV: &str = "KIVO_WASAPI_RING_BUFFER_OUTPUT_THREAD_SMOKE";

pub fn is_opt_in_enabled() -> bool {
    std::env::var(RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV)
        .ok()
        .as_deref()
        == Some("1")
}
