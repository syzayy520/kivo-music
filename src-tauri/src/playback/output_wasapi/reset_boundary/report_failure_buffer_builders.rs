// reset_boundary/report_failure_buffer_builders.rs
//
// Failure builders for buffer, prefill, start, and stop steps
// of the reset boundary smoke probe.
//
// These cover failures from GetBufferSize (zero) through IAudioClient::Stop,
// before the Reset call.

use super::format_fields::FormatFields;
use super::report::WasapiResetBoundarySmokeReport;

impl WasapiResetBoundarySmokeReport {
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

    /// Create a report when prefill GetBuffer failed.
    pub fn prefill_get_buffer_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("prefill get buffer failed"),
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
            prefill_requested_frames: Some(1),
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when prefill ReleaseBuffer failed.
    pub fn prefill_release_buffer_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("prefill release buffer failed"),
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
            prefill_requested_frames: Some(1),
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
            prefill_get_buffer_attempted: true,
            prefill_buffer_obtained: true,
            prefill_release_buffer_attempted: true,
            prefill_buffer_released: true,
            prefill_requested_frames: Some(1),
            prefill_released_frames: Some(1),
            prefill_used_silent_flag: true,
            start_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

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
            prefill_get_buffer_attempted: true,
            prefill_buffer_obtained: true,
            prefill_release_buffer_attempted: true,
            prefill_buffer_released: true,
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

    /// Create a report when Stop failed.
    pub fn stop_failed(fields: FormatFields, buffer_size_frames: u32, error: String) -> Self {
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
            prefill_get_buffer_attempted: true,
            prefill_buffer_obtained: true,
            prefill_release_buffer_attempted: true,
            prefill_buffer_released: true,
            prefill_requested_frames: Some(1),
            prefill_released_frames: Some(1),
            prefill_used_silent_flag: true,
            start_attempted: true,
            started_audio_client: true,
            get_current_padding_attempted: true,
            stop_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
