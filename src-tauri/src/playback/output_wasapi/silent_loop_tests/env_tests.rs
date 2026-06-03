// silent_loop_tests/env_tests.rs
//
// Tests for silent loop smoke environment variable opt-in logic.

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
    // SAFETY: We save and restore the env var to avoid test interference.
    // In CI, tests run in separate processes so this is safe.
    let saved = std::env::var(WASAPI_SILENT_LOOP_SMOKE_ENV).ok();
    std::env::remove_var(WASAPI_SILENT_LOOP_SMOKE_ENV);
    assert!(!is_opt_in_enabled());
    if let Some(val) = saved {
        std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, val);
    }
}

#[test]
fn opt_in_false_when_env_not_one() {
    let saved = std::env::var(WASAPI_SILENT_LOOP_SMOKE_ENV).ok();
    std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, "0");
    assert!(!is_opt_in_enabled());
    std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, "true");
    assert!(!is_opt_in_enabled());
    std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, "yes");
    assert!(!is_opt_in_enabled());
    if let Some(val) = saved {
        std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, val);
    } else {
        std::env::remove_var(WASAPI_SILENT_LOOP_SMOKE_ENV);
    }
}

#[test]
fn opt_in_true_when_env_is_one() {
    let saved = std::env::var(WASAPI_SILENT_LOOP_SMOKE_ENV).ok();
    std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, "1");
    assert!(is_opt_in_enabled());
    if let Some(val) = saved {
        std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, val);
    } else {
        std::env::remove_var(WASAPI_SILENT_LOOP_SMOKE_ENV);
    }
}
