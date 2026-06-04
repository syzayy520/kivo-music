// ring_buffer_output_thread/thread_runner.rs
//
// Thread spawn, recv_timeout, and join logic for ring buffer output thread smoke.
//
// This file contains:
// - spawn_output_thread_probe: spawns a thread to run the ring buffer output thread probe
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

use super::output_thread_flow::run_ring_buffer_output_thread_flow;
use super::thread_report::ThreadReport;

/// Spawn a thread to run the ring buffer output thread probe.
pub fn spawn_output_thread_probe() -> (mpsc::Receiver<ThreadReport>, thread::JoinHandle<()>) {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let result = catch_unwind(AssertUnwindSafe(run_ring_buffer_output_thread_flow));

        match result {
            Ok(report) => {
                let _ = tx.send(ThreadReport::Success(Box::new(report)));
            }
            Err(payload) => {
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
pub fn recv_timeout_and_join(
    rx: mpsc::Receiver<ThreadReport>,
    handle: thread::JoinHandle<()>,
    timeout: Duration,
) -> ThreadReport {
    let start = Instant::now();

    match rx.recv_timeout(timeout) {
        Ok(report) => {
            let elapsed = start.elapsed();
            match report {
                ThreadReport::Success(mut report) => {
                    report.thread_report_received = true;
                    report.thread_recv_timeout_ms = Some(elapsed.as_millis() as u64);
                    report.output_thread_join_attempted = true;

                    match handle.join() {
                        Ok(()) => {
                            report.output_thread_joined = true;
                            report.output_thread_join_failed = false;
                        }
                        Err(_) => {
                            report.output_thread_joined = false;
                            report.output_thread_join_failed = true;
                            report.thread_panic_caught = true;
                            report.thread_panic_message =
                                Some("thread panicked after sending report".to_string());
                        }
                    }
                    ThreadReport::Success(report)
                }
                ThreadReport::Panic(panic_message) => {
                    let mut report = Box::new(
                        super::report::WasapiRingBufferOutputThreadSmokeReport::default(),
                    );
                    report.thread_report_received = true;
                    report.thread_recv_timeout_ms = Some(elapsed.as_millis() as u64);
                    report.thread_panic_caught = true;
                    report.thread_panic_message = Some(panic_message);
                    report.output_thread_join_attempted = true;

                    match handle.join() {
                        Ok(()) => {
                            report.output_thread_joined = true;
                            report.output_thread_join_failed = false;
                        }
                        Err(_) => {
                            report.output_thread_joined = false;
                            report.output_thread_join_failed = true;
                        }
                    }
                    ThreadReport::Success(report)
                }
                other => other,
            }
        }
        Err(mpsc::RecvTimeoutError::Timeout) => ThreadReport::Timeout,
        Err(mpsc::RecvTimeoutError::Disconnected) => match handle.join() {
            Ok(()) => ThreadReport::JoinFailed(
                "channel disconnected, thread exited without report".to_string(),
            ),
            Err(_) => {
                ThreadReport::Panic("thread panicked without sending report".to_string())
            }
        },
    }
}
