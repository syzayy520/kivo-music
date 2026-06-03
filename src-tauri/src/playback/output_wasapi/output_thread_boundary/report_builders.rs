// output_thread_boundary/report_builders.rs
//
// Builder methods for WasapiOutputThreadSmokeReport.

use super::report::WasapiOutputThreadSmokeReport;

impl WasapiOutputThreadSmokeReport {
    /// Create a skipped report due to missing environment variable.
    pub fn skipped_env_missing() -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("environment variable not set to \"1\""),
            ..Default::default()
        }
    }

    /// Create a skipped report due to non-Windows platform.
    pub fn skipped_non_windows() -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("not a Windows platform"),
            ..Default::default()
        }
    }

    /// Create a skipped report with custom error.
    pub fn skipped_with_error(reason: &'static str, error: String) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some(reason),
            error_message: Some(error),
            ..Default::default()
        }
    }
}
