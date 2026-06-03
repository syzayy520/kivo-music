// report.rs
//
// Report type for WASAPI mix format smoke boundary.
//
// This file defines WasapiMixFormatSmokeReport - the data type
// describing the outcome of a mix format smoke probe.

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
