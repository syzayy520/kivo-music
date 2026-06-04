// outcome_tests.rs
//
// Tests for apply_thread_outcome logic (output_thread_outcome.rs).
// Validates all four ThreadReport match branches without spawning threads
// or running WASAPI.

use crate::playback::output_wasapi::ring_buffer_output_thread::{
    output_thread_outcome::apply_thread_outcome,
    report::WasapiRingBufferOutputThreadSmokeReport,
    thread_report::ThreadReport,
};

use super::smoke_assertions;

const TIMEOUT_MS: u64 = 15000;

#[test]
fn outcome_success_merges_report() {
    let mut report = WasapiRingBufferOutputThreadSmokeReport::default();

    let mut source = WasapiRingBufferOutputThreadSmokeReport::default();
    source.attempted = true;
    source.ring_buffer_created = true;
    source.used_silent_flag = true;
    source.total_silence_frames_filled = 4800;
    source.com_initialized = true;
    source.endpoint_available = true;
    source.started_audio_client = true;
    source.stopped_audio_client = true;
    source.reset_succeeded = true;

    apply_thread_outcome(&mut report, ThreadReport::Success(Box::new(source)), TIMEOUT_MS);

    assert!(report.thread_report_recv_attempted, "recv attempted");
    assert!(report.attempted, "attempted merged");
    assert!(report.ring_buffer_created, "ring buffer merged");
    assert!(report.used_silent_flag, "silent flag merged");
    assert_eq!(report.total_silence_frames_filled, 4800, "silence filled merged");
    assert!(report.com_initialized, "COM merged");
    assert!(report.started_audio_client, "started merged");
    assert!(report.reset_succeeded, "reset merged");
    smoke_assertions::assert_prohibited_always_false(&report);
}

#[test]
fn outcome_timeout_sets_timeout_fields() {
    let mut report = WasapiRingBufferOutputThreadSmokeReport::default();

    apply_thread_outcome(&mut report, ThreadReport::Timeout, TIMEOUT_MS);

    assert!(report.thread_report_recv_attempted, "recv attempted");
    assert!(!report.thread_report_received, "not received");
    assert!(report.thread_recv_timed_out, "timed out");
    assert_eq!(report.thread_recv_timeout_ms, Some(TIMEOUT_MS), "timeout ms set");
    assert!(!report.output_thread_join_attempted, "join not attempted");
    assert!(!report.output_thread_joined, "not joined");
    assert!(!report.output_thread_join_failed, "join not failed");
    assert!(!report.thread_panic_caught, "no panic");
    assert!(
        report.error_message.as_deref().unwrap().contains("timed out"),
        "error mentions timeout"
    );
    smoke_assertions::assert_prohibited_always_false(&report);
}

#[test]
fn outcome_panic_sets_panic_fields() {
    let mut report = WasapiRingBufferOutputThreadSmokeReport::default();

    apply_thread_outcome(
        &mut report,
        ThreadReport::Panic("test panic message".to_string()),
        TIMEOUT_MS,
    );

    assert!(report.thread_report_recv_attempted, "recv attempted");
    assert!(report.thread_panic_caught, "panic caught");
    assert_eq!(
        report.thread_panic_message,
        Some("test panic message".to_string()),
        "panic message preserved"
    );
    assert!(
        report.error_message.as_deref().unwrap().contains("panicked"),
        "error mentions panic"
    );
    smoke_assertions::assert_prohibited_always_false(&report);
}

#[test]
fn outcome_join_failed_sets_join_failed_fields() {
    let mut report = WasapiRingBufferOutputThreadSmokeReport::default();

    apply_thread_outcome(
        &mut report,
        ThreadReport::JoinFailed("join error".to_string()),
        TIMEOUT_MS,
    );

    assert!(report.thread_report_recv_attempted, "recv attempted");
    assert!(report.output_thread_join_attempted, "join attempted");
    assert!(!report.output_thread_joined, "not joined");
    assert!(report.output_thread_join_failed, "join failed");
    assert!(
        report.error_message.as_deref().unwrap().contains("join failed"),
        "error mentions join failed"
    );
    smoke_assertions::assert_prohibited_always_false(&report);
}
