// report.rs
//
// Report type for WASAPI start/stop smoke boundary.
//
// This file defines WasapiStartStopSmokeReport - the data type
// describing the outcome of a start/stop smoke probe.

/// Environment variable name for start/stop smoke opt-in.
pub const WASAPI_START_STOP_SMOKE_ENV: &str = "KIVO_WASAPI_START_STOP_SMOKE";

/// Report from a start/stop smoke probe.
///
/// This is NOT a playback error. It describes the result of probing
/// whether IAudioClient::Start and Stop can be called after a successful
/// Initialize in shared mode, GetService, GetBuffer(1), and
/// ReleaseBuffer(1, AUDCLNT_BUFFERFLAGS_SILENT).
///
/// **IMPORTANT**: This report does NOT indicate that audio playback is possible.
/// It only indicates that Start and Stop succeeded with silent buffer data.
#[derive(Clone, Debug)]
pub struct WasapiStartStopSmokeReport {
    // Platform and opt-in
    pub platform: &'static str,
    pub opt_in_env: &'static str,
    pub opt_in_enabled: bool,
    pub attempted: bool,
    pub skipped: bool,
    pub skipped_reason: Option<&'static str>,

    // Prerequisite steps
    pub endpoint_available: bool,
    pub client_activated: bool,
    pub mix_format_available: bool,
    pub initialize_attempted: bool,
    pub initialized_audio_client: bool,
    pub get_service_attempted: bool,
    pub render_client_obtained: bool,

    // Buffer size query
    pub get_buffer_size_attempted: bool,
    pub buffer_size_frames: Option<u32>,

    // Buffer operations
    pub get_buffer_attempted: bool,
    pub buffer_obtained: bool,
    pub release_buffer_attempted: bool,
    pub buffer_released: bool,
    pub requested_frames: Option<u32>,
    pub released_frames: Option<u32>,
    pub used_silent_flag: bool,

    // Start/Stop operations
    pub start_attempted: bool,
    pub started_audio_client: bool,
    pub stop_attempted: bool,
    pub stopped_audio_client: bool,

    // Prohibited operations (must all be false)
    pub is_format_supported_called: bool,
    pub get_current_padding_called: bool,
    pub reset_audio_client: bool,
    pub audio_produced: bool,
    pub output_sink_connected: bool,
    pub capability_exposed: bool,

    // Format information (from GetMixFormat)
    pub sample_rate_hz: Option<u32>,
    pub channels: Option<u16>,
    pub bits_per_sample: Option<u16>,
    pub block_align: Option<u16>,
    pub avg_bytes_per_sec: Option<u32>,
    pub format_tag: Option<u16>,
    pub cb_size: Option<u16>,

    // Initialize parameters
    pub share_mode: &'static str,
    pub stream_flags: u32,
    pub buffer_duration_hns: i64,
    pub periodicity_hns: i64,

    // Wait strategy
    pub wait_duration_ms: Option<u64>,

    // Error information
    pub error_message: Option<String>,
}
