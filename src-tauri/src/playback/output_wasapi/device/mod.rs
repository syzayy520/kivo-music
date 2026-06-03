// device/mod.rs
//
// Device boundary facade for WASAPI endpoint smoke.
//
// This module provides a cross-platform interface to probe whether
// a default audio endpoint is accessible via Windows COM APIs.
//
// **IMPORTANT**: This module does NOT:
//   - Open or activate audio devices for playback
//   - Initialize IAudioClient
//   - Create render clients
//   - Produce audible output
//   - Operate outside of explicit opt-in tests

#[cfg(windows)]
pub use device_windows::*;

#[cfg(not(windows))]
pub use device_stub::*;

#[cfg(windows)]
mod device_windows;

#[cfg(not(windows))]
mod device_stub;

/// Environment variable name for endpoint smoke opt-in.
pub const WASAPI_ENDPOINT_SMOKE_ENV: &str = "KIVO_WASAPI_ENDPOINT_SMOKE";

/// Report from an endpoint smoke probe.
///
/// This is NOT a playback error. It describes the result of probing
/// whether a default audio endpoint is accessible on the current platform.
#[derive(Clone, Debug)]
pub struct WasapiEndpointSmokeReport {
    /// Platform identifier ("windows" or "non-windows").
    pub platform: &'static str,
    /// Name of the opt-in environment variable.
    pub opt_in_env: &'static str,
    /// Whether the opt-in environment variable was set to "1".
    pub opt_in_enabled: bool,
    /// Whether the smoke probe was actually attempted.
    pub attempted: bool,
    /// Whether the probe was skipped (and thus endpoint_available is meaningless).
    pub skipped: bool,
    /// Reason for skipping, if any.
    pub skipped_reason: Option<&'static str>,
    /// Whether the default endpoint was successfully obtained.
    pub endpoint_available: bool,
    /// Error message if the probe failed.
    pub error_message: Option<String>,
}

impl WasapiEndpointSmokeReport {
    /// Create a skipped report for non-Windows platforms.
    pub fn skipped_non_windows() -> Self {
        Self {
            platform: "non-windows",
            opt_in_env: WASAPI_ENDPOINT_SMOKE_ENV,
            opt_in_enabled: false,
            attempted: false,
            skipped: true,
            skipped_reason: Some("unsupported platform"),
            endpoint_available: false,
            error_message: None,
        }
    }

    /// Create a skipped report for missing opt-in environment variable.
    pub fn skipped_env_missing() -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_ENDPOINT_SMOKE_ENV,
            opt_in_enabled: false,
            attempted: false,
            skipped: true,
            skipped_reason: Some("set KIVO_WASAPI_ENDPOINT_SMOKE=1 to run"),
            endpoint_available: false,
            error_message: None,
        }
    }

    /// Create a skipped report for a failed probe attempt.
    pub fn skipped_with_error(reason: &'static str, error: String) -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_ENDPOINT_SMOKE_ENV,
            opt_in_enabled: true,
            attempted: true,
            skipped: true,
            skipped_reason: Some(reason),
            endpoint_available: false,
            error_message: Some(error),
        }
    }

    /// Create a success report when endpoint is available.
    pub fn success() -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_ENDPOINT_SMOKE_ENV,
            opt_in_enabled: true,
            attempted: true,
            skipped: false,
            skipped_reason: None,
            endpoint_available: true,
            error_message: None,
        }
    }
}
