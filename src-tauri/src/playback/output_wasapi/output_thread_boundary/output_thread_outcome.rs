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
///
/// # Field semantics
/// - **Timeout**: thread_report_recv_attempted=true, thread_report_received=false,
///   thread_recv_timed_out=true, thread_join_attempted=false
/// - **Panic**: thread_panic_caught=true, thread_panic_message=Some(...)
/// - **JoinFailed**: thread_join_attempted=true, thread_join_failed=true
/// - **Success**: merge all fields from thread report
pub fn apply_thread_outcome(
    report: &mut WasapiOutputThreadSmokeReport,
    thread_report: ThreadReport,
    timeout_ms: u64,
) {
    report.thread_report_recv_attempted = true;

    match thread_report {
        ThreadReport::Success(success_report) => {
            report.merge_from(*success_report);
        }
        ThreadReport::Timeout => {
            // Timeout: do NOT join (would block indefinitely)
            report.thread_report_received = false;
            report.thread_recv_timed_out = true;
            report.thread_recv_timeout_ms = Some(timeout_ms);
            report.thread_join_attempted = false;
            report.thread_joined = false;
            report.thread_join_failed = false;
            report.thread_panic_caught = false;
            report.error_message = Some("Output thread timed out".to_string());
        }
        ThreadReport::Panic(panic_msg) => {
            // Direct panic report (from catch_unwind before channel send)
            report.thread_panic_caught = true;
            report.thread_panic_message = Some(panic_msg);
            report.error_message = Some("Output thread panicked".to_string());
        }
        ThreadReport::JoinFailed(error) => {
            // Join failed (thread exited without sending report)
            report.thread_join_attempted = true;
            report.thread_joined = false;
            report.thread_join_failed = true;
            report.error_message = Some(format!("Output thread join failed: {error}"));
        }
    }
}
