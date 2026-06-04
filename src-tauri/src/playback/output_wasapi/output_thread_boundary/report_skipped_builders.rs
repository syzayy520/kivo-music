// output_thread_boundary/report_skipped_builders.rs
//
// Skipped report builders for WasapiOutputThreadSmokeReport.

use super::report::WasapiOutputThreadSmokeReport;

impl WasapiOutputThreadSmokeReport {
    /// Create a report for successful thread completion.
    pub fn success(
        fields: super::format_fields::FormatFields,
        buffer_size_frames: u32,
        padding_frames: u32,
        thread_duration_ms: u64,
    ) -> Self {
        let mut report = Self {
            start_attempted: true,
            started_audio_client: true,
            get_current_padding_attempted: true,
            current_padding_frames: Some(padding_frames),
            stop_attempted: true,
            stopped_audio_client: true,
            reset_attempted: true,
            reset_succeeded: true,
            thread_duration_ms: Some(thread_duration_ms),
            ..Default::default()
        };
        report.apply_initialized_through_buffer_size(buffer_size_frames);
        report.apply_prefill_released_fields();
        report.apply_format_fields(&fields);
        report
    }
}
