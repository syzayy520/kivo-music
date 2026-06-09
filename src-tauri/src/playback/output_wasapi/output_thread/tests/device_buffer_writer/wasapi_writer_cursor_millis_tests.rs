//! Cursor millisecond precision tests.
//!
//! Tests for write_position_millis(), buffered_millis(), capacity_millis(),
//! total_written_millis() on WriterCursor.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig, WriteRequest,
};

#[test]
fn initial_cursor_millis_are_zero() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    let cursor = writer.cursor();
    // Default config: sample_rate=44100, capacity=1024
    // Initial: write_position=0, buffered=0, total_written=0
    assert_eq!(cursor.write_position_millis(), 0.0);
    assert_eq!(cursor.buffered_millis(), 0.0);
    assert_eq!(cursor.total_written_millis(), 0.0);
    // capacity is non-zero
    assert!(cursor.capacity_millis() > 0.0);
}

#[test]
fn cursor_millis_after_write() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 44100, // 1 second at 44100 Hz
        channels: 2,
        sample_rate: 44100,
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write 4410 frames = 0.1 seconds = 100ms
    writer
        .process_request(&WriteRequest::write_packet(4410, 44100, 2))
        .unwrap();

    let cursor = writer.cursor();
    let epsilon = 0.001;

    assert!((cursor.write_position_millis() - 100.0).abs() < epsilon);
    assert!((cursor.buffered_millis() - 100.0).abs() < epsilon);
    assert!((cursor.total_written_millis() - 100.0).abs() < epsilon);
    assert!((cursor.capacity_millis() - 1000.0).abs() < epsilon);
}

#[test]
fn cursor_millis_consistent_with_seconds() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 88200,
        channels: 2,
        sample_rate: 48000,
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(4800, 48000, 2))
        .unwrap();

    let cursor = writer.cursor();
    // 4800 / 48000 = 0.1 seconds = 100ms
    assert!(
        (cursor.write_position_millis() - cursor.write_position_seconds() * 1000.0).abs() < 0.0001
    );
    assert!((cursor.buffered_millis() - cursor.buffered_seconds() * 1000.0).abs() < 0.0001);
    assert!((cursor.capacity_millis() - cursor.capacity_seconds() * 1000.0).abs() < 0.0001);
    assert!(
        (cursor.total_written_millis() - cursor.total_written_seconds() * 1000.0).abs() < 0.0001
    );
}

#[test]
fn cursor_millis_with_zero_sample_rate() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1024,
        channels: 2,
        sample_rate: 0,
    };
    let writer = WasapiDeviceBufferWriter::new(config);
    let cursor = writer.cursor();

    assert_eq!(cursor.write_position_millis(), 0.0);
    assert_eq!(cursor.buffered_millis(), 0.0);
    assert_eq!(cursor.capacity_millis(), 0.0);
    assert_eq!(cursor.total_written_millis(), 0.0);
}

#[test]
fn cursor_millis_after_multiple_writes() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 88200,
        channels: 2,
        sample_rate: 44100,
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write 3 x 1470 frames = 4410 frames = 100ms total
    for _ in 0..3 {
        writer
            .process_request(&WriteRequest::write_packet(1470, 44100, 2))
            .unwrap();
    }

    let cursor = writer.cursor();
    let epsilon = 0.001;

    assert!((cursor.write_position_millis() - 100.0).abs() < epsilon);
    assert!((cursor.total_written_millis() - 100.0).abs() < epsilon);
}

#[test]
fn cursor_millis_after_flush() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 88200,
        channels: 2,
        sample_rate: 44100,
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write 4410 frames = 100ms
    writer
        .process_request(&WriteRequest::write_packet(4410, 44100, 2))
        .unwrap();

    // Flush clears buffer
    writer.process_request(&WriteRequest::Flush).unwrap();

    let cursor = writer.cursor();
    assert_eq!(cursor.buffered_millis(), 0.0);
    // write_position doesn't reset on flush (circular buffer position persists)
    // total_written doesn't reset either
    assert!(cursor.total_written_millis() > 0.0);
}

#[test]
fn cursor_millis_precision_at_96khz() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 96000,
        channels: 2,
        sample_rate: 96000,
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write 960 frames = 10ms at 96kHz
    writer
        .process_request(&WriteRequest::write_packet(960, 96000, 2))
        .unwrap();

    let cursor = writer.cursor();
    let epsilon = 0.001;

    assert!((cursor.write_position_millis() - 10.0).abs() < epsilon);
    assert!((cursor.buffered_millis() - 10.0).abs() < epsilon);
    assert!((cursor.capacity_millis() - 1000.0).abs() < epsilon);
}
