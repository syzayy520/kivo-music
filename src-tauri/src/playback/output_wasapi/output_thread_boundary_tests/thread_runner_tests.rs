// output_thread_boundary_tests/thread_runner_tests.rs
//
// Tests for thread runner logic (recv_timeout_and_join).

use std::sync::mpsc;
use std::time::Duration;

use crate::playback::output_wasapi::output_thread_boundary::report::WasapiOutputThreadSmokeReport;
use crate::playback::output_wasapi::output_thread_boundary::thread_report::ThreadReport;
use crate::playback::output_wasapi::output_thread_boundary::thread_runner::recv_timeout_and_join;

#[test]
fn recv_timeout_and_join_success() {
    let (tx, rx) = mpsc::channel();

    // Send a success report
    let report = WasapiOutputThreadSmokeReport::default();
    tx.send(ThreadReport::Success(Box::new(report))).unwrap();

    let result = recv_timeout_and_join(rx, Duration::from_secs(1));
    match result {
        ThreadReport::Success(report) => {
            assert!(report.thread_report_received);
            assert!(report.thread_recv_timeout_ms.is_some());
        }
        other => panic!("Expected Success, got {:?}", other),
    }
}

#[test]
fn recv_timeout_and_join_timeout() {
    let (_tx, rx) = mpsc::channel();

    // Don't send anything, should timeout
    let result = recv_timeout_and_join(rx, Duration::from_millis(100));
    match result {
        ThreadReport::Timeout => {
            // Expected
        }
        other => panic!("Expected Timeout, got {:?}", other),
    }
}

#[test]
fn recv_timeout_and_join_disconnected() {
    let (tx, rx) = mpsc::channel();

    // Drop the sender to disconnect the channel
    drop(tx);

    let result = recv_timeout_and_join(rx, Duration::from_secs(1));
    match result {
        ThreadReport::JoinFailed(_) => {
            // Expected
        }
        other => panic!("Expected JoinFailed, got {:?}", other),
    }
}

#[test]
fn recv_timeout_and_join_panic() {
    let (tx, rx) = mpsc::channel();

    // Send a panic message
    tx.send(ThreadReport::Panic("test panic".to_string()))
        .unwrap();

    let result = recv_timeout_and_join(rx, Duration::from_secs(1));
    match result {
        ThreadReport::Panic(msg) => {
            assert_eq!(msg, "test panic");
        }
        other => panic!("Expected Panic, got {:?}", other),
    }
}
