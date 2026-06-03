// report.rs
//
// Report type for WASAPI padding query smoke boundary.
//
// This file defines WasapiPaddingQuerySmokeReport - the data type
// describing the outcome of a padding query smoke probe.

/// Environment variable name for padding query smoke opt-in.
pub const WASAPI_PADDING_QUERY_SMOKE_ENV: &str = "KIVO_WASAPI_PADDING_QUERY_SMOKE";

/// Report from a padding query smoke probe.
///
/// This is NOT a playback error. It describes the result of probing
/// whether IAudioClient::GetCurrentPadding can be called after a successful
/// Initialize in shared mode, GetService, GetBuffer(1),
/// ReleaseBuffer(1, AUDCLNT_BUFFERFLAGS_SILENT), and Start.
///
/// **IMPORTANT**: This report does NOT indicate that audio playback is possible.
/// It only indicates that GetCurrentPadding succeeded with silent buffer data.
/// Actual playback requires additional steps (non-silent buffer submission,
/// render loop) that are NOT part of this smoke test.
#[derive(Clone, Debug)]
pub struct WasapiPaddingQuerySmokeReport {
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

    // Buffer operations (prefill)
    pub get_buffer_attempted: bool,
    pub buffer_obtained: bool,
    pub release_buffer_attempted: bool,
    pub buffer_released: bool,
    pub prefill_requested_frames: Option<u32>,
    pub prefill_released_frames: Option<u32>,
    pub prefill_used_silent_flag: bool,

    // Start/Stop operations
    pub start_attempted: bool,
    pub started_audio_client: bool,
    pub stop_attempted: bool,
    pub stopped_audio_client: bool,

    // Padding query operations
    pub get_current_padding_attempted: bool,
    pub padding_frames: Option<u32>,

    // Prohibited operations (must all be false)
    pub is_format_supported_called: bool,
    pub reset_audio_client: bool,
    pub audio_produced: bool,
    pub output_sink_connected: bool,
    pub capability_exposed: bool,
    pub thread_created: bool,
    pub async_runtime_created: bool,
    pub callback_registered: bool,

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

    // Query mode and wait strategy
    pub query_mode: &'static str,
    pub wait_duration_ms: Option<u64>,

    // Error information
    pub error_message: Option<String>,
}
