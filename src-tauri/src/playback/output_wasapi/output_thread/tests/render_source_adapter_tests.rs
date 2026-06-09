//! Render source adapter tests.
//!
//! Tests for adapter context, outcome, and error types.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::render_source_adapter::{
    AdapterContext, AdapterError, AdapterOutcome,
};

#[test]
fn adapter_context_default_values() {
    let ctx = AdapterContext::default();
    assert_eq!(ctx.driver_result, DriverResult::Idle);
    assert_eq!(ctx.frame_count, 0);
    assert_eq!(ctx.sample_rate, 44100);
    assert_eq!(ctx.channel_count, 2);
}

#[test]
fn adapter_context_custom_values() {
    let ctx = AdapterContext {
        driver_result: DriverResult::Idle,
        frame_count: 1024,
        sample_rate: 48000,
        channel_count: 2,
    };
    assert_eq!(ctx.frame_count, 1024);
    assert_eq!(ctx.sample_rate, 48000);
}

#[test]
fn adapter_context_clone_eq() {
    let ctx1 = AdapterContext::default();
    let ctx2 = ctx1.clone();
    assert_eq!(ctx1, ctx2);
}

#[test]
fn adapter_outcome_default_is_noop() {
    let outcome = AdapterOutcome::default();
    assert_eq!(outcome, AdapterOutcome::Noop);
}

#[test]
fn adapter_outcome_packet_variant() {
    let outcome = AdapterOutcome::Packet {
        frames_provided: 512,
        bytes_read: 2048,
    };
    match outcome {
        AdapterOutcome::Packet {
            frames_provided,
            bytes_read,
        } => {
            assert_eq!(frames_provided, 512);
            assert_eq!(bytes_read, 2048);
        }
        _ => panic!("Expected Packet variant"),
    }
}

#[test]
fn adapter_outcome_exhausted_variant() {
    let outcome = AdapterOutcome::Exhausted;
    assert_eq!(outcome, AdapterOutcome::Exhausted);
}

#[test]
fn adapter_outcome_skipped_variant() {
    let outcome = AdapterOutcome::Skipped;
    assert_eq!(outcome, AdapterOutcome::Skipped);
}

#[test]
fn adapter_outcome_failed_variant() {
    let outcome = AdapterOutcome::Failed;
    assert_eq!(outcome, AdapterOutcome::Failed);
}

#[test]
fn adapter_outcome_clone_eq() {
    let o1 = AdapterOutcome::Packet {
        frames_provided: 100,
        bytes_read: 400,
    };
    let o2 = o1.clone();
    assert_eq!(o1, o2);
}

#[test]
fn adapter_error_source_exhausted_display() {
    let err = AdapterError::SourceExhausted;
    assert_eq!(format!("{}", err), "source exhausted");
}

#[test]
fn adapter_error_format_mismatch_display() {
    let err = AdapterError::FormatMismatch {
        expected: "f32le".to_string(),
        actual: "s16le".to_string(),
    };
    assert_eq!(
        format!("{}", err),
        "format mismatch: expected f32le, got s16le"
    );
}

#[test]
fn adapter_error_source_closed_display() {
    let err = AdapterError::SourceClosed;
    assert_eq!(format!("{}", err), "source closed");
}

#[test]
fn adapter_error_internal_display() {
    let err = AdapterError::Internal {
        description: "test error".to_string(),
    };
    assert_eq!(format!("{}", err), "adapter internal error: test error");
}

#[test]
fn adapter_error_clone_eq() {
    let e1 = AdapterError::Internal {
        description: "test".to_string(),
    };
    let e2 = e1.clone();
    assert_eq!(e1, e2);
}

#[test]
fn adapter_error_not_eq_different_variants() {
    let e1 = AdapterError::SourceExhausted;
    let e2 = AdapterError::SourceClosed;
    assert_ne!(e1, e2);
}

#[test]
fn adapter_outcome_debug_format() {
    let outcome = AdapterOutcome::Packet {
        frames_provided: 256,
        bytes_read: 1024,
    };
    let debug_str = format!("{:?}", outcome);
    assert!(debug_str.contains("Packet"));
    assert!(debug_str.contains("256"));
    assert!(debug_str.contains("1024"));
}

#[test]
fn adapter_context_debug_format() {
    let ctx = AdapterContext::default();
    let debug_str = format!("{:?}", ctx);
    assert!(debug_str.contains("AdapterContext"));
    assert!(debug_str.contains("44100"));
}

#[test]
fn adapter_error_debug_format() {
    let err = AdapterError::FormatMismatch {
        expected: "f32le".to_string(),
        actual: "s16le".to_string(),
    };
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("FormatMismatch"));
    assert!(debug_str.contains("f32le"));
    assert!(debug_str.contains("s16le"));
}
