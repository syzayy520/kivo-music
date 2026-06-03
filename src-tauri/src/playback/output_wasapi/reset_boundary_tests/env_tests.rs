// reset_boundary_tests/env_tests.rs
//
// Tests for reset boundary smoke environment variable opt-in logic.
// All env checks are combined into a single test to avoid Windows
// env var race conditions when tests run in parallel within the same
// process. env::set_var / env::remove_var affect the entire process,
// so parallel test execution can cause intermittent failures.

use crate::playback::output_wasapi::reset_boundary::env::is_opt_in_enabled;
use crate::playback::output_wasapi::reset_boundary::report::WASAPI_RESET_SMOKE_ENV;

#[test]
fn env_opt_in_works_correctly() {
    // 1. Env name constant is correct
    assert_eq!(WASAPI_RESET_SMOKE_ENV, "KIVO_WASAPI_RESET_SMOKE");

    // 2. Opt-in false when env missing
    std::env::remove_var(WASAPI_RESET_SMOKE_ENV);
    assert!(!is_opt_in_enabled());

    // 3. Opt-in false for non-"1" values
    for val in &["0", "true", "yes", "on", "2"] {
        std::env::set_var(WASAPI_RESET_SMOKE_ENV, val);
        assert!(!is_opt_in_enabled(), "should be false for '{val}'");
    }

    // 4. Opt-in true when env is "1"
    std::env::set_var(WASAPI_RESET_SMOKE_ENV, "1");
    assert!(is_opt_in_enabled());

    // Cleanup
    std::env::remove_var(WASAPI_RESET_SMOKE_ENV);
}
