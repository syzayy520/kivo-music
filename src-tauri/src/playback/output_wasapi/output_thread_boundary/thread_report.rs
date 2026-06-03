// output_thread_boundary/thread_report.rs
//
// Thread message carrier for output thread boundary smoke.
//
// This file defines the message type sent from the output thread
// back to the main thread via a channel.

use super::report::WasapiOutputThreadSmokeReport;

/// Message sent from the output thread back to the main thread.
///
/// This is the result of the output thread boundary smoke probe,
/// which can be either a full report, a thread panic, or a timeout.
#[derive(Debug)]
pub enum ThreadReport {
    /// The output thread completed successfully and sent a report.
    Success(Box<WasapiOutputThreadSmokeReport>),
    /// The output thread panicked.
    Panic(String),
    /// The output thread timed out waiting for the report.
    Timeout,
    /// The output thread failed to join.
    JoinFailed(String),
}
