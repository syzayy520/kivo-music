// silent_loop_tests/env_tests.rs
//
// Tests for silent loop smoke environment variable opt-in logic.
// Each test does remove_var before set_var to avoid Windows env var
// interference between tests running in the same process.

use crate::playback::output_wasapi::silent_loop::env::is_opt_in_enabled;
use crate::playback::output_wasapi::silent_loop::report::WASAPI_SILENT_LOOP_SMOKE_ENV;

#[test]
fn env_name_is_correct() {
    assert_eq!(
        WASAPI_SILENT_LOOP_SMOKE_ENV,
        "KIVO_WASAPI_SILENT_LOOP_SMOKE"
    );
}

#[test]
fn opt_in_false_when_env_missing() {
    std::env::remove_var(WASAPI_SILENT_LOOP_SMOKE_ENV);
    assert!(!is_opt_in_enabled());
}

#[test]
fn opt_in_false_when_env_not_one() {
    for val in &["0", "true", "yes", "on", "2"] {
        std::env::remove_var(WASAPI_SILENT_LOOP_SMOKE_ENV);
        std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, val);
        assert!(!is_opt_in_enabled(), "should be false for '{val}'");
    }
    std::env::remove_var(WASAPI_SILENT_LOOP_SMOKE_ENV);
}

#[test]
fn opt_in_true_when_env_is_one() {
    std::env::remove_var(WASAPI_SILENT_LOOP_SMOKE_ENV);
    std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, "1");
    assert!(is_opt_in_enabled());
    std::env::remove_var(WASAPI_SILENT_LOOP_SMOKE_ENV);
}
