//! Cursor time position tests.
//!
//! Tests for time-based position methods in WriterCursor.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig, WriteRequest,
};

#[test]
fn initial_cursor_time_position_is_zero() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    let cursor = writer.cursor();
    assert_eq!(cursor.write_position_seconds(), 0.0);
    assert_eq!(cursor.buffered_seconds(), 0.0);
    assert_eq!(cursor.total_written_seconds(), 0.0);
    // capacity_seconds is non-zero (1024/44100)
    assert!(cursor.capacity_seconds() > 0.0);
}

#[test]
fn cursor_time_position_after_write() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 44100 * 10, // 10 seconds at 44100 Hz
        sample_rate: 44100,
        channels: 2,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write 44100 frames = 1 second
    writer
        .process_request(&WriteRequest::write_packet(44100, 44100, 2))
        .unwrap();

    let cursor = writer.cursor();
    assert!((cursor.write_position_seconds() - 1.0).abs() < 0.001);
    assert!((cursor.buffered_seconds() - 1.0).abs() < 0.001);
    assert!((cursor.total_written_seconds() - 1.0).abs() < 0.001);
}

#[test]
fn cursor_time_position_with_different_sample_rate() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 48000 * 5, // 5 seconds at 48000 Hz
        sample_rate: 48000,
        channels: 2,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write 48000 frames = 1 second at 48000 Hz
    writer
        .process_request(&WriteRequest::write_packet(48000, 48000, 2))
        .unwrap();

    let cursor = writer.cursor();
    assert!((cursor.write_position_seconds() - 1.0).abs() < 0.001);
    assert!((cursor.buffered_seconds() - 1.0).abs() < 0.001);
}

#[test]
fn cursor_time_position_after_flush() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 44100 * 10,
        sample_rate: 44100,
        channels: 2,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write 2 seconds
    writer
        .process_request(&WriteRequest::write_packet(44100 * 2, 44100, 2))
        .unwrap();

    // Flush clears buffer but not write position
    writer.process_request(&WriteRequest::Flush).unwrap();

    let cursor = writer.cursor();
    // Write position should still be at 2 seconds
    assert!((cursor.write_position_seconds() - 2.0).abs() < 0.001);
    // Buffered should be 0 after flush
    assert!((cursor.buffered_seconds() - 0.0).abs() < 0.001);
    // Total written should still be 2 seconds
    assert!((cursor.total_written_seconds() - 2.0).abs() < 0.001);
}

#[test]
fn cursor_capacity_seconds() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 44100 * 5, // 5 seconds
        sample_rate: 44100,
        channels: 2,
        ..Default::default()
    };
    let writer = WasapiDeviceBufferWriter::new(config);

    let cursor = writer.cursor();
    assert!((cursor.capacity_seconds() - 5.0).abs() < 0.001);
}

#[test]
fn cursor_time_zero_sample_rate_returns_zero() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        sample_rate: 0,
        channels: 2,
        ..Default::default()
    };
    let writer = WasapiDeviceBufferWriter::new(config);

    let cursor = writer.cursor();
    assert_eq!(cursor.write_position_seconds(), 0.0);
    assert_eq!(cursor.buffered_seconds(), 0.0);
    assert_eq!(cursor.capacity_seconds(), 0.0);
    assert_eq!(cursor.total_written_seconds(), 0.0);
}

#[test]
fn cursor_time_position_circular_buffer() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 44100 * 2, // 2 second buffer
        sample_rate: 44100,
        channels: 2,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write 0.5 seconds
    writer
        .process_request(&WriteRequest::write_packet(22050, 44100, 2))
        .unwrap();

    let cursor = writer.cursor();
    assert!((cursor.write_position_seconds() - 0.5).abs() < 0.001);

    // Flush and write again
    writer.process_request(&WriteRequest::Flush).unwrap();
    writer
        .process_request(&WriteRequest::write_packet(22050, 44100, 2))
        .unwrap();

    let cursor = writer.cursor();
    // Write position should be at 1.0 seconds (22050 + 22050 = 44100 frames)
    assert!((cursor.write_position_seconds() - 1.0).abs() < 0.001);
    // Total written should be 1 second (0.5 + 0.5)
    assert!((cursor.total_written_seconds() - 1.0).abs() < 0.001);
}
