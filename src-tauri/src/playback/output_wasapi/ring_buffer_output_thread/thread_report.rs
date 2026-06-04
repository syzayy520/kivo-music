// ring_buffer_output_thread/thread_report.rs
//
// Thread message carrier for ring buffer output thread smoke.
//
// This file defines the message type sent from the output thread
// back to the main thread via a channel.

use super::report::WasapiRingBufferOutputThreadSmokeReport;

/// Message sent from the output thread back to the main thread.
///
/// This is the result of the ring buffer output thread smoke probe,
/// which can be either a full report, a thread panic, or a timeout.
#[derive(Debug)]
pub enum ThreadReport {
    /// The output thread completed successfully and sent a report.
    Success(Box<WasapiRingBufferOutputThreadSmokeReport>),
    /// The output thread panicked.
    Panic(String),
    /// The output thread timed out waiting for the report.
    Timeout,
    /// The output thread failed to join.
    JoinFailed(String),
}
