// thread_runner_tests.rs
//
// Tests for thread runner logic (recv_timeout_and_join).
// Only compiled on Windows since thread_runner is Windows-only.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::playback::output_wasapi::ring_buffer_output_thread::report::WasapiRingBufferOutputThreadSmokeReport;
use crate::playback::output_wasapi::ring_buffer_output_thread::thread_report::ThreadReport;
use crate::playback::output_wasapi::ring_buffer_output_thread::thread_runner::recv_timeout_and_join;

#[test]
fn recv_timeout_and_join_success() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let report = WasapiRingBufferOutputThreadSmokeReport::default();
        tx.send(ThreadReport::Success(Box::new(report))).unwrap();
    });

    let result = recv_timeout_and_join(rx, handle, Duration::from_secs(1));
    match result {
        ThreadReport::Success(report) => {
            assert!(report.thread_report_received);
            assert!(report.thread_recv_timeout_ms.is_some());
            assert!(report.output_thread_join_attempted);
            assert!(report.output_thread_joined);
            assert!(!report.output_thread_join_failed);
            assert!(!report.thread_panic_caught);
        }
        other => panic!("Expected Success, got {:?}", other),
    }
}

#[test]
fn recv_timeout_and_join_timeout() {
    let (_tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
    });

    let result = recv_timeout_and_join(rx, handle, Duration::from_millis(100));
    match result {
        ThreadReport::Timeout => {}
        other => panic!("Expected Timeout, got {:?}", other),
    }
}

#[test]
fn recv_timeout_and_join_disconnected_join_success() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        drop(tx);
    });

    let result = recv_timeout_and_join(rx, handle, Duration::from_secs(1));
    match result {
        ThreadReport::JoinFailed(msg) => {
            assert!(msg.contains("channel disconnected"));
        }
        other => panic!("Expected JoinFailed, got {:?}", other),
    }
}

#[test]
fn recv_timeout_and_join_disconnected_join_panic() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        drop(tx);
        panic!("test panic without sending report");
    });

    let result = recv_timeout_and_join(rx, handle, Duration::from_secs(1));
    match result {
        ThreadReport::Panic(msg) => {
            assert!(msg.contains("thread panicked without sending report"));
        }
        other => panic!("Expected Panic, got {:?}", other),
    }
}

#[test]
fn recv_timeout_and_join_panic_via_channel() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let _ = tx.send(ThreadReport::Panic("test panic".to_string()));
    });

    let result = recv_timeout_and_join(rx, handle, Duration::from_secs(1));
    match result {
        ThreadReport::Success(report) => {
            assert!(report.thread_report_received);
            assert!(report.thread_panic_caught);
            assert_eq!(report.thread_panic_message, Some("test panic".to_string()));
            assert!(report.output_thread_join_attempted);
            assert!(report.output_thread_joined);
        }
        other => panic!("Expected Success with panic, got {:?}", other),
    }
}
