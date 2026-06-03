// report_failure_start_stop_builders.rs
//
// Builder methods for Start/Stop failure cases in start/stop smoke reports.
//
// These builders handle cases where the smoke probe failed during
// IAudioClient::Start or IAudioClient::Stop operations.

use super::report::WasapiStartStopSmokeReport;

impl WasapiStartStopSmokeReport {
    /// Create a report when Start failed.
    pub fn start_failed(
        fields: super::format_fields::FormatFields,
        buffer_size_frames: u32,
        error: String,
    ) -> Self {
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
            requested_frames: Some(1),
            released_frames: Some(1),
            used_silent_flag: true,
            start_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when Start succeeded but Stop failed.
    pub fn stop_failed(
        fields: super::format_fields::FormatFields,
        buffer_size_frames: u32,
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
            requested_frames: Some(1),
            released_frames: Some(1),
            used_silent_flag: true,
            start_attempted: true,
            started_audio_client: true,
            stop_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
