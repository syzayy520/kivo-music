// report_success_builders.rs
//
// Builder methods for successful render client smoke reports.
//
// These builders handle the case where GetService succeeded.

use super::format_fields::FormatFields;
use super::report::WasapiRenderClientSmokeReport;

impl WasapiRenderClientSmokeReport {
    /// Create a success report when GetService(IAudioRenderClient) succeeded.
    pub fn success(fields: FormatFields) -> Self {
        Self {
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
