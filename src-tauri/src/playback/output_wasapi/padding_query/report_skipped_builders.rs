// report_skipped_builders.rs
//
// Builder methods for skipped padding query smoke reports.
//
// These builders handle cases where the smoke probe was skipped
// before any WASAPI operations were attempted.

use super::report::WasapiPaddingQuerySmokeReport;

impl WasapiPaddingQuerySmokeReport {
    /// Create a report for non-Windows platforms (skipped).
    pub fn skipped_non_windows() -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("non-windows platform"),
            ..Self::base_report_for_non_windows()
        }
    }

    /// Create a report when opt-in environment variable is missing (skipped).
    pub fn skipped_env_missing() -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("opt-in environment variable not set"),
            ..Self::base_report_for_windows()
        }
    }

    /// Create a report when an error occurred during setup (skipped).
    pub fn skipped_with_error(reason: &'static str, error: String) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some(reason),
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
    }
}
