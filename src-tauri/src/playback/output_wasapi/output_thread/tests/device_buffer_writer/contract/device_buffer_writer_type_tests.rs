//! Device buffer writer type tests.
//!
//! Tests for WriteRequest, WriteResult, and WriteError construction and properties.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    WriteError, WriteRequest, WriteResult,
};

#[test]
fn write_request_write_packet() {
    let request = WriteRequest::write_packet(512, 44100, 2);
    assert!(request.is_write_packet());
    assert!(!request.is_flush());
    assert!(!request.is_close());
    assert!(!request.is_noop());
    assert_eq!(request.frame_count(), 512);
    assert_eq!(request.sample_rate(), 44100);
    assert_eq!(request.channel_count(), 2);
}

#[test]
fn write_request_flush() {
    let request = WriteRequest::flush();
    assert!(request.is_flush());
    assert!(!request.is_write_packet());
    assert_eq!(request.frame_count(), 0);
}

#[test]
fn write_request_close() {
    let request = WriteRequest::close();
    assert!(request.is_close());
    assert!(!request.is_noop());
}

#[test]
fn write_request_noop_default() {
    assert!(WriteRequest::noop().is_noop());
    assert!(WriteRequest::default().is_noop());
}

#[test]
fn write_request_clone_debug() {
    let request = WriteRequest::write_packet(100, 44100, 2);
    let cloned = request.clone();
    assert_eq!(request, cloned);
    assert!(format!("{:?}", request).contains("WritePacket"));
}

#[test]
fn write_result_written() {
    let result = WriteResult::written(512, 4096);
    assert!(result.is_written());
    assert!(!result.is_would_block());
    assert!(!result.is_skipped());
    assert!(!result.is_noop());
    assert_eq!(result.frames_written(), 512);
    assert_eq!(result.bytes_written(), 4096);
}

#[test]
fn write_result_would_block() {
    let result = WriteResult::would_block();
    assert!(result.is_would_block());
    assert!(!result.is_written());
    assert_eq!(result.frames_written(), 0);
}

#[test]
fn write_result_skipped_noop() {
    let skipped = WriteResult::Skipped;
    assert!(skipped.is_skipped());
    let noop = WriteResult::default();
    assert!(noop.is_noop());
}

#[test]
fn write_result_clone_debug() {
    let result = WriteResult::written(100, 800);
    let cloned = result.clone();
    assert_eq!(result, cloned);
    assert!(format!("{:?}", result).contains("Written"));
}

#[test]
fn write_error_would_block() {
    let error = WriteError::WouldBlock;
    assert!(!error.is_fatal());
    assert!(error.is_recoverable());
    assert!(error.is_would_block());
    assert_eq!(format!("{}", error), "device buffer write would block");
}

#[test]
fn write_error_device_closed() {
    let error = WriteError::DeviceClosed;
    assert!(error.is_fatal());
    assert!(!error.is_recoverable());
    assert_eq!(format!("{}", error), "device buffer writer closed");
}

#[test]
fn write_error_format_mismatch() {
    let error = WriteError::FormatMismatch {
        expected: "f32 stereo".to_string(),
        actual: "i16 mono".to_string(),
    };
    assert!(error.is_format_mismatch());
    assert!(format!("{}", error).contains("expected f32 stereo"));
    assert!(format!("{}", error).contains("got i16 mono"));
}

#[test]
fn write_error_underflow() {
    let error = WriteError::Underflow {
        available: 100,
        requested: 512,
    };
    assert!(error.is_underflow());
    assert!(format!("{}", error).contains("available 100"));
}

#[test]
fn write_error_write_failed() {
    let error = WriteError::WriteFailed {
        description: "buffer overflow".to_string(),
    };
    assert!(error.is_write_failed());
    assert!(format!("{}", error).contains("buffer overflow"));
}

#[test]
fn write_error_internal() {
    let error = WriteError::Internal {
        description: "unexpected".to_string(),
    };
    assert!(!error.is_fatal());
    assert!(error.is_recoverable());
    assert!(format!("{}", error).contains("unexpected"));
}

#[test]
fn write_error_clone_eq_debug() {
    let error = WriteError::FormatMismatch {
        expected: "f32".to_string(),
        actual: "i16".to_string(),
    };
    let cloned = error.clone();
    assert_eq!(error, cloned);
    assert_ne!(error, WriteError::WouldBlock);
    assert!(format!("{:?}", WriteError::WouldBlock).contains("WouldBlock"));
}
