// report_failure_prereq_builders.rs
//
// Failure builders for prerequisite steps of the padding query smoke probe.
//
// These cover failures from endpoint activation through GetBufferSize,
// before any buffer or start/stop operations.

use super::format_fields::FormatFields;
use super::report::WasapiPaddingQuerySmokeReport;

impl WasapiPaddingQuerySmokeReport {
    /// Create a report when endpoint is available but activation failed.
    pub fn endpoint_available_but_activate_failed(error: String) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("audio client activation failed"),
            endpoint_available: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
    }

    /// Create a report when client activated but GetMixFormat failed.
    pub fn client_activated_but_mix_format_failed(error: String) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("mix format unavailable"),
            endpoint_available: true,
            client_activated: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
    }

    /// Create a report when mix format obtained but Initialize failed.
    pub fn mix_format_obtained_but_initialize_failed(fields: FormatFields, error: String) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("initialize failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when Initialize succeeded but GetService failed.
    pub fn initialized_but_get_service_failed(fields: FormatFields, error: String) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("get service failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when GetService succeeded but GetBufferSize failed.
    pub fn render_client_obtained_but_get_buffer_size_failed(
        fields: FormatFields,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("get buffer size failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
