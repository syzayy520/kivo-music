// reset_boundary/report_failure_reset_builders.rs
//
// Failure builders for the Reset step of the reset boundary smoke probe.
//
// These cover failures from IAudioClient::Reset after a successful Stop.

use super::format_fields::FormatFields;
use super::report::WasapiResetBoundarySmokeReport;

impl WasapiResetBoundarySmokeReport {
    /// Create a report when Reset failed after successful Stop.
    pub fn reset_failed(
        fields: FormatFields,
        buffer_size_frames: u32,
        padding_frames: u32,
        hresult: i32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("reset failed"),
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
            current_padding_frames: Some(padding_frames),
            stop_attempted: true,
            stopped_audio_client: true,
            reset_attempted: true,
            reset_succeeded: false,
            reset_hresult: Some(hresult),
            error_message: Some(error),
            error_hresult: Some(hresult),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
