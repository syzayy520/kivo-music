// report_success_builders.rs
//
// Builder methods for successful initialize smoke reports.
//
// These builders handle the case where Initialize succeeded.

use super::format_fields::FormatFields;
use super::report::WasapiClientInitializeSmokeReport;

impl WasapiClientInitializeSmokeReport {
    /// Create a success report when Initialize succeeded.
    pub fn success(fields: FormatFields) -> Self {
        Self {
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
