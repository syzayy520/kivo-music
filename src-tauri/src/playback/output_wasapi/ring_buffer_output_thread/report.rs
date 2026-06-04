// ring_buffer_output_thread/report.rs
//
// Report type for ring buffer output thread smoke.

/// Report from a ring buffer output thread smoke probe.
///
/// This is NOT a playback error. It describes the result of probing
/// whether a WASAPI output thread can safely execute the minimal
/// lifecycle with a ring buffer: COM init → endpoint → activate → GetMixFormat
/// → Initialize → GetService → GetBufferSize → Create RingBuffer
/// → read_frames_or_silence → GetBuffer → ReleaseBuffer(SILENT)
/// → Start → GetCurrentPadding → Stop → Reset, and return a report via channel.
///
/// **IMPORTANT**: This report does NOT indicate that audio playback is possible.
/// It only indicates that the ring buffer output thread boundary is safe.
#[derive(Debug, Clone)]
pub struct WasapiRingBufferOutputThreadSmokeReport {
    // Platform/env
    pub platform: &'static str,
    pub opt_in_env: &'static str,
    pub opt_in_enabled: bool,
    pub attempted: bool,
    pub skipped: bool,
    pub skipped_reason: Option<&'static str>,
    pub error_message: Option<String>,

    // Thread lifecycle
    pub output_thread_spawn_attempted: bool,
    pub output_thread_spawned: bool,
    pub thread_report_recv_attempted: bool,
    pub thread_report_received: bool,
    pub thread_recv_timeout_ms: Option<u64>,
    pub output_thread_join_attempted: bool,
    pub output_thread_joined: bool,
    pub output_thread_join_failed: bool,
    pub thread_panic_caught: bool,
    pub thread_panic_message: Option<String>,
    pub thread_recv_timed_out: bool,
    pub thread_duration_ms: Option<u64>,

    // Ring buffer
    pub ring_buffer_created: bool,
    pub ring_buffer_capacity_frames: Option<u32>,
    pub ring_buffer_available_frames: Option<u32>,
    pub ring_buffer_closed: bool,
    pub ring_buffer_underrun_count: u64,
    pub ring_buffer_overrun_count: u64,
    pub total_frames_written: u64,
    pub total_frames_read: u64,
    pub total_silence_frames_filled: u64,

    // WASAPI lifecycle
    pub com_initialized: bool,
    pub endpoint_available: bool,
    pub client_activated: bool,
    pub mix_format_available: bool,
    pub sample_rate_hz: Option<u32>,
    pub channels: Option<u16>,
    pub bits_per_sample: Option<u16>,
    pub block_align: Option<u16>,
    pub initialize_attempted: bool,
    pub initialized_audio_client: bool,
    pub get_service_attempted: bool,
    pub render_client_obtained: bool,
    pub get_buffer_size_attempted: bool,
    pub buffer_size_frames: Option<u32>,
    pub wasapi_buffer_obtained: bool,
    pub wasapi_buffer_released: bool,
    pub used_silent_flag: bool,
    pub start_attempted: bool,
    pub started_audio_client: bool,
    pub get_current_padding_attempted: bool,
    pub current_padding_frames: Option<u32>,
    pub stop_attempted: bool,
    pub stopped_audio_client: bool,
    pub reset_attempted: bool,
    pub reset_succeeded: bool,

    // Prohibited (must all be false)
    pub output_sink_connected: bool,
    pub capability_exposed: bool,
    pub decoder_connected: bool,
    pub pipeline_connected: bool,
    pub manager_connected: bool,
    pub real_pcm_produced: bool,
    pub non_silent_data_written: bool,
    pub audio_produced: bool,
    pub playback_capability_enabled: bool,
}
