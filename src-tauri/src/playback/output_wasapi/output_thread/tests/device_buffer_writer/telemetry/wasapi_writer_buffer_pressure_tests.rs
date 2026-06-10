//! Buffer pressure detection tests.
//!
//! Tests for is_under_pressure() and pressure_level() on WriterState.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    writer_state::BufferPressure, DeviceBufferWriter, WasapiDeviceBufferWriter,
    WasapiDeviceBufferWriterConfig, WriteRequest,
};

#[test]
fn initial_pressure_is_relaxed() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    let snap = writer.snapshot();
    assert_eq!(snap.pressure_level(), BufferPressure::Relaxed);
    assert!(!snap.is_under_pressure(50));
}

#[test]
fn pressure_level_relaxed_below_50_percent() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // 40% fill
    writer
        .process_request(&WriteRequest::write_packet(40, 44100, 2))
        .unwrap();

    let snap = writer.snapshot();
    assert_eq!(snap.buffer_fill_percentage(), 40);
    assert_eq!(snap.pressure_level(), BufferPressure::Relaxed);
    assert!(!snap.is_under_pressure(50));
    assert!(snap.is_under_pressure(40));
}

#[test]
fn pressure_level_moderate_50_to_74_percent() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // 60% fill
    writer
        .process_request(&WriteRequest::write_packet(60, 44100, 2))
        .unwrap();

    let snap = writer.snapshot();
    assert_eq!(snap.pressure_level(), BufferPressure::Moderate);
    assert!(snap.is_under_pressure(50));
    assert!(snap.is_under_pressure(60));
    assert!(!snap.is_under_pressure(75));
}

#[test]
fn pressure_level_high_75_to_89_percent() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // 80% fill
    writer
        .process_request(&WriteRequest::write_packet(80, 44100, 2))
        .unwrap();

    let snap = writer.snapshot();
    assert_eq!(snap.pressure_level(), BufferPressure::High);
    assert!(snap.pressure_level().is_concerning());
    assert!(!snap.pressure_level().is_critical());
    assert!(snap.is_under_pressure(75));
    assert!(!snap.is_under_pressure(90));
}

#[test]
fn pressure_level_critical_90_percent_and_above() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // 100% fill (exactly at capacity)
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    let snap = writer.snapshot();
    assert_eq!(snap.pressure_level(), BufferPressure::Critical);
    assert!(snap.pressure_level().is_concerning());
    assert!(snap.pressure_level().is_critical());
    assert!(snap.is_under_pressure(90));
    assert!(snap.is_under_pressure(100));
}

#[test]
fn pressure_exactly_at_threshold() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Exactly 50% fill
    writer
        .process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();

    let snap = writer.snapshot();
    assert_eq!(snap.pressure_level(), BufferPressure::Moderate);
    assert!(snap.is_under_pressure(50));
    assert!(!snap.is_under_pressure(51));
}

#[test]
fn pressure_with_zero_capacity_returns_relaxed() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 0,
        ..Default::default()
    };
    let writer = WasapiDeviceBufferWriter::new(config);
    let snap = writer.snapshot();

    assert_eq!(snap.pressure_level(), BufferPressure::Relaxed);
    assert!(!snap.is_under_pressure(0));
}

#[test]
fn pressure_after_flush_drops_to_relaxed() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill to 90%
    writer
        .process_request(&WriteRequest::write_packet(90, 44100, 2))
        .unwrap();
    assert_eq!(writer.snapshot().pressure_level(), BufferPressure::Critical);

    // Flush drops to Empty
    writer.process_request(&WriteRequest::Flush).unwrap();
    let snap = writer.snapshot();
    assert_eq!(snap.pressure_level(), BufferPressure::Relaxed);
    assert!(!snap.is_under_pressure(1));
}

#[test]
fn is_under_pressure_clamps_threshold_above_100() {
    let config = WasapiDeviceBufferWriterConfig {
        capacity_frames: 100,
        ..Default::default()
    };
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(99, 44100, 2))
        .unwrap();

    let snap = writer.snapshot();
    // 99% < 100% clamped, so should not be under 100% pressure
    assert!(!snap.is_under_pressure(101));
}
