// silent_loop/report_failure_loop_builders.rs
//
// Failure builders for the silent loop execution and Stop steps.
//
// These cover failures from GetCurrentPadding (loop iteration),
// loop GetBuffer, loop ReleaseBuffer, and IAudioClient::Stop.

use super::format_fields::FormatFields;
use super::report::WasapiSilentLoopSmokeReport;

impl WasapiSilentLoopSmokeReport {
    /// Create a report when GetCurrentPadding failed during the loop.
    pub fn loop_get_current_padding_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        iteration: u32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("loop get current padding failed"),
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
            loop_iterations_configured: Some(3),
            loop_iterations_completed: Some(iteration),
            small_frame_count: Some(1),
            get_current_padding_attempted: true,
            error_message: Some(error),
            error_iteration: Some(iteration),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when loop GetBuffer failed.
    pub fn loop_get_buffer_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        iteration: u32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("loop get buffer failed"),
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
            loop_iterations_configured: Some(3),
            loop_iterations_completed: Some(iteration),
            small_frame_count: Some(1),
            get_current_padding_attempted: true,
            loop_get_buffer_attempted: true,
            error_message: Some(error),
            error_iteration: Some(iteration),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when loop ReleaseBuffer failed.
    pub fn loop_release_buffer_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        iteration: u32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("loop release buffer failed"),
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
            loop_iterations_configured: Some(3),
            loop_iterations_completed: Some(iteration),
            small_frame_count: Some(1),
            get_current_padding_attempted: true,
            loop_get_buffer_attempted: true,
            loop_get_buffer_success_count: Some(0),
            loop_release_buffer_attempted: true,
            error_message: Some(error),
            error_iteration: Some(iteration),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when Stop failed after the loop.
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
            loop_iterations_configured: Some(3),
            loop_iterations_completed: Some(3),
            small_frame_count: Some(1),
            get_current_padding_attempted: true,
            loop_get_buffer_attempted: true,
            loop_get_buffer_success_count: Some(0),
            loop_release_buffer_attempted: true,
            loop_release_buffer_success_count: Some(0),
            loop_all_releases_silent: true,
            stop_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
