//! Fake device buffer writer tests.
//!
//! Tests for FakeDeviceBufferWriter construction, snapshot, cursor, and basic properties.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, FakeDeviceBufferWriter, WriteRequest, WriteResult,
};

#[test]
fn fake_writer_new() {
    let writer = FakeDeviceBufferWriter::new(1000);
    assert!(writer.is_ready());
    assert!(!writer.is_closed());
    assert_eq!(writer.capacity(), 1000);
    assert_eq!(writer.buffered_frames(), 0);
}

#[test]
fn fake_writer_empty() {
    let writer = FakeDeviceBufferWriter::empty();
    assert!(writer.is_ready());
    assert!(!writer.is_closed());
    assert_eq!(writer.capacity(), 10000);
}

#[test]
fn fake_writer_with_format() {
    let writer = FakeDeviceBufferWriter::with_format(5000, 48000, 1);
    assert_eq!(writer.capacity(), 5000);
    let cursor = writer.cursor();
    assert_eq!(cursor.sample_rate, 48000);
    assert_eq!(cursor.channel_count, 1);
}

#[test]
fn fake_writer_write_packet() {
    let mut writer = FakeDeviceBufferWriter::new(1000);
    let result = writer
        .process_request(&WriteRequest::WritePacket {
            frame_count: 512,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    assert!(result.is_written());
    assert_eq!(result.frames_written(), 512);
    assert_eq!(writer.buffered_frames(), 512);
}

#[test]
fn fake_writer_capacity_limit() {
    let mut writer = FakeDeviceBufferWriter::new(100);
    writer
        .process_request(&WriteRequest::WritePacket {
            frame_count: 50,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    writer
        .process_request(&WriteRequest::WritePacket {
            frame_count: 50,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    let result = writer
        .process_request(&WriteRequest::WritePacket {
            frame_count: 10,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    assert_eq!(result, WriteResult::WouldBlock);
}

#[test]
fn fake_writer_flush() {
    let mut writer = FakeDeviceBufferWriter::new(1000);
    writer
        .process_request(&WriteRequest::WritePacket {
            frame_count: 512,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap();
    assert_eq!(writer.buffered_frames(), 512);
    writer.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(writer.buffered_frames(), 0);
}

#[test]
fn fake_writer_close() {
    let mut writer = FakeDeviceBufferWriter::new(1000);
    writer.process_request(&WriteRequest::Close).unwrap();
    assert!(writer.is_closed());
    assert!(!writer.is_ready());
}

#[test]
fn fake_writer_closed_returns_error() {
    let mut writer = FakeDeviceBufferWriter::new(1000);
    writer.process_request(&WriteRequest::Close).unwrap();
    let err = writer
        .process_request(&WriteRequest::WritePacket {
            frame_count: 100,
            sample_rate: 44100,
            channel_count: 2,
        })
        .unwrap_err();
    assert!(err.is_fatal());
}

#[test]
fn fake_writer_snapshot_default() {
    let writer = FakeDeviceBufferWriter::empty();
    let snapshot = writer.snapshot();
    assert_eq!(snapshot.requests_accepted, 0);
    assert_eq!(snapshot.writes_completed, 0);
    assert_eq!(snapshot.frames_written, 0);
    assert_eq!(snapshot.bytes_written, 0);
    assert_eq!(snapshot.errors, 0);
    assert!(!snapshot.is_closed);
    assert!(snapshot.is_ready);
}

#[test]
fn fake_writer_cursor_default() {
    let writer = FakeDeviceBufferWriter::empty();
    let cursor = writer.cursor();
    assert_eq!(cursor.write_position, 0);
    assert_eq!(cursor.buffer_capacity, 10000);
    assert_eq!(cursor.buffered_frames, 0);
    assert_eq!(cursor.sample_rate, 44100);
    assert_eq!(cursor.channel_count, 2);
}

#[test]
fn fake_writer_debug_format() {
    let writer = FakeDeviceBufferWriter::empty();
    let debug_str = format!("{:?}", writer);
    assert!(debug_str.contains("FakeDeviceBufferWriter"));
}
