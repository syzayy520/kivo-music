// output_thread_boundary_tests/env_tests.rs
//
// Tests for environment variable opt-in logic.

use crate::playback::output_wasapi::output_thread_boundary::env::{
    is_opt_in_enabled, WASAPI_OUTPUT_THREAD_SMOKE_ENV,
};

#[test]
fn test_env_var_missing_returns_false() {
    // Save and remove the env var if present
    let saved = std::env::var(WASAPI_OUTPUT_THREAD_SMOKE_ENV).ok();
    std::env::remove_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV);

    assert!(!is_opt_in_enabled());

    // Restore the env var if it was present
    if let Some(val) = saved {
        std::env::set_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV, val);
    }
}

#[test]
fn test_env_var_empty_returns_false() {
    let saved = std::env::var(WASAPI_OUTPUT_THREAD_SMOKE_ENV).ok();
    std::env::set_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV, "");

    assert!(!is_opt_in_enabled());

    if let Some(val) = saved {
        std::env::set_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV, val);
    } else {
        std::env::remove_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV);
    }
}

#[test]
fn test_env_var_zero_returns_false() {
    let saved = std::env::var(WASAPI_OUTPUT_THREAD_SMOKE_ENV).ok();
    std::env::set_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV, "0");

    assert!(!is_opt_in_enabled());

    if let Some(val) = saved {
        std::env::set_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV, val);
    } else {
        std::env::remove_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV);
    }
}

#[test]
fn test_env_var_one_returns_true() {
    let saved = std::env::var(WASAPI_OUTPUT_THREAD_SMOKE_ENV).ok();
    std::env::set_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV, "1");

    assert!(is_opt_in_enabled());

    if let Some(val) = saved {
        std::env::set_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV, val);
    } else {
        std::env::remove_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV);
    }
}

#[test]
fn test_env_var_yes_returns_false() {
    let saved = std::env::var(WASAPI_OUTPUT_THREAD_SMOKE_ENV).ok();
    std::env::set_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV, "yes");

    assert!(!is_opt_in_enabled());

    if let Some(val) = saved {
        std::env::set_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV, val);
    } else {
        std::env::remove_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV);
    }
}

#[test]
fn test_env_var_true_returns_false() {
    let saved = std::env::var(WASAPI_OUTPUT_THREAD_SMOKE_ENV).ok();
    std::env::set_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV, "true");

    assert!(!is_opt_in_enabled());

    if let Some(val) = saved {
        std::env::set_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV, val);
    } else {
        std::env::remove_var(WASAPI_OUTPUT_THREAD_SMOKE_ENV);
    }
}
