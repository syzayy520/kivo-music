//! Request validation tests for WASAPI device buffer writer.
//!
//! Verifies request validation:
//! 1. Zero frame_count rejected
//! 2. Zero channel_count rejected
//! 3. Zero sample_rate rejected
//! 4. Valid requests pass validation
//! 5. Validation errors don't affect state

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WasapiDeviceBufferWriter, WriteRequest,
};

#[test]
fn zero_frame_count_returns_invalid_request() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    let err = writer
        .process_request(&WriteRequest::write_packet(0, 44100, 2))
        .unwrap_err();

    assert!(err.is_invalid_request());
    assert!(err.to_string().contains("frame_count must be > 0"));
}

#[test]
fn zero_channel_count_returns_invalid_request() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    let err = writer
        .process_request(&WriteRequest::write_packet(256, 44100, 0))
        .unwrap_err();

    assert!(err.is_invalid_request());
    assert!(err.to_string().contains("channel_count must be > 0"));
}

#[test]
fn zero_sample_rate_returns_invalid_request() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    let err = writer
        .process_request(&WriteRequest::write_packet(256, 0, 2))
        .unwrap_err();

    assert!(err.is_invalid_request());
    assert!(err.to_string().contains("sample_rate must be > 0"));
}

#[test]
fn valid_request_passes_validation() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    let result = writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    assert!(result.is_written());
}

#[test]
fn validation_error_does_not_increment_write_attempts() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Invalid request
    let _ = writer.process_request(&WriteRequest::write_packet(0, 44100, 2));

    assert_eq!(writer.internal_state().write_attempts(), 0);
}

#[test]
fn validation_error_does_not_update_buffer_fill() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Invalid request
    let _ = writer.process_request(&WriteRequest::write_packet(0, 44100, 2));

    assert_eq!(writer.internal_state().buffer_fill_frames(), 0);
}

#[test]
fn validation_error_does_not_update_frames_written() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Invalid request
    let _ = writer.process_request(&WriteRequest::write_packet(0, 44100, 2));

    assert_eq!(writer.internal_state().frames_written(), 0);
}

#[test]
fn validation_error_does_not_update_bytes_written() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Invalid request
    let _ = writer.process_request(&WriteRequest::write_packet(0, 44100, 2));

    assert_eq!(writer.internal_state().bytes_written(), 0);
}

#[test]
fn validation_error_does_not_affect_would_block_count() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Invalid request
    let _ = writer.process_request(&WriteRequest::write_packet(0, 44100, 2));

    assert_eq!(writer.internal_state().would_block_count(), 0);
}

#[test]
fn flush_request_bypasses_validation() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Flush should work even with no prior writes
    let result = writer.process_request(&WriteRequest::Flush).unwrap();
    assert!(result.is_noop());
}

#[test]
fn close_request_bypasses_validation() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Close should work
    let result = writer.process_request(&WriteRequest::Close).unwrap();
    assert!(result.is_noop());
    assert!(writer.is_closed());
}

#[test]
fn noop_request_bypasses_validation() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Noop should work
    let result = writer.process_request(&WriteRequest::Noop).unwrap();
    assert!(result.is_noop());
}

#[test]
fn validation_error_preserves_last_result() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // First valid write
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    let last_result_before = writer.internal_state().last_result().clone();

    // Invalid request
    let _ = writer.process_request(&WriteRequest::write_packet(0, 44100, 2));

    let last_result_after = writer.internal_state().last_result().clone();
    assert_eq!(last_result_before, last_result_after);
}

#[test]
fn multiple_validation_errors_independent() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Multiple invalid requests
    let _ = writer.process_request(&WriteRequest::write_packet(0, 44100, 2));
    let _ = writer.process_request(&WriteRequest::write_packet(256, 0, 2));
    let _ = writer.process_request(&WriteRequest::write_packet(256, 44100, 0));

    // State should still be clean
    assert_eq!(writer.internal_state().write_attempts(), 0);
    assert_eq!(writer.internal_state().buffer_fill_frames(), 0);
    assert_eq!(writer.internal_state().frames_written(), 0);
}
