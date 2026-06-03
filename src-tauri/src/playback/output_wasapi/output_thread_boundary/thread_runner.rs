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

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use super::output_thread_windows::probe_output_thread_boundary_internal;
use super::thread_report::ThreadReport;

/// Spawn a thread to run the output thread boundary probe.
///
/// Returns a receiver that will receive the thread's report and the thread's JoinHandle.
/// The thread will run the probe with catch_unwind and send the report via the channel.
pub fn spawn_output_thread_probe() -> (mpsc::Receiver<ThreadReport>, thread::JoinHandle<()>) {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        // Run the probe with catch_unwind to capture panics
        let result = catch_unwind(AssertUnwindSafe(probe_output_thread_boundary_internal));

        match result {
            Ok(report) => {
                // Success: send the report
                let _ = tx.send(ThreadReport::Success(Box::new(report)));
            }
            Err(payload) => {
                // Panic captured: extract message and send Panic report
                let panic_message = if let Some(s) = payload.downcast_ref::<String>() {
                    s.clone()
                } else if let Some(s) = payload.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "unknown panic".to_string()
                };
                let _ = tx.send(ThreadReport::Panic(panic_message));
            }
        }
    });

    (rx, handle)
}

/// Receive the report with a timeout and optionally join the thread.
///
/// Returns a `ThreadReport` indicating success, timeout, panic, or join failure.
///
/// # Join semantics
/// - **Ok(report)**: Report received. Join handle (thread should be finished or finishing).
/// - **Timeout**: Do NOT join (would block indefinitely). Return Timeout.
/// - **Disconnected**: Join to determine if thread panicked or exited abnormally.
pub fn recv_timeout_and_join(
    rx: mpsc::Receiver<ThreadReport>,
    handle: thread::JoinHandle<()>,
    timeout: Duration,
) -> ThreadReport {
    let start = Instant::now();

    match rx.recv_timeout(timeout) {
        Ok(report) => {
            // Report received successfully
            let elapsed = start.elapsed();
            match report {
                ThreadReport::Success(mut report) => {
                    report.thread_report_received = true;
                    report.thread_recv_timeout_ms = Some(elapsed.as_millis() as u64);
                    report.thread_join_attempted = true;

                    // Join handle - thread should be finished after sending report
                    match handle.join() {
                        Ok(()) => {
                            report.thread_joined = true;
                            report.thread_join_failed = false;
                        }
                        Err(_) => {
                            // Thread panicked after sending report (unlikely but possible)
                            report.thread_joined = false;
                            report.thread_join_failed = true;
                            report.thread_panic_caught = true;
                            report.thread_panic_message =
                                Some("thread panicked after sending report".to_string());
                        }
                    }
                    ThreadReport::Success(report)
                }
                ThreadReport::Panic(panic_message) => {
                    // Thread caught its own panic and sent a Panic report
                    // Still attempt join to clean up
                    let mut report = Box::new(
                        crate::playback::output_wasapi::output_thread_boundary::report::WasapiOutputThreadSmokeReport::default(),
                    );
                    report.thread_report_received = true;
                    report.thread_recv_timeout_ms = Some(elapsed.as_millis() as u64);
                    report.thread_panic_caught = true;
                    report.thread_panic_message = Some(panic_message);
                    report.thread_join_attempted = true;

                    match handle.join() {
                        Ok(()) => {
                            report.thread_joined = true;
                            report.thread_join_failed = false;
                        }
                        Err(_) => {
                            report.thread_joined = false;
                            report.thread_join_failed = true;
                        }
                    }
                    ThreadReport::Success(report)
                }
                other => other,
            }
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            // Timeout waiting for report
            // Do NOT join - would block indefinitely
            ThreadReport::Timeout
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            // Channel disconnected without receiving a report
            // Join to determine if thread panicked or exited abnormally
            match handle.join() {
                Ok(()) => {
                    // Thread exited without sending a report (abnormal but not panic)
                    ThreadReport::JoinFailed(
                        "channel disconnected, thread exited without report".to_string(),
                    )
                }
                Err(_) => {
                    // Thread panicked without sending a report
                    ThreadReport::Panic("thread panicked without sending report".to_string())
                }
            }
        }
    }
}
