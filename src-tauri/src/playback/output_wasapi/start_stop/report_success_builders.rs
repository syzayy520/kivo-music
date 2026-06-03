// report_success_builders.rs
//
// Builder methods for successful start/stop smoke reports.
//
// These builders handle the case where Start and Stop succeeded.

use super::format_fields::FormatFields;
use super::report::WasapiStartStopSmokeReport;

impl WasapiStartStopSmokeReport {
    /// Create a success report when Start and Stop succeeded.
    pub fn success(fields: FormatFields, buffer_size_frames: u32) -> Self {
        Self {
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
            stopped_audio_client: true,
            wait_duration_ms: Some(0),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
