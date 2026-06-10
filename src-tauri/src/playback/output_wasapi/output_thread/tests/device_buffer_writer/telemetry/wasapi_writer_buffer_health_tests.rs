//! Buffer health metrics tests for WASAPI device buffer writer.
//!
//! Verifies buffer health tracking:
//! 1. Consecutive would-block counter increments
//! 2. Consecutive would-block counter resets on success
//! 3. Max consecutive would-block tracking
//! 4. Snapshot reflects health metrics
//! 5. Stall detection

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig, WriteRequest,
};

#[test]
fn consecutive_would_blocks_starts_at_zero() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    assert_eq!(writer.internal_state().consecutive_would_blocks(), 0);
    assert_eq!(writer.internal_state().max_consecutive_would_blocks(), 0);
}

#[test]
fn single_would_block_increments_consecutive() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    // Trigger WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    assert_eq!(writer.internal_state().consecutive_would_blocks(), 1);
    assert_eq!(writer.internal_state().max_consecutive_would_blocks(), 1);
}

#[test]
fn multiple_would_blocks_increment_consecutive() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    // Trigger 3 WouldBlocks
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    assert_eq!(writer.internal_state().consecutive_would_blocks(), 3);
    assert_eq!(writer.internal_state().max_consecutive_would_blocks(), 3);
}

#[test]
fn successful_write_resets_consecutive_would_blocks() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    // Trigger 2 WouldBlocks
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    assert_eq!(writer.internal_state().consecutive_would_blocks(), 2);

    // Flush to make room
    writer.process_request(&WriteRequest::Flush).unwrap();

    // Successful write resets consecutive
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    assert_eq!(writer.internal_state().consecutive_would_blocks(), 0);
    // Max should still be 2
    assert_eq!(writer.internal_state().max_consecutive_would_blocks(), 2);
}

#[test]
fn max_consecutive_tracks_peak() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // First streak: 2 WouldBlocks
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    // Flush and write to reset
    writer.process_request(&WriteRequest::Flush).unwrap();
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    // Second streak: 1 WouldBlock (less than peak)
    writer
        .process_request(&WriteRequest::write_packet(200, 44100, 2))
        .unwrap();

    assert_eq!(writer.internal_state().consecutive_would_blocks(), 1);
    assert_eq!(writer.internal_state().max_consecutive_would_blocks(), 2);
}

#[test]
fn snapshot_reflects_health_metrics() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill and trigger WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    let snapshot = writer.snapshot();
    assert_eq!(snapshot.consecutive_would_blocks, 1);
    assert_eq!(snapshot.max_consecutive_would_blocks, 1);
}

#[test]
fn is_stalled_returns_true_when_consecutive_would_blocks() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Not stalled initially
    assert!(!writer.snapshot().is_stalled());

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    // Trigger WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    // Now stalled
    assert!(writer.snapshot().is_stalled());
}

#[test]
fn reset_clears_health_metrics() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill and trigger WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    assert_eq!(writer.internal_state().consecutive_would_blocks(), 1);
    assert_eq!(writer.internal_state().max_consecutive_would_blocks(), 1);

    // Reset clears all
    writer.reset();

    assert_eq!(writer.internal_state().consecutive_would_blocks(), 0);
    assert_eq!(writer.internal_state().max_consecutive_would_blocks(), 0);
}

#[test]
fn noop_does_not_affect_health_metrics() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    // Trigger WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    let consecutive_before = writer.internal_state().consecutive_would_blocks();
    let max_before = writer.internal_state().max_consecutive_would_blocks();

    // Noop should not change metrics
    writer.process_request(&WriteRequest::Noop).unwrap();

    assert_eq!(
        writer.internal_state().consecutive_would_blocks(),
        consecutive_before
    );
    assert_eq!(
        writer.internal_state().max_consecutive_would_blocks(),
        max_before
    );
}
