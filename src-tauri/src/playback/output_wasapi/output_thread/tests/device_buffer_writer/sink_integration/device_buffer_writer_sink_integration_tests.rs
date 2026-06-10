//! Device buffer writer sink integration tests.
//!
//! Tests for FakeDeviceBufferWriter integration with helper functions.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    create_empty_writer, create_test_writer, create_writer_with_format, invoke_test_close,
    invoke_test_flush, invoke_test_write, write_all_packets, DeviceBufferWriter,
};

#[test]
fn create_test_writer_helper() {
    let writer = create_test_writer(1000);
    assert!(writer.is_ready());
    assert_eq!(writer.capacity(), 1000);
}

#[test]
fn create_empty_writer_helper() {
    let writer = create_empty_writer();
    assert!(writer.is_ready());
    assert!(!writer.is_closed());
    assert_eq!(writer.capacity(), 10000);
}

#[test]
fn create_writer_with_format_helper() {
    let writer = create_writer_with_format(5000, 48000, 1);
    assert_eq!(writer.capacity(), 5000);
    let cursor = writer.cursor();
    assert_eq!(cursor.sample_rate, 48000);
    assert_eq!(cursor.channel_count, 1);
}

#[test]
fn invoke_test_write_success() {
    let mut writer = create_test_writer(1000);
    let result = invoke_test_write(&mut writer, 512, 44100, 2).unwrap();
    assert!(result.is_written());
    assert_eq!(result.frames_written(), 512);
}

#[test]
fn invoke_test_write_would_block() {
    let mut writer = create_test_writer(100);
    invoke_test_write(&mut writer, 50, 44100, 2).unwrap();
    invoke_test_write(&mut writer, 50, 44100, 2).unwrap();
    let err = invoke_test_write(&mut writer, 10, 44100, 2).unwrap_err();
    assert!(err.is_would_block());
}

#[test]
fn invoke_test_flush_helper() {
    let mut writer = create_test_writer(1000);
    invoke_test_write(&mut writer, 512, 44100, 2).unwrap();
    assert_eq!(writer.buffered_frames(), 512);
    invoke_test_flush(&mut writer).unwrap();
    assert_eq!(writer.buffered_frames(), 0);
}

#[test]
fn invoke_test_close_helper() {
    let mut writer = create_test_writer(1000);
    invoke_test_close(&mut writer).unwrap();
    assert!(writer.is_closed());
    assert!(!writer.is_ready());
}

#[test]
fn write_all_packets_helper() {
    let mut writer = create_test_writer(1000);
    let (total_frames, total_bytes) = write_all_packets(&mut writer, 100, 44100, 2);
    assert_eq!(total_frames, 1000);
    assert_eq!(total_bytes, 1000 * 2 * 4);
}

#[test]
fn write_all_packets_empty_writer() {
    let mut writer = create_test_writer(0);
    let (total_frames, total_bytes) = write_all_packets(&mut writer, 100, 44100, 2);
    assert_eq!(total_frames, 0);
    assert_eq!(total_bytes, 0);
}

#[test]
fn writer_snapshot_after_integration_use() {
    let mut writer = create_test_writer(1000);
    invoke_test_write(&mut writer, 512, 44100, 2).unwrap();
    invoke_test_write(&mut writer, 256, 44100, 2).unwrap();
    invoke_test_flush(&mut writer).unwrap();

    let snapshot = writer.snapshot();
    assert_eq!(snapshot.requests_accepted, 3);
    assert_eq!(snapshot.writes_completed, 2);
    assert_eq!(snapshot.frames_written, 768);
    assert_eq!(snapshot.flush_count, 1);
}

#[test]
fn writer_cursor_after_integration_use() {
    let mut writer = create_test_writer(1000);
    invoke_test_write(&mut writer, 512, 44100, 2).unwrap();

    let cursor = writer.cursor();
    assert_eq!(cursor.write_position, 512);
    assert_eq!(cursor.total_frames_written, 512);
    assert_eq!(cursor.buffered_frames, 512);
}
