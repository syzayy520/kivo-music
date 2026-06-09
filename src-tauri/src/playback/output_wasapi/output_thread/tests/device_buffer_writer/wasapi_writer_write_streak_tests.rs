//! Write streak tracking tests.
//!
//! Tests for consecutive successful write tracking in WASAPI device buffer writer.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig, WriteRequest,
};

#[test]
fn initial_write_streak_is_zero() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    let snap = writer.snapshot();
    assert_eq!(snap.write_streak, 0);
    assert_eq!(snap.max_write_streak, 0);
}

#[test]
fn successful_write_increments_streak() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    assert_eq!(writer.snapshot().write_streak, 1);

    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    assert_eq!(writer.snapshot().write_streak, 2);

    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    assert_eq!(writer.snapshot().write_streak, 3);
}

#[test]
fn max_write_streak_tracks_peak() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 256,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Build a streak of 3
    writer
        .process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    assert_eq!(writer.snapshot().max_write_streak, 3);

    // WouldBlock resets streak
    writer
        .process_request(&WriteRequest::write_packet(200, 44100, 2))
        .unwrap();
    assert_eq!(writer.snapshot().write_streak, 0);
    assert_eq!(writer.snapshot().max_write_streak, 3); // peak preserved

    // New streak of 2
    writer.process_request(&WriteRequest::Flush).unwrap();
    writer
        .process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    assert_eq!(writer.snapshot().write_streak, 2);
    assert_eq!(writer.snapshot().max_write_streak, 3); // still 3
}

#[test]
fn write_streak_resets_on_would_block() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 200,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Build streak
    writer
        .process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    assert_eq!(writer.snapshot().write_streak, 2);

    // WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(200, 44100, 2))
        .unwrap();
    assert_eq!(writer.snapshot().write_streak, 0);
}

#[test]
fn write_streak_not_affected_by_flush() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    assert_eq!(writer.snapshot().write_streak, 2);

    // Flush doesn't reset streak
    writer.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(writer.snapshot().write_streak, 2);

    // Continue streak
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    assert_eq!(writer.snapshot().write_streak, 3);
}

#[test]
fn write_streak_not_affected_by_noop() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    assert_eq!(writer.snapshot().write_streak, 1);

    writer.process_request(&WriteRequest::Noop).unwrap();
    assert_eq!(writer.snapshot().write_streak, 1);
}

#[test]
fn write_streak_survives_reset() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    writer.reset();
    assert_eq!(writer.snapshot().write_streak, 0);
    assert_eq!(writer.snapshot().max_write_streak, 0);
}

#[test]
fn write_streak_internal_state_consistent() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    let internal = writer.internal_state();
    assert_eq!(internal.write_streak, 2);
    assert_eq!(internal.max_write_streak, 2);
}
