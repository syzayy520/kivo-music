// reset_boundary/report_defaults.rs
//
// Default constructors for WasapiResetBoundarySmokeReport.
//
// This file provides base report constructors with safe defaults
// for Windows and non-Windows platforms.

use super::format_fields::FormatFields;
use super::report::WasapiResetBoundarySmokeReport;

impl WasapiResetBoundarySmokeReport {
    /// Create a base report for Windows with safe defaults.
    ///
    /// All prohibited fields are false. All optional fields are None.
    /// query_mode is "reset". wait_duration_ms is 0.
    pub fn base_report_for_windows() -> Self {
        Self {
            platform: "windows",
            opt_in_env: super::report::WASAPI_RESET_SMOKE_ENV,
            opt_in_enabled: false,
            attempted: false,
            skipped: true,
            skipped_reason: None,

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
            stop_attempted: false,
            stopped_audio_client: false,

            get_current_padding_attempted: false,
            current_padding_frames: None,

            reset_attempted: false,
            reset_succeeded: false,
            reset_hresult: None,

            is_format_supported_called: false,
            loop_executed: false,
            audio_produced: false,
            output_sink_connected: false,
            capability_exposed: false,
            thread_created: false,
            async_runtime_created: false,
            callback_registered: false,
            ring_buffer_created: false,
            decoder_connected: false,
            pipeline_connected: false,

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

            query_mode: "reset",
            wait_duration_ms: Some(0),

            error_message: None,
            error_hresult: None,
        }
    }

    /// Create a base report for non-Windows platforms.
    ///
    /// All fields are safe defaults. skipped = true.
    pub fn base_report_for_non_windows() -> Self {
        Self {
            platform: "non-windows",
            ..Self::base_report_for_windows()
        }
    }

    /// Update format fields in the report.
    pub fn with_format_fields(mut self, fields: FormatFields) -> Self {
        self.sample_rate_hz = Some(fields.sample_rate_hz);
        self.channels = Some(fields.channels);
        self.bits_per_sample = Some(fields.bits_per_sample);
        self.block_align = Some(fields.block_align);
        self.avg_bytes_per_sec = Some(fields.avg_bytes_per_sec);
        self.format_tag = Some(fields.format_tag);
        self.cb_size = Some(fields.cb_size);
        self
    }
}
