//! Ring buffer read result and error tests.
//!
//! Tests for RingBufferReadResult and RingBufferReadError construction and properties.

use crate::playback::output_wasapi::output_thread::runtime::ring_buffer_render_source::{
    RingBufferReadError, RingBufferReadResult,
};

#[test]
fn read_result_packet() {
    let result = RingBufferReadResult::Packet {
        frames_read: 512,
        bytes_read: 4096,
        sequence_number: 1,
    };
    assert!(result.is_packet());
    assert!(!result.is_exhausted());
    assert!(!result.is_empty());
    assert!(!result.is_skipped());
    assert_eq!(result.frames_read(), 512);
    assert_eq!(result.bytes_read(), 4096);
    assert_eq!(result.sequence_number(), 1);
}

#[test]
fn read_result_exhausted() {
    let result = RingBufferReadResult::Exhausted;
    assert!(!result.is_packet());
    assert!(result.is_exhausted());
    assert!(!result.is_empty());
    assert!(!result.is_skipped());
    assert_eq!(result.frames_read(), 0);
    assert_eq!(result.bytes_read(), 0);
    assert_eq!(result.sequence_number(), 0);
}

#[test]
fn read_result_empty() {
    let result = RingBufferReadResult::Empty;
    assert!(!result.is_packet());
    assert!(!result.is_exhausted());
    assert!(result.is_empty());
    assert!(!result.is_skipped());
    assert_eq!(result.frames_read(), 0);
}

#[test]
fn read_result_skipped() {
    let result = RingBufferReadResult::Skipped;
    assert!(!result.is_packet());
    assert!(!result.is_exhausted());
    assert!(!result.is_empty());
    assert!(result.is_skipped());
    assert_eq!(result.frames_read(), 0);
}

#[test]
fn read_result_default() {
    let result = RingBufferReadResult::default();
    assert!(result.is_skipped());
}

#[test]
fn read_result_clone() {
    let result = RingBufferReadResult::Packet {
        frames_read: 100,
        bytes_read: 800,
        sequence_number: 5,
    };
    let cloned = result.clone();
    assert_eq!(result, cloned);
}

#[test]
fn read_result_debug_format() {
    let result = RingBufferReadResult::Packet {
        frames_read: 100,
        bytes_read: 800,
        sequence_number: 5,
    };
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("Packet"));
    assert!(debug_str.contains("frames_read: 100"));
}

#[test]
fn read_error_source_exhausted() {
    let error = RingBufferReadError::SourceExhausted;
    assert!(error.is_fatal());
    assert!(!error.is_recoverable());
    assert!(!error.is_underrun());
    assert!(!error.is_overrun());
    assert!(!error.is_format_mismatch());
    assert_eq!(format!("{}", error), "ring buffer source exhausted");
}

#[test]
fn read_error_buffer_underrun() {
    let error = RingBufferReadError::BufferUnderrun;
    assert!(!error.is_fatal());
    assert!(error.is_recoverable());
    assert!(error.is_underrun());
    assert!(!error.is_overrun());
    assert!(!error.is_format_mismatch());
    assert_eq!(format!("{}", error), "ring buffer underrun: buffer empty");
}

#[test]
fn read_error_buffer_overrun() {
    let error = RingBufferReadError::BufferOverrun;
    assert!(!error.is_fatal());
    assert!(error.is_recoverable());
    assert!(!error.is_underrun());
    assert!(error.is_overrun());
    assert!(!error.is_format_mismatch());
    assert_eq!(format!("{}", error), "ring buffer overrun: buffer full");
}

#[test]
fn read_error_format_mismatch() {
    let error = RingBufferReadError::FormatMismatch {
        expected: "f32 stereo".to_string(),
        actual: "i16 mono".to_string(),
    };
    assert!(!error.is_fatal());
    assert!(error.is_recoverable());
    assert!(!error.is_underrun());
    assert!(!error.is_overrun());
    assert!(error.is_format_mismatch());
    assert!(format!("{}", error).contains("expected f32 stereo"));
    assert!(format!("{}", error).contains("got i16 mono"));
}

#[test]
fn read_error_source_closed() {
    let error = RingBufferReadError::SourceClosed;
    assert!(error.is_fatal());
    assert!(!error.is_recoverable());
    assert_eq!(format!("{}", error), "ring buffer source closed");
}

#[test]
fn read_error_capacity_exceeded() {
    let error = RingBufferReadError::CapacityExceeded {
        requested: 2000,
        maximum: 1000,
    };
    assert!(!error.is_fatal());
    assert!(error.is_recoverable());
    assert!(format!("{}", error).contains("requested 2000"));
    assert!(format!("{}", error).contains("maximum 1000"));
}

#[test]
fn read_error_invalid_state() {
    let error = RingBufferReadError::InvalidState {
        description: "read pointer ahead of write pointer".to_string(),
    };
    assert!(!error.is_fatal());
    assert!(error.is_recoverable());
    assert!(format!("{}", error).contains("read pointer ahead of write pointer"));
}

#[test]
fn read_error_internal() {
    let error = RingBufferReadError::Internal {
        description: "unexpected error".to_string(),
    };
    assert!(!error.is_fatal());
    assert!(error.is_recoverable());
    assert!(format!("{}", error).contains("unexpected error"));
}

#[test]
fn read_error_clone() {
    let error = RingBufferReadError::FormatMismatch {
        expected: "f32".to_string(),
        actual: "i16".to_string(),
    };
    let cloned = error.clone();
    assert_eq!(error, cloned);
}

#[test]
fn read_error_debug_format() {
    let error = RingBufferReadError::BufferUnderrun;
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("BufferUnderrun"));
}

#[test]
fn read_error_partial_eq() {
    let error1 = RingBufferReadError::SourceExhausted;
    let error2 = RingBufferReadError::SourceExhausted;
    assert_eq!(error1, error2);

    let error3 = RingBufferReadError::BufferUnderrun;
    assert_ne!(error1, error3);
}
