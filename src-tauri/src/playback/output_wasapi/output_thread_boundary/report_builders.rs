// output_thread_boundary/report_builders.rs
//
// Builder methods and helpers for WasapiOutputThreadSmokeReport.

use super::format_fields::FormatFields;
use super::report::WasapiOutputThreadSmokeReport;

impl WasapiOutputThreadSmokeReport {
    /// Apply format fields from a `FormatFields` struct to the report.
    pub fn apply_format_fields(&mut self, fields: &FormatFields) {
        self.sample_rate_hz = Some(fields.sample_rate_hz);
        self.channels = Some(fields.channels);
        self.bits_per_sample = Some(fields.bits_per_sample);
        self.block_align = Some(fields.block_align);
        self.avg_bytes_per_sec = Some(fields.avg_bytes_per_sec);
        self.format_tag = Some(fields.format_tag);
        self.cb_size = Some(fields.cb_size);
    }

    /// Set fields for the full WASAPI client lifecycle through buffer size.
    ///
    /// Sets: attempted, com_initialized, endpoint_available, client_activated,
    /// mix_format_available, initialize_attempted, initialized_audio_client,
    /// get_service_attempted, render_client_obtained, get_buffer_size_attempted,
    /// buffer_size_frames.
    pub fn apply_initialized_through_buffer_size(&mut self, buffer_size_frames: u32) {
        self.attempted = true;
        self.com_initialized = true;
        self.endpoint_available = true;
        self.client_activated = true;
        self.mix_format_available = true;
        self.initialize_attempted = true;
        self.initialized_audio_client = true;
        self.get_service_attempted = true;
        self.render_client_obtained = true;
        self.get_buffer_size_attempted = true;
        self.buffer_size_frames = Some(buffer_size_frames);
    }

    /// Set fields for full prefill released state (1 frame requested, 1 released, SILENT flag).
    pub fn apply_prefill_released_fields(&mut self) {
        self.prefill_get_buffer_attempted = true;
        self.prefill_buffer_obtained = true;
        self.prefill_release_buffer_attempted = true;
        self.prefill_buffer_released = true;
        self.prefill_requested_frames = Some(1);
        self.prefill_released_frames = Some(1);
        self.prefill_used_silent_flag = true;
    }

    /// Create a skipped report due to missing environment variable.
    pub fn skipped_env_missing() -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("environment variable not set to \"1\""),
            ..Default::default()
        }
    }

    /// Create a skipped report due to non-Windows platform.
    pub fn skipped_non_windows() -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("not a Windows platform"),
            ..Default::default()
        }
    }

    /// Create a skipped report with custom error.
    pub fn skipped_with_error(reason: &'static str, error: String) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some(reason),
            error_message: Some(error),
            ..Default::default()
        }
    }
}
