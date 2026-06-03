// report_failure_padding_builders.rs
//
// Failure builders for padding query and stop steps of the padding query
// smoke probe.
//
// These cover failures from IAudioClient::GetCurrentPadding and
// IAudioClient::Stop, after Start has succeeded.

use super::format_fields::FormatFields;
use super::report::WasapiPaddingQuerySmokeReport;

impl WasapiPaddingQuerySmokeReport {
    /// Create a report when GetCurrentPadding failed.
    pub fn get_current_padding_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("get current padding failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: Some(buffer_size_frames),
            get_buffer_attempted: true,
            buffer_obtained: true,
            release_buffer_attempted: true,
            buffer_released: true,
            prefill_requested_frames: Some(1),
            prefill_released_frames: Some(1),
            prefill_used_silent_flag: true,
            start_attempted: true,
            started_audio_client: true,
            get_current_padding_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when Start succeeded but Stop failed.
    pub fn stop_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        padding_frames: Option<u32>,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("stop failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: Some(buffer_size_frames),
            get_buffer_attempted: true,
            buffer_obtained: true,
            release_buffer_attempted: true,
            buffer_released: true,
            prefill_requested_frames: Some(1),
            prefill_released_frames: Some(1),
            prefill_used_silent_flag: true,
            start_attempted: true,
            started_audio_client: true,
            get_current_padding_attempted: true,
            padding_frames,
            stop_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
