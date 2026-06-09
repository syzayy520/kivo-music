//! Health summary tests.
//!
//! Tests for is_healthy() and health_warnings() on WriterState.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig, WriteRequest,
};

#[test]
fn initial_state_is_healthy() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    let snap = writer.snapshot();
    assert!(snap.is_healthy());
    assert!(snap.health_warnings().is_empty());
}

#[test]
fn healthy_after_successful_write() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    let snap = writer.snapshot();
    assert!(snap.is_healthy());
    assert!(snap.health_warnings().is_empty());
}

#[test]
fn unhealthy_when_stalled() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    // WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    let snap = writer.snapshot();
    assert!(!snap.is_healthy());
    assert!(snap
        .health_warnings()
        .contains(&"currently stalled (WouldBlock)"));
}

#[test]
fn unhealthy_when_closed() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 1000,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer.process_request(&WriteRequest::Close).unwrap();

    let snap = writer.snapshot();
    assert!(!snap.is_healthy());
    assert!(snap.health_warnings().contains(&"writer is closed"));
}

#[test]
fn unhealthy_when_buffer_full() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer exactly
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    let snap = writer.snapshot();
    // Buffer full is a warning but not unhealthy by itself
    assert!(snap.is_healthy()); // No errors, not stalled, not closed
    assert!(snap.health_warnings().contains(&"buffer is full"));
}

#[test]
fn health_warnings_multiple_conditions() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    // WouldBlock (stalled + buffer full)
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    let snap = writer.snapshot();
    let warnings = snap.health_warnings();
    assert!(warnings.contains(&"currently stalled (WouldBlock)"));
    assert!(warnings.contains(&"buffer is full"));
}

#[test]
fn healthy_after_flush_resolves_stall() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    // WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    assert!(!writer.snapshot().is_healthy());

    // Flush resolves stall
    writer.process_request(&WriteRequest::Flush).unwrap();

    let snap = writer.snapshot();
    assert!(snap.is_healthy());
    assert!(snap.health_warnings().is_empty());
}

#[test]
fn health_warnings_returns_correct_count() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    assert_eq!(writer.snapshot().health_warnings().len(), 0);

    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    // WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    // 2 warnings: stalled + buffer full
    assert_eq!(writer.snapshot().health_warnings().len(), 2);
}
