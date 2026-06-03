// output_thread_boundary/output_thread_outcome.rs
//
// Thread report outcome handling for output thread boundary smoke.
//
// This file contains the logic for merging ThreadReport results
// into the main WasapiOutputThreadSmokeReport.

use super::report::WasapiOutputThreadSmokeReport;
use super::thread_report::ThreadReport;

/// Apply a ThreadReport outcome to the main report.
///
/// Handles success merge, timeout, panic, and join failure cases.
pub fn apply_thread_outcome(
    report: &mut WasapiOutputThreadSmokeReport,
    thread_report: ThreadReport,
    timeout_ms: u64,
) {
    match thread_report {
        ThreadReport::Success(success_report) => {
            report.merge_from(*success_report);
        }
        ThreadReport::Timeout => {
            report.thread_recv_timed_out = true;
            report.thread_recv_timeout_ms = Some(timeout_ms);
            report.error_message = Some("Output thread timed out".to_string());
        }
        ThreadReport::Panic(panic_msg) => {
            report.thread_panic_caught = true;
            report.thread_panic_message = Some(panic_msg);
            report.error_message = Some("Output thread panicked".to_string());
        }
        ThreadReport::JoinFailed(error) => {
            report.thread_join_failed = true;
            report.error_message = Some(format!("Output thread join failed: {error}"));
        }
    }
}
