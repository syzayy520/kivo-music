//! Buffer lifecycle tests for WASAPI device buffer writer.
//!
//! Verifies buffer lifecycle state tracking:
//! 1. Initial state is Empty
//! 2. Partial fill transitions to Partial
//! 3. Full buffer transitions to Full
//! 4. Flush transitions back to Empty
//! 5. Close transitions to Closed
//! 6. Lifecycle helpers work correctly

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::wasapi_writer::BufferLifecycle;
use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig, WriteRequest,
};

#[test]
fn initial_lifecycle_is_empty() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Empty);
}

#[test]
fn partial_fill_transitions_to_partial() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    assert_eq!(
        writer.internal_state().lifecycle(),
        BufferLifecycle::Partial
    );
}

#[test]
fn full_buffer_transitions_to_full() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Full);
}

#[test]
fn flush_transitions_back_to_empty() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill partially
    writer
        .process_request(&WriteRequest::write_packet(128, 44100, 2))
        .unwrap();
    assert_eq!(
        writer.internal_state().lifecycle(),
        BufferLifecycle::Partial
    );

    // Flush
    writer.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Empty);
}

#[test]
fn close_transitions_to_closed() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    writer.process_request(&WriteRequest::Close).unwrap();

    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Closed);
}

#[test]
fn close_from_partial_transitions_to_closed() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill partially
    writer
        .process_request(&WriteRequest::write_packet(128, 44100, 2))
        .unwrap();

    // Close
    writer.process_request(&WriteRequest::Close).unwrap();

    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Closed);
}

#[test]
fn lifecycle_can_accept_frames_for_empty() {
    assert!(BufferLifecycle::Empty.can_accept_frames());
}

#[test]
fn lifecycle_can_accept_frames_for_partial() {
    assert!(BufferLifecycle::Partial.can_accept_frames());
}

#[test]
fn lifecycle_cannot_accept_frames_for_full() {
    assert!(!BufferLifecycle::Full.can_accept_frames());
}

#[test]
fn lifecycle_cannot_accept_frames_for_closed() {
    assert!(!BufferLifecycle::Closed.can_accept_frames());
}

#[test]
fn lifecycle_is_closed_for_closed() {
    assert!(BufferLifecycle::Closed.is_closed());
}

#[test]
fn lifecycle_is_not_closed_for_others() {
    assert!(!BufferLifecycle::Empty.is_closed());
    assert!(!BufferLifecycle::Partial.is_closed());
    assert!(!BufferLifecycle::Full.is_closed());
}

#[test]
fn lifecycle_is_full_for_full() {
    assert!(BufferLifecycle::Full.is_full());
}

#[test]
fn lifecycle_is_not_full_for_others() {
    assert!(!BufferLifecycle::Empty.is_full());
    assert!(!BufferLifecycle::Partial.is_full());
    assert!(!BufferLifecycle::Closed.is_full());
}

#[test]
fn would_block_does_not_change_lifecycle() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Full);

    // WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    // Still Full
    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Full);
}

#[test]
fn noop_does_not_change_lifecycle() {
    let mut writer = WasapiDeviceBufferWriter::with_defaults();

    writer.process_request(&WriteRequest::Noop).unwrap();

    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Empty);
}

#[test]
fn reset_restores_lifecycle_to_empty() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill and close
    writer
        .process_request(&WriteRequest::write_packet(128, 44100, 2))
        .unwrap();
    writer.process_request(&WriteRequest::Close).unwrap();
    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Closed);

    // Reset
    writer.reset();

    assert_eq!(writer.internal_state().lifecycle(), BufferLifecycle::Empty);
}
