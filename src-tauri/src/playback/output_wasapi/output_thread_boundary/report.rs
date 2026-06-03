// output_thread_boundary/report.rs
//
// Report type for WASAPI output thread boundary smoke.

/// Report from an output thread boundary smoke probe.
///
/// This is NOT a playback error. It describes the result of probing
/// whether a WASAPI output thread can safely execute the minimal
/// lifecycle: COM init → endpoint → activate → GetMixFormat → Initialize
/// → GetService → GetBufferSize → GetBuffer(1) → ReleaseBuffer(1, SILENT)
/// → Start → GetCurrentPadding → Stop → Reset, and return a report via channel.
///
/// **IMPORTANT**: This report does NOT indicate that audio playback is possible.
/// It only indicates that the output thread boundary is safe.
#[derive(Clone, Debug)]
pub struct WasapiOutputThreadSmokeReport {
    // Platform and opt-in
    pub platform: &'static str,
    pub opt_in_env: &'static str,
    pub opt_in_enabled: bool,
    pub attempted: bool,
    pub skipped: bool,
    pub skipped_reason: Option<&'static str>,
    pub error_message: Option<String>,
    pub error_hresult: Option<i32>,

    // Thread lifecycle
    pub thread_spawn_attempted: bool,
    pub thread_spawned: bool,
    pub thread_report_recv_attempted: bool,
    pub thread_report_received: bool,
    pub thread_recv_timeout_ms: Option<u64>,
    pub thread_recv_timed_out: bool,
    pub thread_join_attempted: bool,
    pub thread_joined: bool,
    pub thread_join_failed: bool,
    pub thread_panic_caught: bool,
    pub thread_panic_message: Option<String>,
    pub thread_duration_ms: Option<u64>,

    // Thread internal WASAPI fields
    pub com_initialized: bool,
    pub endpoint_available: bool,
    pub client_activated: bool,
    pub mix_format_available: bool,
    pub initialize_attempted: bool,
    pub initialized_audio_client: bool,
    pub get_service_attempted: bool,
    pub render_client_obtained: bool,
    pub get_buffer_size_attempted: bool,
    pub buffer_size_frames: Option<u32>,

    // Prefill fields
    pub prefill_get_buffer_attempted: bool,
    pub prefill_buffer_obtained: bool,
    pub prefill_release_buffer_attempted: bool,
    pub prefill_buffer_released: bool,
    pub prefill_requested_frames: Option<u32>,
    pub prefill_released_frames: Option<u32>,
    pub prefill_used_silent_flag: bool,

    // Start/Stop/Reset fields
    pub start_attempted: bool,
    pub started_audio_client: bool,
    pub get_current_padding_attempted: bool,
    pub current_padding_frames: Option<u32>,
    pub stop_attempted: bool,
    pub stopped_audio_client: bool,
    pub reset_attempted: bool,
    pub reset_succeeded: bool,
    pub reset_hresult: Option<i32>,

    // Prohibited operations (must all be false)
    pub output_sink_connected: bool,
    pub capability_exposed: bool,
    pub thread_callback_registered: bool,
    pub async_runtime_created: bool,
    pub ring_buffer_created: bool,
    pub decoder_connected: bool,
    pub pipeline_connected: bool,
    pub manager_connected: bool,
    pub real_pcm_produced: bool,
    pub non_silent_data_written: bool,
    pub audio_produced: bool,
    pub playback_capability_enabled: bool,

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
}

impl WasapiOutputThreadSmokeReport {
    /// Merge fields from another report into this one.
    ///
    /// This is used when the output thread sends a report back via channel,
    /// and we need to merge it into the main report.
    pub fn merge_from(&mut self, other: WasapiOutputThreadSmokeReport) {
        // Platform and opt-in (keep self values)
        // Thread lifecycle (merge specific fields)
        self.thread_report_recv_attempted = true;
        self.thread_report_received = other.thread_report_received;
        self.thread_recv_timeout_ms = other.thread_recv_timeout_ms;
        self.thread_recv_timed_out = other.thread_recv_timed_out;
        self.thread_join_attempted = other.thread_join_attempted;
        self.thread_joined = other.thread_joined;
        self.thread_join_failed = other.thread_join_failed;
        self.thread_panic_caught = other.thread_panic_caught;
        self.thread_panic_message = other.thread_panic_message;
        self.thread_duration_ms = other.thread_duration_ms;

        // Thread internal WASAPI fields (copy from other)
        self.com_initialized = other.com_initialized;
        self.endpoint_available = other.endpoint_available;
        self.client_activated = other.client_activated;
        self.mix_format_available = other.mix_format_available;
        self.initialize_attempted = other.initialize_attempted;
        self.initialized_audio_client = other.initialized_audio_client;
        self.get_service_attempted = other.get_service_attempted;
        self.render_client_obtained = other.render_client_obtained;
        self.get_buffer_size_attempted = other.get_buffer_size_attempted;
        self.buffer_size_frames = other.buffer_size_frames;

        // Prefill fields (copy from other)
        self.prefill_get_buffer_attempted = other.prefill_get_buffer_attempted;
        self.prefill_buffer_obtained = other.prefill_buffer_obtained;
        self.prefill_release_buffer_attempted = other.prefill_release_buffer_attempted;
        self.prefill_buffer_released = other.prefill_buffer_released;
        self.prefill_requested_frames = other.prefill_requested_frames;
        self.prefill_released_frames = other.prefill_released_frames;
        self.prefill_used_silent_flag = other.prefill_used_silent_flag;

        // Start/Stop/Reset fields (copy from other)
        self.start_attempted = other.start_attempted;
        self.started_audio_client = other.started_audio_client;
        self.get_current_padding_attempted = other.get_current_padding_attempted;
        self.current_padding_frames = other.current_padding_frames;
        self.stop_attempted = other.stop_attempted;
        self.stopped_audio_client = other.stopped_audio_client;
        self.reset_attempted = other.reset_attempted;
        self.reset_succeeded = other.reset_succeeded;
        self.reset_hresult = other.reset_hresult;

        // Prohibited operations (copy from other)
        self.output_sink_connected = other.output_sink_connected;
        self.capability_exposed = other.capability_exposed;
        self.thread_callback_registered = other.thread_callback_registered;
        self.async_runtime_created = other.async_runtime_created;
        self.ring_buffer_created = other.ring_buffer_created;
        self.decoder_connected = other.decoder_connected;
        self.pipeline_connected = other.pipeline_connected;
        self.manager_connected = other.manager_connected;
        self.real_pcm_produced = other.real_pcm_produced;
        self.non_silent_data_written = other.non_silent_data_written;
        self.audio_produced = other.audio_produced;
        self.playback_capability_enabled = other.playback_capability_enabled;

        // Format information (copy from other)
        self.sample_rate_hz = other.sample_rate_hz;
        self.channels = other.channels;
        self.bits_per_sample = other.bits_per_sample;
        self.block_align = other.block_align;
        self.avg_bytes_per_sec = other.avg_bytes_per_sec;
        self.format_tag = other.format_tag;
        self.cb_size = other.cb_size;

        // Initialize parameters (copy from other)
        self.share_mode = other.share_mode;
        self.stream_flags = other.stream_flags;
        self.buffer_duration_hns = other.buffer_duration_hns;
        self.periodicity_hns = other.periodicity_hns;

        // Query mode and wait strategy (copy from other)
        self.query_mode = other.query_mode;
        self.wait_duration_ms = other.wait_duration_ms;

        // Error fields (copy from other if present)
        if other.error_message.is_some() {
            self.error_message = other.error_message;
        }
        if other.error_hresult.is_some() {
            self.error_hresult = other.error_hresult;
        }
        if other.skipped_reason.is_some() {
            self.skipped_reason = other.skipped_reason;
        }
        self.skipped = other.skipped;
        self.attempted = other.attempted;
    }
}
