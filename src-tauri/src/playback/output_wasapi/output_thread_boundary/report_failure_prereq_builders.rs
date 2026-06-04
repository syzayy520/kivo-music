// output_thread_boundary/report_failure_prereq_builders.rs
//
// Prerequisite failure report builders for WasapiOutputThreadSmokeReport.

use super::format_fields::FormatFields;
use super::report::WasapiOutputThreadSmokeReport;

impl WasapiOutputThreadSmokeReport {
    /// Create a report for COM initialization failure.
    pub fn com_init_failed(error: String) -> Self {
        Self {
            attempted: true,
            com_initialized: false,
            error_message: Some(error),
            ..Default::default()
        }
    }

    /// Create a report for endpoint unavailable.
    pub fn endpoint_unavailable(error: String) -> Self {
        Self {
            attempted: true,
            com_initialized: true,
            endpoint_available: false,
            error_message: Some(error),
            ..Default::default()
        }
    }

    /// Create a report for client activation failure.
    pub fn client_activate_failed(error: String) -> Self {
        Self {
            attempted: true,
            com_initialized: true,
            endpoint_available: true,
            client_activated: false,
            error_message: Some(error),
            ..Default::default()
        }
    }

    /// Create a report for mix format failure.
    pub fn mix_format_failed(error: String) -> Self {
        Self {
            attempted: true,
            com_initialized: true,
            endpoint_available: true,
            client_activated: true,
            mix_format_available: false,
            error_message: Some(error),
            ..Default::default()
        }
    }

    /// Create a report for initialize failure.
    pub fn initialize_failed(fields: FormatFields, error: String) -> Self {
        let mut report = Self {
            attempted: true,
            com_initialized: true,
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: false,
            error_message: Some(error),
            ..Default::default()
        };
        report.apply_format_fields(&fields);
        report
    }

    /// Create a report for get service failure.
    pub fn get_service_failed(fields: FormatFields, error: String) -> Self {
        let mut report = Self {
            attempted: true,
            com_initialized: true,
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: false,
            error_message: Some(error),
            ..Default::default()
        };
        report.apply_format_fields(&fields);
        report
    }

    /// Create a report for get buffer size failure.
    pub fn get_buffer_size_failed(fields: FormatFields, error: String) -> Self {
        let mut report = Self {
            attempted: true,
            com_initialized: true,
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: None,
            error_message: Some(error),
            ..Default::default()
        };
        report.apply_format_fields(&fields);
        report
    }

    /// Create a report for buffer size zero.
    pub fn buffer_size_zero(fields: FormatFields) -> Self {
        let mut report = Self {
            attempted: true,
            com_initialized: true,
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: Some(0),
            error_message: Some("buffer size is zero".to_string()),
            ..Default::default()
        };
        report.apply_format_fields(&fields);
        report
    }
}
