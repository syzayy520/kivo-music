// report_skipped_builders.rs
//
// Builder methods for skipped start/stop smoke reports.
//
// These builders handle cases where the smoke probe was skipped
// without attempting Start/Stop (platform unsupported, env missing, early error).

use super::report::WasapiStartStopSmokeReport;

impl WasapiStartStopSmokeReport {
    /// Create a skipped report for non-Windows platforms.
    pub fn skipped_non_windows() -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("unsupported platform"),
            ..Self::base_report_for_non_windows()
        }
    }

    /// Create a skipped report for missing opt-in environment variable.
    pub fn skipped_env_missing() -> Self {
        Self {
            opt_in_enabled: false,
            attempted: false,
            skipped: true,
            skipped_reason: Some("set KIVO_WASAPI_START_STOP_SMOKE=1 to run"),
            ..Self::base_report_for_windows()
        }
    }

    /// Create a skipped report for a failed probe attempt.
    pub fn skipped_with_error(reason: &'static str, error: String) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some(reason),
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
    }
}
