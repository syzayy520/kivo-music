// ring_buffer_output_thread/report_defaults.rs
//
// Default implementation for WasapiRingBufferOutputThreadSmokeReport.

use super::report::WasapiRingBufferOutputThreadSmokeReport;

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
            thread_report_recv_attempted: false,
            thread_report_received: false,
            thread_recv_timeout_ms: None,
            output_thread_join_attempted: false,
            output_thread_joined: false,
            output_thread_join_failed: false,
            thread_panic_caught: false,
            thread_panic_message: None,
            thread_recv_timed_out: false,
            thread_duration_ms: None,

            ring_buffer_created: false,
            ring_buffer_capacity_frames: None,
            ring_buffer_available_frames: None,
            ring_buffer_closed: false,
            ring_buffer_underrun_count: 0,
            ring_buffer_overrun_count: 0,
            total_frames_written: 0,
            total_frames_read: 0,
            total_silence_frames_filled: 0,

            com_initialized: false,
            endpoint_available: false,
            client_activated: false,
            mix_format_available: false,
            sample_rate_hz: None,
            channels: None,
            bits_per_sample: None,
            block_align: None,
            initialize_attempted: false,
            initialized_audio_client: false,
            get_service_attempted: false,
            render_client_obtained: false,
            get_buffer_size_attempted: false,
            buffer_size_frames: None,
            wasapi_buffer_obtained: false,
            wasapi_buffer_released: false,
            used_silent_flag: false,
            start_attempted: false,
            started_audio_client: false,
            get_current_padding_attempted: false,
            current_padding_frames: None,
            stop_attempted: false,
            stopped_audio_client: false,
            reset_attempted: false,
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
