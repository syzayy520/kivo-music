// output_thread_boundary_tests/thread_runner_tests.rs
//
// Tests for thread runner logic (recv_timeout_and_join).

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::playback::output_wasapi::output_thread_boundary::report::WasapiOutputThreadSmokeReport;
use crate::playback::output_wasapi::output_thread_boundary::thread_report::ThreadReport;
use crate::playback::output_wasapi::output_thread_boundary::thread_runner::recv_timeout_and_join;

#[test]
fn recv_timeout_and_join_success() {
    let (tx, rx) = mpsc::channel();

    // Spawn a thread that sends a success report and exits
    let handle = thread::spawn(move || {
        let report = WasapiOutputThreadSmokeReport::default();
        tx.send(ThreadReport::Success(Box::new(report))).unwrap();
    });

    let result = recv_timeout_and_join(rx, handle, Duration::from_secs(1));
    match result {
        ThreadReport::Success(report) => {
            assert!(report.thread_report_received);
            assert!(report.thread_recv_timeout_ms.is_some());
            assert!(report.thread_join_attempted);
            assert!(report.thread_joined);
            assert!(!report.thread_join_failed);
            assert!(!report.thread_panic_caught);
        }
        other => panic!("Expected Success, got {:?}", other),
    }
}

#[test]
fn recv_timeout_and_join_timeout() {
    let (_tx, rx) = mpsc::channel();

    // Spawn a thread that does nothing (will be abandoned)
    let handle = thread::spawn(move || {
        // Sleep longer than timeout
        thread::sleep(Duration::from_secs(10));
    });

    // Don't send anything, should timeout
    let result = recv_timeout_and_join(rx, handle, Duration::from_millis(100));
    match result {
        ThreadReport::Timeout => {
            // Expected - no join attempted
        }
        other => panic!("Expected Timeout, got {:?}", other),
    }
}

#[test]
fn recv_timeout_and_join_disconnected_join_success() {
    let (tx, rx) = mpsc::channel();

    // Spawn a thread that drops the sender and exits normally
    let handle = thread::spawn(move || {
        drop(tx);
    });

    let result = recv_timeout_and_join(rx, handle, Duration::from_secs(1));
    match result {
        ThreadReport::JoinFailed(msg) => {
            // Thread exited without sending report
            assert!(msg.contains("channel disconnected"));
        }
        other => panic!("Expected JoinFailed, got {:?}", other),
    }
}

#[test]
fn recv_timeout_and_join_disconnected_join_panic() {
    let (tx, rx) = mpsc::channel();

    // Spawn a thread that drops the sender and panics
    let handle = thread::spawn(move || {
        drop(tx);
        panic!("test panic without sending report");
    });

    let result = recv_timeout_and_join(rx, handle, Duration::from_secs(1));
    match result {
        ThreadReport::Panic(msg) => {
            // Thread panicked without sending report
            assert!(msg.contains("thread panicked without sending report"));
        }
        other => panic!("Expected Panic, got {:?}", other),
    }
}

#[test]
fn recv_timeout_and_join_panic_via_channel() {
    let (tx, rx) = mpsc::channel();

    // Spawn a thread that catches its own panic and sends Panic report
    let handle = thread::spawn(move || {
        // Simulate catch_unwind behavior
        let _ = tx.send(ThreadReport::Panic("test panic".to_string()));
    });

    let result = recv_timeout_and_join(rx, handle, Duration::from_secs(1));
    match result {
        ThreadReport::Success(report) => {
            // Panic via channel is converted to Success with panic fields
            assert!(report.thread_report_received);
            assert!(report.thread_panic_caught);
            assert_eq!(report.thread_panic_message, Some("test panic".to_string()));
            assert!(report.thread_join_attempted);
            assert!(report.thread_joined);
        }
        other => panic!("Expected Success with panic, got {:?}", other),
    }
}
