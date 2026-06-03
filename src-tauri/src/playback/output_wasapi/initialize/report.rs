// report.rs
//
// Report type for WASAPI initialize smoke boundary.
//
// This file defines WasapiClientInitializeSmokeReport - the data type
// describing the outcome of an initialize smoke probe.

/// Environment variable name for initialize smoke opt-in.
pub const WASAPI_CLIENT_INIT_SMOKE_ENV: &str = "KIVO_WASAPI_CLIENT_INIT_SMOKE";

/// Report from an initialize smoke probe.
///
/// This is NOT a playback error. It describes the result of probing
/// whether IAudioClient::Initialize can be called in shared mode.
///
/// **IMPORTANT**: This report does NOT indicate that audio playback is possible.
/// It only indicates that the Initialize call succeeded. Actual playback
/// requires additional steps (GetService, GetBuffer, Start) that are NOT
/// part of this smoke test.
#[derive(Clone, Debug)]
pub struct WasapiClientInitializeSmokeReport {
    // Platform and opt-in
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

    // Prerequisite steps
    /// Whether the default endpoint was successfully obtained.
    pub endpoint_available: bool,
    /// Whether IAudioClient was successfully activated.
    pub client_activated: bool,
    /// Whether IAudioClient::GetMixFormat succeeded.
    pub mix_format_available: bool,

    // Initialize status
    /// Whether IAudioClient::Initialize was actually called.
    pub initialize_attempted: bool,
    /// Whether IAudioClient::Initialize succeeded (returned S_OK).
    pub initialized_audio_client: bool,

    // Prohibited operations (must all be false)
    /// Whether IAudioClient::IsFormatSupported was called (always false).
    pub is_format_supported_called: bool,
    /// Whether IAudioRenderClient was obtained (always false).
    pub render_client_available: bool,
    /// Whether IAudioClient::GetService was called (always false).
    pub service_requested: bool,
    /// Whether GetBuffer was called (always false).
    pub buffer_requested: bool,
    /// Whether IAudioClient::Start was called (always false).
    pub started_audio_client: bool,
    /// Whether IAudioClient::Stop was called (always false).
    pub stopped_audio_client: bool,
    /// Whether IAudioClient::Reset was called (always false).
    pub reset_audio_client: bool,

    // Format information (from GetMixFormat)
    /// Sample rate in Hz.
    pub sample_rate_hz: Option<u32>,
    /// Number of channels.
    pub channels: Option<u16>,
    /// Bits per sample.
    pub bits_per_sample: Option<u16>,
    /// Block alignment.
    pub block_align: Option<u16>,
    /// Average bytes per second.
    pub avg_bytes_per_sec: Option<u32>,
    /// Format tag.
    pub format_tag: Option<u16>,
    /// cbSize from WAVEFORMATEX.
    pub cb_size: Option<u16>,

    // Initialize parameters
    /// Share mode used ("shared").
    pub share_mode: &'static str,
    /// Stream flags used (0).
    pub stream_flags: u32,
    /// Buffer duration in hundred-nanoseconds (0 = default).
    pub buffer_duration_hns: i64,
    /// Periodicity in hundred-nanoseconds (0 = default).
    pub periodicity_hns: i64,

    // Error information
    /// Error message if the probe failed.
    pub error_message: Option<String>,
}
