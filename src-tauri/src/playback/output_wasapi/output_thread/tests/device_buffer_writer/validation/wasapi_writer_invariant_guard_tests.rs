//! Invariant guard tests for WASAPI device buffer writer.
//!
//! Verifies runtime invariants:
//! 1. buffer_fill_frames <= capacity_frames
//! 2. write_head < capacity_frames (when capacity > 0)
//! 3. lifecycle matches actual state
//! 4. consecutive_would_blocks <= max_consecutive_would_blocks

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::wasapi_writer::BufferLifecycle;
use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig, WriteRequest,
};

#[test]
fn initial_state_invariants_hold() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    assert!(writer.internal_state().invariants_hold(1024));
}

#[test]
fn after_write_invariants_hold() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    assert!(writer.internal_state().invariants_hold(512));
}

#[test]
fn after_would_block_invariants_hold() {
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

    assert!(writer.internal_state().invariants_hold(256));
}

#[test]
fn after_flush_invariants_hold() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(128, 44100, 2))
        .unwrap();
    writer.process_request(&WriteRequest::Flush).unwrap();

    assert!(writer.internal_state().invariants_hold(256));
}

#[test]
fn after_close_invariants_hold() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(128, 44100, 2))
        .unwrap();
    writer.process_request(&WriteRequest::Close).unwrap();

    assert!(writer.internal_state().invariants_hold(256));
}

#[test]
fn after_reset_invariants_hold() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(128, 44100, 2))
        .unwrap();
    writer.reset();

    assert!(writer.internal_state().invariants_hold(256));
}

#[test]
fn lifecycle_matches_after_write() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Empty
    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Empty);

    // Partial
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    assert_eq!(
        writer.internal_state().lifecycle(),
        BufferLifecycle::Partial
    );

    // Full
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Full);
}

#[test]
fn lifecycle_matches_after_flush() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(128, 44100, 2))
        .unwrap();
    writer.process_request(&WriteRequest::Flush).unwrap();

    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Empty);
}

#[test]
fn lifecycle_matches_after_close() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer.process_request(&WriteRequest::Close).unwrap();

    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Closed);
}

#[test]
fn write_head_within_capacity() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write multiple times to cause wrap
    for _ in 0..5 {
        writer
            .process_request(&WriteRequest::write_packet(100, 44100, 2))
            .unwrap();
        writer.process_request(&WriteRequest::Flush).unwrap();
    }

    assert!(writer.internal_state().write_head() < 256);
}

#[test]
fn buffer_fill_within_capacity() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(200, 44100, 2))
        .unwrap();

    assert!(writer.internal_state().buffer_fill_frames() <= 256);
}

#[test]
fn consecutive_would_blocks_within_max() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    // Trigger multiple WouldBlocks
    for _ in 0..5 {
        writer
            .process_request(&WriteRequest::write_packet(1, 44100, 2))
            .unwrap();
    }

    assert!(
        writer.internal_state().consecutive_would_blocks()
            <= writer.internal_state().max_consecutive_would_blocks()
    );
}

#[test]
fn check_invariants_returns_ok_for_valid_state() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(128, 44100, 2))
        .unwrap();

    assert!(writer.internal_state().check_invariants(256).is_ok());
}

#[test]
fn invariants_hold_through_full_lifecycle() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Empty
    assert!(writer.internal_state().invariants_hold(512));

    // Partial
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    assert!(writer.internal_state().invariants_hold(512));

    // Full
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    assert!(writer.internal_state().invariants_hold(512));

    // WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    assert!(writer.internal_state().invariants_hold(512));

    // Flush
    writer.process_request(&WriteRequest::Flush).unwrap();
    assert!(writer.internal_state().invariants_hold(512));

    // Close
    writer.process_request(&WriteRequest::Close).unwrap();
    assert!(writer.internal_state().invariants_hold(512));
}
