// output_thread_boundary/report_failure_reset_builders.rs
//
// Reset failure report builders for WasapiOutputThreadSmokeReport.

use super::format_fields::FormatFields;
use super::report::WasapiOutputThreadSmokeReport;

impl WasapiOutputThreadSmokeReport {
    /// Create a report for reset failure.
    pub fn reset_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        padding_frames: u32,
        hresult: i32,
        error: String,
    ) -> Self {
        let mut report = Self {
            start_attempted: true,
            started_audio_client: true,
            get_current_padding_attempted: true,
            current_padding_frames: Some(padding_frames),
            stop_attempted: true,
            stopped_audio_client: true,
            reset_attempted: true,
            reset_succeeded: false,
            reset_hresult: Some(hresult),
            error_message: Some(error),
            error_hresult: Some(hresult),
            ..Default::default()
        };
        report.apply_initialized_through_buffer_size(buffer_size_frames);
        report.apply_prefill_released_fields();
        report.apply_format_fields(&fields);
        report
    }
}
