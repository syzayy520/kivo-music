// silent_loop/env.rs
//
// Environment variable handling for silent loop smoke opt-in.
//
// This file contains the logic to check whether the smoke probe
// should be attempted based on the KIVO_WASAPI_SILENT_LOOP_SMOKE
// environment variable.

use std::env;

use super::report::WASAPI_SILENT_LOOP_SMOKE_ENV;

/// Check if the opt-in environment variable is set to "1".
pub fn is_opt_in_enabled() -> bool {
    env::var(WASAPI_SILENT_LOOP_SMOKE_ENV).ok().as_deref() == Some("1")
}
