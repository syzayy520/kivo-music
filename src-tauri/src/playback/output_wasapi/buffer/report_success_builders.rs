// report_success_builders.rs
//
// Builder methods for successful buffer smoke reports.
//
// These builders handle the case where GetBuffer and ReleaseBuffer succeeded.

use super::format_fields::FormatFields;
use super::report::WasapiBufferSmokeReport;

impl WasapiBufferSmokeReport {
    /// Create a success report when GetBuffer and ReleaseBuffer succeeded.
    pub fn success(fields: FormatFields, buffer_size_frames: u32, requested_frames: u32) -> Self {
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
            requested_frames: Some(requested_frames),
            released_frames: Some(requested_frames),
            used_silent_flag: true,
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
