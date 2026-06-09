//! Runtime placeholder tests for WASAPI device buffer writer.
//!
//! Verifies simulated buffer behavior:
//! 1. WritePacket returns Written with correct frames/bytes
//! 2. Buffer fill tracking
//! 3. WouldBlock when buffer full
//! 4. Flush clears buffer fill
//! 5. Close idempotency
//! 6. Noop does not pollute state
//! 7. Snapshot/cursor reflect real state
//! 8. No real WASAPI/COM/device calls

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig, WriteError,
    WriteRequest, WriteResult,
};

#[test]
fn write_packet_returns_written_with_correct_frames() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    let result = writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    match result {
        WriteResult::Written {
            frames_written,
            bytes_written,
        } => {
            assert_eq!(frames_written, 256);
            assert_eq!(bytes_written, 256 * 44100 * 2 * 4);
        }
        other => panic!("expected Written, got {:?}", other),
    }
}

#[test]
fn write_packet_updates_buffer_fill() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    assert_eq!(writer.internal_state().buffer_fill_frames(), 256);

    writer
        .process_request(&WriteRequest::write_packet(128, 44100, 2))
        .unwrap();
    assert_eq!(writer.internal_state().buffer_fill_frames(), 384);
}

#[test]
fn write_packet_updates_frames_written() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(128, 44100, 2))
        .unwrap();

    assert_eq!(writer.internal_state().frames_written(), 384);
}

#[test]
fn write_packet_updates_bytes_written() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    let expected_bytes = 256 * 44100 * 2 * 4;
    assert_eq!(writer.internal_state().bytes_written(), expected_bytes);
}

#[test]
fn write_packet_increments_write_attempts() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(128, 44100, 2))
        .unwrap();

    assert_eq!(writer.internal_state().write_attempts(), 2);
}

#[test]
fn buffer_full_returns_would_block() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer to capacity
    let result = writer
        .process_request(&WriteRequest::write_packet(512, 44100, 2))
        .unwrap();
    assert!(result.is_written());

    // Next write should WouldBlock
    let result = writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    assert_eq!(result, WriteResult::WouldBlock);
}

#[test]
fn partial_fill_then_overflow_returns_would_block() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill partially
    writer
        .process_request(&WriteRequest::write_packet(400, 44100, 2))
        .unwrap();

    // Try to write more than remaining capacity (512 - 400 = 112)
    let result = writer
        .process_request(&WriteRequest::write_packet(200, 44100, 2))
        .unwrap();
    assert_eq!(result, WriteResult::WouldBlock);
}

#[test]
fn would_block_increments_counter() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    // Trigger WouldBlock twice
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    assert_eq!(writer.internal_state().would_block_count(), 2);
}

#[test]
fn flush_clears_buffer_fill() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Fill buffer partially
    writer
        .process_request(&WriteRequest::write_packet(512, 44100, 2))
        .unwrap();
    assert_eq!(writer.internal_state().buffer_fill_frames(), 512);

    // Flush
    writer.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(writer.internal_state().buffer_fill_frames(), 0);
}

#[test]
fn flush_increments_flush_count() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    writer.process_request(&WriteRequest::Flush).unwrap();
    writer.process_request(&WriteRequest::Flush).unwrap();

    assert_eq!(writer.internal_state().flush_count(), 2);
}

#[test]
fn flush_allows_more_writes() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    // WouldBlock
    let result = writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    assert_eq!(result, WriteResult::WouldBlock);

    // Flush clears buffer
    writer.process_request(&WriteRequest::Flush).unwrap();

    // Now writes succeed again
    let result = writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    assert!(result.is_written());
}

#[test]
fn close_is_idempotent() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    writer.process_request(&WriteRequest::Close).unwrap();
    assert!(writer.is_closed());

    // Second close returns DeviceClosed (not panic)
    let err = writer.process_request(&WriteRequest::Close).unwrap_err();
    assert_eq!(err, WriteError::DeviceClosed);
}

#[test]
fn noop_does_not_pollute_state() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    let state_before = writer.internal_state().clone();
    writer.process_request(&WriteRequest::Noop).unwrap();
    let state_after = writer.internal_state();

    assert_eq!(
        state_before.buffer_fill_frames(),
        state_after.buffer_fill_frames()
    );
    assert_eq!(state_before.frames_written(), state_after.frames_written());
    assert_eq!(state_before.bytes_written(), state_after.bytes_written());
    assert_eq!(state_before.write_attempts(), state_after.write_attempts());
    assert_eq!(
        state_before.would_block_count(),
        state_after.would_block_count()
    );
    assert_eq!(state_before.flush_count(), state_after.flush_count());
}

#[test]
fn snapshot_reflects_real_state() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    let snapshot = writer.snapshot();
    assert_eq!(snapshot.buffer_fill_frames, 256);
    assert_eq!(snapshot.frames_written, 256);
    assert_eq!(snapshot.buffer_capacity_frames, 1024);
    assert_eq!(snapshot.requests_accepted, 1);
    assert_eq!(snapshot.would_block_count, 0);
    assert!(snapshot.is_ready);
    assert!(!snapshot.is_closed);
}

#[test]
fn cursor_reflects_real_state() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    let cursor = writer.cursor();
    assert_eq!(cursor.write_position, 256);
    assert_eq!(cursor.buffered_frames, 256);
    assert_eq!(cursor.buffer_capacity, 1024);
    assert_eq!(cursor.total_frames_written, 256);
    assert_eq!(cursor.sample_rate, 44100);
    assert_eq!(cursor.channel_count, 2);
}

#[test]
fn reset_clears_all_runtime_state() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Write, flush, trigger WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(1024, 44100, 2))
        .unwrap();
    writer.process_request(&WriteRequest::Flush).unwrap();
    writer
        .process_request(&WriteRequest::write_packet(1024, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap(); // WouldBlock

    writer.reset();

    let state = writer.internal_state();
    assert!(!state.is_closed());
    assert_eq!(state.buffer_fill_frames(), 0);
    assert_eq!(state.frames_written(), 0);
    assert_eq!(state.bytes_written(), 0);
    assert_eq!(state.write_attempts(), 0);
    assert_eq!(state.would_block_count(), 0);
    assert_eq!(state.flush_count(), 0);
}

#[test]
fn write_with_different_frame_counts() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    // Write 1 frame
    let result = writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    assert!(result.is_written());

    // Write 1000 frames
    let result = writer
        .process_request(&WriteRequest::write_packet(1000, 44100, 2))
        .unwrap();
    assert!(result.is_written());

    assert_eq!(writer.internal_state().frames_written(), 1001);
}

#[test]
fn no_real_wasapi_com_or_device_dependency() {
    // This test exists to confirm the type system allows construction
    // without any WASAPI/COM/device resources.
    let config = WasapiDeviceBufferWriterConfig::new(1024, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Full lifecycle without real device
    writer
        .process_request(&WriteRequest::write_packet(512, 44100, 2))
        .unwrap();
    writer.process_request(&WriteRequest::Flush).unwrap();
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    writer.process_request(&WriteRequest::Close).unwrap();

    // All operations completed without COM/WASAPI
    assert!(writer.is_closed());
    assert!(writer.internal_state().write_attempts() > 0);
}
