// output_thread_boundary/report_defaults.rs
//
// Default implementations for WasapiOutputThreadSmokeReport.

use super::report::WasapiOutputThreadSmokeReport;

impl Default for WasapiOutputThreadSmokeReport {
    fn default() -> Self {
        Self {
            platform: std::env::consts::OS,
            opt_in_env: super::env::WASAPI_OUTPUT_THREAD_SMOKE_ENV,
            opt_in_enabled: false,
            attempted: false,
            skipped: false,
            skipped_reason: None,
            error_message: None,
            error_hresult: None,

            thread_spawn_attempted: false,
            thread_spawned: false,
            thread_report_recv_attempted: false,
            thread_report_received: false,
            thread_recv_timeout_ms: None,
            thread_recv_timed_out: false,
            thread_join_attempted: false,
            thread_joined: false,
            thread_join_failed: false,
            thread_panic_caught: false,
            thread_panic_message: None,
            thread_duration_ms: None,

            com_initialized: false,
            endpoint_available: false,
            client_activated: false,
            mix_format_available: false,
            initialize_attempted: false,
            initialized_audio_client: false,
            get_service_attempted: false,
            render_client_obtained: false,
            get_buffer_size_attempted: false,
            buffer_size_frames: None,

            prefill_get_buffer_attempted: false,
            prefill_buffer_obtained: false,
            prefill_release_buffer_attempted: false,
            prefill_buffer_released: false,
            prefill_requested_frames: None,
            prefill_released_frames: None,
            prefill_used_silent_flag: false,

            start_attempted: false,
            started_audio_client: false,
            get_current_padding_attempted: false,
            current_padding_frames: None,
            stop_attempted: false,
            stopped_audio_client: false,
            reset_attempted: false,
            reset_succeeded: false,
            reset_hresult: None,

            output_sink_connected: false,
            capability_exposed: false,
            thread_callback_registered: false,
            async_runtime_created: false,
            ring_buffer_created: false,
            decoder_connected: false,
            pipeline_connected: false,
            manager_connected: false,
            real_pcm_produced: false,
            non_silent_data_written: false,
            audio_produced: false,
            playback_capability_enabled: false,

            sample_rate_hz: None,
            channels: None,
            bits_per_sample: None,
            block_align: None,
            avg_bytes_per_sec: None,
            format_tag: None,
            cb_size: None,

            share_mode: "shared",
            stream_flags: 0,
            buffer_duration_hns: 0,
            periodicity_hns: 0,

            query_mode: "output_thread_boundary",
            wait_duration_ms: Some(0),
        }
    }
}
