// report_failure_buffer_builders.rs
//
// Failure builders for buffer, prefill, and start steps of the padding query
// smoke probe.
//
// These cover failures from GetBufferSize (zero) through IAudioClient::Start,
// before the GetCurrentPadding call.

use super::format_fields::FormatFields;
use super::report::WasapiPaddingQuerySmokeReport;

impl WasapiPaddingQuerySmokeReport {
    /// Create a report when GetBufferSize returned zero.
    pub fn buffer_size_zero(fields: FormatFields) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("buffer size is zero"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: Some(0),
            error_message: Some("GetBufferSize returned 0".to_string()),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when GetBuffer failed.
    pub fn get_buffer_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        requested_frames: u32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("get buffer failed"),
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
            prefill_requested_frames: Some(requested_frames),
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when ReleaseBuffer failed.
    pub fn release_buffer_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        requested_frames: u32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("release buffer failed"),
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
            prefill_requested_frames: Some(requested_frames),
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when Start failed.
    pub fn start_failed(fields: FormatFields, buffer_size_frames: u32, error: String) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("start failed"),
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
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
