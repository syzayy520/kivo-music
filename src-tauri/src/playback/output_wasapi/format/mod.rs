// format/mod.rs
//
// Format boundary facade for WASAPI IAudioClient GetMixFormat smoke.
//
// This module provides a cross-platform interface to probe whether
// IAudioClient::GetMixFormat can be called and the returned format
// pointer can be safely released via CoTaskMemFree.
//
// **IMPORTANT**: This module does NOT:
//   - Initialize IAudioClient
//   - IsFormatSupported
//   - GetService
//   - Get IAudioRenderClient
//   - GetBuffer / ReleaseBuffer
//   - Start / Stop / Reset
//   - Produce audible output
//   - Operate outside of explicit opt-in tests

#[cfg(windows)]
pub use format_windows::*;

#[cfg(not(windows))]
pub use format_stub::*;

#[cfg(windows)]
mod format_windows;

#[cfg(not(windows))]
mod format_stub;

/// Environment variable name for mix format smoke opt-in.
pub const WASAPI_MIX_FORMAT_SMOKE_ENV: &str = "KIVO_WASAPI_MIX_FORMAT_SMOKE";

/// Report from a mix format smoke probe.
///
/// This is NOT a playback error. It describes the result of probing
/// whether IAudioClient::GetMixFormat can be called and the format
/// pointer can be safely released.
#[derive(Clone, Debug)]
pub struct WasapiMixFormatSmokeReport {
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
    /// Whether IAudioClient::GetMixFormat succeeded.
    pub mix_format_available: bool,
    /// Whether IAudioClient::Initialize was called (always false).
    pub initialized_audio_client: bool,
    /// Whether IAudioClient::IsFormatSupported was called (always false).
    pub is_format_supported_called: bool,
    /// Whether IAudioRenderClient was obtained (always false).
    pub render_client_available: bool,
    /// Sample rate in Hz (from GetMixFormat).
    pub sample_rate_hz: Option<u32>,
    /// Number of channels (from GetMixFormat).
    pub channels: Option<u16>,
    /// Bits per sample (from GetMixFormat).
    pub bits_per_sample: Option<u16>,
    /// Block alignment (from GetMixFormat).
    pub block_align: Option<u16>,
    /// Average bytes per second (from GetMixFormat).
    pub avg_bytes_per_sec: Option<u32>,
    /// Format tag (from GetMixFormat).
    pub format_tag: Option<u16>,
    /// cbSize from WAVEFORMATEX (from GetMixFormat).
    pub cb_size: Option<u16>,
    /// Error message if the probe failed.
    pub error_message: Option<String>,
}

impl WasapiMixFormatSmokeReport {
    /// Create a skipped report for non-Windows platforms.
    pub fn skipped_non_windows() -> Self {
        Self {
            platform: "non-windows",
            opt_in_env: WASAPI_MIX_FORMAT_SMOKE_ENV,
            opt_in_enabled: false,
            attempted: false,
            skipped: true,
            skipped_reason: Some("unsupported platform"),
            endpoint_available: false,
            client_activated: false,
            mix_format_available: false,
            initialized_audio_client: false,
            is_format_supported_called: false,
            render_client_available: false,
            sample_rate_hz: None,
            channels: None,
            bits_per_sample: None,
            block_align: None,
            avg_bytes_per_sec: None,
            format_tag: None,
            cb_size: None,
            error_message: None,
        }
    }

    /// Create a skipped report for missing opt-in environment variable.
    pub fn skipped_env_missing() -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_MIX_FORMAT_SMOKE_ENV,
            opt_in_enabled: false,
            attempted: false,
            skipped: true,
            skipped_reason: Some("set KIVO_WASAPI_MIX_FORMAT_SMOKE=1 to run"),
            endpoint_available: false,
            client_activated: false,
            mix_format_available: false,
            initialized_audio_client: false,
            is_format_supported_called: false,
            render_client_available: false,
            sample_rate_hz: None,
            channels: None,
            bits_per_sample: None,
            block_align: None,
            avg_bytes_per_sec: None,
            format_tag: None,
            cb_size: None,
            error_message: None,
        }
    }

    /// Create a skipped report for a failed probe attempt.
    pub fn skipped_with_error(reason: &'static str, error: String) -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_MIX_FORMAT_SMOKE_ENV,
            opt_in_enabled: true,
            attempted: true,
            skipped: true,
            skipped_reason: Some(reason),
            endpoint_available: false,
            client_activated: false,
            mix_format_available: false,
            initialized_audio_client: false,
            is_format_supported_called: false,
            render_client_available: false,
            sample_rate_hz: None,
            channels: None,
            bits_per_sample: None,
            block_align: None,
            avg_bytes_per_sec: None,
            format_tag: None,
            cb_size: None,
            error_message: Some(error),
        }
    }

    /// Create a report when endpoint is available but activation failed.
    pub fn endpoint_available_but_activate_failed(error: String) -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_MIX_FORMAT_SMOKE_ENV,
            opt_in_enabled: true,
            attempted: true,
            skipped: true,
            skipped_reason: Some("audio client activation failed"),
            endpoint_available: true,
            client_activated: false,
            mix_format_available: false,
            initialized_audio_client: false,
            is_format_supported_called: false,
            render_client_available: false,
            sample_rate_hz: None,
            channels: None,
            bits_per_sample: None,
            block_align: None,
            avg_bytes_per_sec: None,
            format_tag: None,
            cb_size: None,
            error_message: Some(error),
        }
    }

    /// Create a report when client activated but GetMixFormat failed.
    pub fn client_activated_but_mix_format_failed(error: String) -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_MIX_FORMAT_SMOKE_ENV,
            opt_in_enabled: true,
            attempted: true,
            skipped: true,
            skipped_reason: Some("mix format unavailable"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: false,
            initialized_audio_client: false,
            is_format_supported_called: false,
            render_client_available: false,
            sample_rate_hz: None,
            channels: None,
            bits_per_sample: None,
            block_align: None,
            avg_bytes_per_sec: None,
            format_tag: None,
            cb_size: None,
            error_message: Some(error),
        }
    }

    /// Create a success report when GetMixFormat succeeded.
    pub fn success(
        sample_rate_hz: u32,
        channels: u16,
        bits_per_sample: u16,
        block_align: u16,
        avg_bytes_per_sec: u32,
        format_tag: u16,
        cb_size: u16,
    ) -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_MIX_FORMAT_SMOKE_ENV,
            opt_in_enabled: true,
            attempted: true,
            skipped: false,
            skipped_reason: None,
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialized_audio_client: false,
            is_format_supported_called: false,
            render_client_available: false,
            sample_rate_hz: Some(sample_rate_hz),
            channels: Some(channels),
            bits_per_sample: Some(bits_per_sample),
            block_align: Some(block_align),
            avg_bytes_per_sec: Some(avg_bytes_per_sec),
            format_tag: Some(format_tag),
            cb_size: Some(cb_size),
            error_message: None,
        }
    }
}
