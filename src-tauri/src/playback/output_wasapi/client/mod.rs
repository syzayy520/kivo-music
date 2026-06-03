// client/mod.rs
//
// Client boundary facade for WASAPI IAudioClient Activate smoke.
//
// This module provides a cross-platform interface to probe whether
// an IAudioClient can be activated from the default audio endpoint
// via Windows COM APIs.
//
// **IMPORTANT**: This module does NOT:
//   - Initialize IAudioClient
//   - Start / Stop / Reset IAudioClient
//   - Get IAudioRenderClient
//   - GetBuffer / ReleaseBuffer
//   - Produce audible output
//   - Operate outside of explicit opt-in tests

#[cfg(windows)]
pub use client_windows::*;

#[cfg(not(windows))]
pub use client_stub::*;

#[cfg(windows)]
mod client_windows;

#[cfg(not(windows))]
mod client_stub;

/// Environment variable name for client activate smoke opt-in.
pub const WASAPI_CLIENT_ACTIVATE_SMOKE_ENV: &str = "KIVO_WASAPI_CLIENT_ACTIVATE_SMOKE";

/// Report from a client activate smoke probe.
///
/// This is NOT a playback error. It describes the result of probing
/// whether an IAudioClient can be activated from the default audio endpoint.
#[derive(Clone, Debug)]
pub struct WasapiClientActivateSmokeReport {
    /// Platform identifier ("windows" or "non-windows").
    pub platform: &'static str,
    /// Name of the opt-in environment variable.
    pub opt_in_env: &'static str,
    /// Whether the opt-in environment variable was set to "1".
    pub opt_in_enabled: bool,
    /// Whether the smoke probe was actually attempted.
    pub attempted: bool,
    /// Whether the probe was skipped (and thus other fields are meaningless).
    pub skipped: bool,
    /// Reason for skipping, if any.
    pub skipped_reason: Option<&'static str>,
    /// Whether the default endpoint was successfully obtained.
    pub endpoint_available: bool,
    /// Whether IAudioClient was successfully activated.
    pub client_activated: bool,
    /// Whether IAudioClient::Initialize was called (always false).
    pub initialized_audio_client: bool,
    /// Whether IAudioRenderClient was obtained (always false).
    pub render_client_available: bool,
    /// Error message if the probe failed.
    pub error_message: Option<String>,
}

impl WasapiClientActivateSmokeReport {
    /// Create a skipped report for non-Windows platforms.
    pub fn skipped_non_windows() -> Self {
        Self {
            platform: "non-windows",
            opt_in_env: WASAPI_CLIENT_ACTIVATE_SMOKE_ENV,
            opt_in_enabled: false,
            attempted: false,
            skipped: true,
            skipped_reason: Some("unsupported platform"),
            endpoint_available: false,
            client_activated: false,
            initialized_audio_client: false,
            render_client_available: false,
            error_message: None,
        }
    }

    /// Create a skipped report for missing opt-in environment variable.
    pub fn skipped_env_missing() -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_CLIENT_ACTIVATE_SMOKE_ENV,
            opt_in_enabled: false,
            attempted: false,
            skipped: true,
            skipped_reason: Some("set KIVO_WASAPI_CLIENT_ACTIVATE_SMOKE=1 to run"),
            endpoint_available: false,
            client_activated: false,
            initialized_audio_client: false,
            render_client_available: false,
            error_message: None,
        }
    }

    /// Create a skipped report for a failed probe attempt.
    pub fn skipped_with_error(reason: &'static str, error: String) -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_CLIENT_ACTIVATE_SMOKE_ENV,
            opt_in_enabled: true,
            attempted: true,
            skipped: true,
            skipped_reason: Some(reason),
            endpoint_available: false,
            client_activated: false,
            initialized_audio_client: false,
            render_client_available: false,
            error_message: Some(error),
        }
    }

    /// Create a report when endpoint is available but activation failed.
    pub fn endpoint_available_but_activate_failed(error: String) -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_CLIENT_ACTIVATE_SMOKE_ENV,
            opt_in_enabled: true,
            attempted: true,
            skipped: true,
            skipped_reason: Some("audio client activation failed"),
            endpoint_available: true,
            client_activated: false,
            initialized_audio_client: false,
            render_client_available: false,
            error_message: Some(error),
        }
    }

    /// Create a success report when client was activated.
    pub fn success() -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_CLIENT_ACTIVATE_SMOKE_ENV,
            opt_in_enabled: true,
            attempted: true,
            skipped: false,
            skipped_reason: None,
            endpoint_available: true,
            client_activated: true,
            initialized_audio_client: false,
            render_client_available: false,
            error_message: None,
        }
    }
}
