/// Report for ring buffer output thread smoke boundary.
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

    // Thread lifecycle placeholder
    pub output_thread_spawn_attempted: bool,
    pub output_thread_spawned: bool,
    pub output_thread_join_attempted: bool,
    pub output_thread_joined: bool,
    pub output_thread_join_failed: bool,
    pub thread_panic_caught: bool,
    pub thread_recv_timed_out: bool,

    // Ring buffer placeholder
    pub ring_buffer_created: bool,
    pub ring_buffer_capacity_frames: Option<u32>,
    pub ring_buffer_available_frames: Option<u32>,
    pub ring_buffer_closed: bool,
    pub ring_buffer_underrun_count: u64,
    pub ring_buffer_overrun_count: u64,
    pub total_frames_written: u64,
    pub total_frames_read: u64,
    pub total_silence_frames_filled: u64,

    // WASAPI lifecycle placeholder
    pub mix_format_available: bool,
    pub buffer_size_frames: Option<u32>,
    pub wasapi_buffer_obtained: bool,
    pub wasapi_buffer_released: bool,
    pub used_silent_flag: bool,
    pub started_audio_client: bool,
    pub stopped_audio_client: bool,
    pub reset_succeeded: bool,

    // Prohibited
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

impl Default for WasapiRingBufferOutputThreadSmokeReport {
    fn default() -> Self {
        Self {
            platform: std::env::consts::OS,
            opt_in_env: super::env::RING_BUFFER_OUTPUT_THREAD_SMOKE_ENV,
            opt_in_enabled: false,
            attempted: false,
            skipped: false,
            skipped_reason: None,
            error_message: None,

            output_thread_spawn_attempted: false,
            output_thread_spawned: false,
            output_thread_join_attempted: false,
            output_thread_joined: false,
            output_thread_join_failed: false,
            thread_panic_caught: false,
            thread_recv_timed_out: false,

            ring_buffer_created: false,
            ring_buffer_capacity_frames: None,
            ring_buffer_available_frames: None,
            ring_buffer_closed: false,
            ring_buffer_underrun_count: 0,
            ring_buffer_overrun_count: 0,
            total_frames_written: 0,
            total_frames_read: 0,
            total_silence_frames_filled: 0,

            mix_format_available: false,
            buffer_size_frames: None,
            wasapi_buffer_obtained: false,
            wasapi_buffer_released: false,
            used_silent_flag: false,
            started_audio_client: false,
            stopped_audio_client: false,
            reset_succeeded: false,

            output_sink_connected: false,
            capability_exposed: false,
            decoder_connected: false,
            pipeline_connected: false,
            manager_connected: false,
            real_pcm_produced: false,
            non_silent_data_written: false,
            audio_produced: false,
            playback_capability_enabled: false,
        }
    }
}
