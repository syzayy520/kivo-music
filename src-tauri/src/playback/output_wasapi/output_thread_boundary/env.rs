// output_thread_boundary/env.rs
//
// Environment variable opt-in for output thread boundary smoke.

/// Environment variable name for output thread boundary smoke opt-in.
pub const WASAPI_OUTPUT_THREAD_SMOKE_ENV: &str = "KIVO_WASAPI_OUTPUT_THREAD_SMOKE";

/// Check if the output thread boundary smoke is opt-in enabled.
///
/// Returns true only if the environment variable is set to "1".
/// Missing or non-"1" values return false.
pub fn is_opt_in_enabled() -> bool {
    std::env::var(WASAPI_OUTPUT_THREAD_SMOKE_ENV)
        .ok()
        .as_deref()
        == Some("1")
}
