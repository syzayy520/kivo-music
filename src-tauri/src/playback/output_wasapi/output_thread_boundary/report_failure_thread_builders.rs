// output_thread_boundary/report_failure_thread_builders.rs
//
// Thread failure report builders for WasapiOutputThreadSmokeReport.

use super::format_fields::FormatFields;
use super::report::WasapiOutputThreadSmokeReport;

impl WasapiOutputThreadSmokeReport {
    /// Create a report for start failure.
    pub fn start_failed(fields: FormatFields, buffer_size_frames: u32, error: String) -> Self {
        let mut report = Self {
            start_attempted: true,
            started_audio_client: false,
            error_message: Some(error),
            ..Default::default()
        };
        report.apply_initialized_through_buffer_size(buffer_size_frames);
        report.apply_prefill_released_fields();
        report.apply_format_fields(&fields);
        report
    }

    /// Create a report for get current padding failure.
    pub fn get_current_padding_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        error: String,
    ) -> Self {
        let mut report = Self {
            start_attempted: true,
            started_audio_client: true,
            get_current_padding_attempted: true,
            current_padding_frames: None,
            error_message: Some(error),
            ..Default::default()
        };
        report.apply_initialized_through_buffer_size(buffer_size_frames);
        report.apply_prefill_released_fields();
        report.apply_format_fields(&fields);
        report
    }

    /// Create a report for stop failure.
    pub fn stop_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        padding_frames: u32,
        error: String,
    ) -> Self {
        let mut report = Self {
            start_attempted: true,
            started_audio_client: true,
            get_current_padding_attempted: true,
            current_padding_frames: Some(padding_frames),
            stop_attempted: true,
            stopped_audio_client: false,
            error_message: Some(error),
            ..Default::default()
        };
        report.apply_initialized_through_buffer_size(buffer_size_frames);
        report.apply_prefill_released_fields();
        report.apply_format_fields(&fields);
        report
    }
}
