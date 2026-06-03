// output_thread_boundary/report_failure_buffer_builders.rs
//
// Buffer failure report builders for WasapiOutputThreadSmokeReport.

use super::format_fields::FormatFields;
use super::report::WasapiOutputThreadSmokeReport;

impl WasapiOutputThreadSmokeReport {
    /// Create a report for prefill get buffer failure.
    pub fn prefill_get_buffer_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        error: String,
    ) -> Self {
        Self {
            attempted: true,
            com_initialized: true,
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: Some(buffer_size_frames),
            prefill_get_buffer_attempted: true,
            prefill_buffer_obtained: false,
            sample_rate_hz: Some(fields.sample_rate_hz),
            channels: Some(fields.channels),
            bits_per_sample: Some(fields.bits_per_sample),
            block_align: Some(fields.block_align),
            avg_bytes_per_sec: Some(fields.avg_bytes_per_sec),
            format_tag: Some(fields.format_tag),
            cb_size: Some(fields.cb_size),
            error_message: Some(error),
            ..Default::default()
        }
    }

    /// Create a report for prefill release buffer failure.
    pub fn prefill_release_buffer_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        error: String,
    ) -> Self {
        Self {
            attempted: true,
            com_initialized: true,
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: Some(buffer_size_frames),
            prefill_get_buffer_attempted: true,
            prefill_buffer_obtained: true,
            prefill_release_buffer_attempted: true,
            prefill_buffer_released: false,
            prefill_requested_frames: Some(1),
            prefill_used_silent_flag: true,
            sample_rate_hz: Some(fields.sample_rate_hz),
            channels: Some(fields.channels),
            bits_per_sample: Some(fields.bits_per_sample),
            block_align: Some(fields.block_align),
            avg_bytes_per_sec: Some(fields.avg_bytes_per_sec),
            format_tag: Some(fields.format_tag),
            cb_size: Some(fields.cb_size),
            error_message: Some(error),
            ..Default::default()
        }
    }
}
