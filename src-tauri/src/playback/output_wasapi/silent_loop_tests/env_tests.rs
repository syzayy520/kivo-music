// silent_loop_tests/env_tests.rs
//
// Tests for silent loop smoke environment variable opt-in logic.
//
// All env-modifying tests are merged into a single serial test function
// to avoid parallel test contention on the same process-level env var.

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
fn opt_in_env_values_serial() {
    // SAFETY: All env var scenarios are tested serially in one function
    // to avoid parallel test race conditions on process-level env vars.
    let saved = std::env::var(WASAPI_SILENT_LOOP_SMOKE_ENV).ok();

    // Case 1: env missing -> false
    std::env::remove_var(WASAPI_SILENT_LOOP_SMOKE_ENV);
    assert!(!is_opt_in_enabled());

    // Case 2: env "0" -> false
    std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, "0");
    assert!(!is_opt_in_enabled());

    // Case 3: env "true" -> false
    std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, "true");
    assert!(!is_opt_in_enabled());

    // Case 4: env "yes" -> false
    std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, "yes");
    assert!(!is_opt_in_enabled());

    // Case 5: env "1" -> true
    std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, "1");
    assert!(is_opt_in_enabled());

    // Restore original env var
    if let Some(val) = saved {
        std::env::set_var(WASAPI_SILENT_LOOP_SMOKE_ENV, val);
    } else {
        std::env::remove_var(WASAPI_SILENT_LOOP_SMOKE_ENV);
    }
}
