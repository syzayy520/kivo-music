// ring_buffer_output_thread/output_thread_outcome.rs
//
// Thread report outcome handling for ring buffer output thread smoke.

use super::report::WasapiRingBufferOutputThreadSmokeReport;
use super::thread_report::ThreadReport;

/// Apply a ThreadReport outcome to the main report.
pub fn apply_thread_outcome(
    report: &mut WasapiRingBufferOutputThreadSmokeReport,
    thread_report: ThreadReport,
    timeout_ms: u64,
) {
    report.thread_report_recv_attempted = true;

    match thread_report {
        ThreadReport::Success(success_report) => {
            report.merge_from(*success_report);
        }
        ThreadReport::Timeout => {
            report.thread_report_received = false;
            report.thread_recv_timed_out = true;
            report.thread_recv_timeout_ms = Some(timeout_ms);
            report.output_thread_join_attempted = false;
            report.output_thread_joined = false;
            report.output_thread_join_failed = false;
            report.thread_panic_caught = false;
            report.error_message = Some("Output thread timed out".to_string());
        }
        ThreadReport::Panic(panic_msg) => {
            report.thread_panic_caught = true;
            report.thread_panic_message = Some(panic_msg);
            report.error_message = Some("Output thread panicked".to_string());
        }
        ThreadReport::JoinFailed(error) => {
            report.output_thread_join_attempted = true;
            report.output_thread_joined = false;
            report.output_thread_join_failed = true;
            report.error_message = Some(format!("Output thread join failed: {error}"));
        }
    }
}
