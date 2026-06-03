// env.rs
//
// Environment variable handling for start/stop smoke opt-in.
//
// This file contains the logic to check whether the smoke probe
// should be attempted based on the KIVO_WASAPI_START_STOP_SMOKE
// environment variable.

use std::env;

use super::report::WASAPI_START_STOP_SMOKE_ENV;

/// Check if the opt-in environment variable is set to "1".
pub fn is_opt_in_enabled() -> bool {
    env::var(WASAPI_START_STOP_SMOKE_ENV).ok().as_deref() == Some("1")
}
