// output_thread_boundary/thread_runner.rs
//
// Thread spawn, recv_timeout, and join logic for output thread boundary smoke.
//
// This file contains:
// - spawn_output_thread_probe: spawns a thread to run the output thread boundary probe
// - recv_timeout_and_join: receives the report with timeout and joins the thread
//
// **PROHIBITED** (not implemented here):
//   - Audio playback
//   - Ring buffer / decoder / pipeline
//   - OutputSink / PlaybackCapabilities
//   - Async runtime

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use super::output_thread_windows::probe_output_thread_boundary_internal;
use super::thread_report::ThreadReport;

/// Spawn a thread to run the output thread boundary probe.
///
/// Returns a receiver that will receive the thread's report.
/// The thread will run the probe and send the report via the channel.
pub fn spawn_output_thread_probe() -> mpsc::Receiver<ThreadReport> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // Run the probe and send the result
        let report = probe_output_thread_boundary_internal();
        let _ = tx.send(ThreadReport::Success(Box::new(report)));
    });

    rx
}

/// Receive the report with a timeout and join the thread.
///
/// Returns a `ThreadReport` indicating success, timeout, or join failure.
/// The thread is expected to send a `ThreadReport::Success` on completion.
pub fn recv_timeout_and_join(rx: mpsc::Receiver<ThreadReport>, timeout: Duration) -> ThreadReport {
    let start = Instant::now();

    match rx.recv_timeout(timeout) {
        Ok(report) => {
            // Report received successfully
            let elapsed = start.elapsed();
            match report {
                ThreadReport::Success(mut report) => {
                    report.thread_report_received = true;
                    report.thread_recv_timeout_ms = Some(elapsed.as_millis() as u64);
                    ThreadReport::Success(report) // already boxed
                }
                other => other,
            }
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            // Timeout waiting for report
            ThreadReport::Timeout
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            // Channel disconnected (thread panicked or dropped)
            ThreadReport::JoinFailed("Channel disconnected".to_string())
        }
    }
}
