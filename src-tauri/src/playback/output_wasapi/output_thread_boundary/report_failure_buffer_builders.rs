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
        let mut report = Self {
            prefill_get_buffer_attempted: true,
            prefill_buffer_obtained: false,
            error_message: Some(error),
            ..Default::default()
        };
        report.apply_initialized_through_buffer_size(buffer_size_frames);
        report.apply_format_fields(&fields);
        report
    }

    /// Create a report for prefill release buffer failure.
    pub fn prefill_release_buffer_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        error: String,
    ) -> Self {
        let mut report = Self {
            prefill_get_buffer_attempted: true,
            prefill_buffer_obtained: true,
            prefill_release_buffer_attempted: true,
            prefill_buffer_released: false,
            prefill_requested_frames: Some(1),
            prefill_used_silent_flag: true,
            error_message: Some(error),
            ..Default::default()
        };
        report.apply_initialized_through_buffer_size(buffer_size_frames);
        report.apply_format_fields(&fields);
        report
    }
}
