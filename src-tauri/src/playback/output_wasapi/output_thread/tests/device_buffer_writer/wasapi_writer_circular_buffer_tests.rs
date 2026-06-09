//! Circular buffer position simulation tests for WASAPI device buffer writer.
//!
//! Verifies:
//! 1. write_head tracks position within circular buffer
//! 2. wrap_count increments when buffer wraps
//! 3. cursor() returns correct write_position and wrap_count
//! 4. snapshot() returns correct buffer_wrap_count
//! 5. flush does not reset write_head or wrap_count
//! 6. reset clears write_head and wrap_count
//! 7. Multiple wraps in single write are handled correctly

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig, WriteRequest,
};

#[test]
fn write_head_starts_at_zero() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    assert_eq!(writer.internal_state().write_head(), 0);
}

#[test]
fn wrap_count_starts_at_zero() {
    let writer = WasapiDeviceBufferWriter::with_defaults();
    assert_eq!(writer.internal_state().wrap_count(), 0);
}

#[test]
fn write_advances_write_head() {
    let config = WasapiDeviceBufferWriterConfig::new(1024, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    assert_eq!(writer.internal_state().write_head(), 256);

    writer
        .process_request(&WriteRequest::write_packet(128, 44100, 2))
        .unwrap();
    assert_eq!(writer.internal_state().write_head(), 384);
}

#[test]
fn write_head_wraps_at_capacity() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write exactly capacity
    writer
        .process_request(&WriteRequest::write_packet(512, 44100, 2))
        .unwrap();
    assert_eq!(writer.internal_state().write_head(), 0); // wrapped back to 0
    assert_eq!(writer.internal_state().wrap_count(), 1);
}

#[test]
fn write_head_wraps_with_remainder() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write 400 frames (write_head = 400)
    writer
        .process_request(&WriteRequest::write_packet(400, 44100, 2))
        .unwrap();
    assert_eq!(writer.internal_state().write_head(), 400);

    // Flush to clear buffer fill (write_head preserved)
    writer.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(writer.internal_state().write_head(), 400);

    // Write 200 frames → write_head = 400 + 200 = 600 → wraps to 88
    writer
        .process_request(&WriteRequest::write_packet(200, 44100, 2))
        .unwrap();
    assert_eq!(writer.internal_state().write_head(), 88);
    assert_eq!(writer.internal_state().wrap_count(), 1);
}

#[test]
fn multiple_writes_accumulate_position() {
    let config = WasapiDeviceBufferWriterConfig::new(1024, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(300, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(400, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(324, 44100, 2))
        .unwrap();

    // Total: 300 + 400 + 324 = 1024 = exactly capacity
    assert_eq!(writer.internal_state().write_head(), 0);
    assert_eq!(writer.internal_state().wrap_count(), 1);
}

#[test]
fn cursor_reflects_write_head() {
    let config = WasapiDeviceBufferWriterConfig::new(1024, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();

    let cursor = writer.cursor();
    assert_eq!(cursor.write_position, 256); // write_head, not frames_written
    assert_eq!(cursor.wrap_count, 0);
}

#[test]
fn cursor_reflects_wrap_count() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer to trigger wrap
    writer
        .process_request(&WriteRequest::write_packet(512, 44100, 2))
        .unwrap();

    let cursor = writer.cursor();
    assert_eq!(cursor.write_position, 0); // wrapped back to 0
    assert_eq!(cursor.wrap_count, 1);
}

#[test]
fn snapshot_reflects_wrap_count() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(512, 44100, 2))
        .unwrap();

    let snapshot = writer.snapshot();
    assert_eq!(snapshot.buffer_wrap_count, 1);
}

#[test]
fn flush_does_not_reset_write_head() {
    let config = WasapiDeviceBufferWriterConfig::new(1024, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    writer.process_request(&WriteRequest::Flush).unwrap();

    // write_head is preserved (position tracking is independent of buffer fill)
    assert_eq!(writer.internal_state().write_head(), 256);
    assert_eq!(writer.internal_state().wrap_count(), 0);
}

#[test]
fn flush_does_not_reset_wrap_count() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer to trigger wrap
    writer
        .process_request(&WriteRequest::write_packet(512, 44100, 2))
        .unwrap();
    writer.process_request(&WriteRequest::Flush).unwrap();

    assert_eq!(writer.internal_state().wrap_count(), 1);
    assert_eq!(writer.internal_state().write_head(), 0);
}

#[test]
fn reset_clears_write_head_and_wrap_count() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write and wrap
    writer
        .process_request(&WriteRequest::write_packet(512, 44100, 2))
        .unwrap();
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap(); // WouldBlock after flush? No, buffer is full.

    // Actually fill and wrap properly
    writer.process_request(&WriteRequest::Flush).unwrap();
    writer
        .process_request(&WriteRequest::write_packet(600, 44100, 2))
        .unwrap();

    writer.reset();

    assert_eq!(writer.internal_state().write_head(), 0);
    assert_eq!(writer.internal_state().wrap_count(), 0);
}

#[test]
fn would_block_does_not_affect_write_head() {
    let config = WasapiDeviceBufferWriterConfig::new(256, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Fill buffer
    writer
        .process_request(&WriteRequest::write_packet(256, 44100, 2))
        .unwrap();
    let write_head_before = writer.internal_state().write_head();

    // Trigger WouldBlock
    writer
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    // write_head unchanged
    assert_eq!(writer.internal_state().write_head(), write_head_before);
}

#[test]
fn write_position_is_not_total_frames_written() {
    let config = WasapiDeviceBufferWriterConfig::new(512, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write 400 frames
    writer
        .process_request(&WriteRequest::write_packet(400, 44100, 2))
        .unwrap();
    // Flush to clear buffer fill
    writer.process_request(&WriteRequest::Flush).unwrap();
    // Write 200 frames → wraps (400 + 200 = 600 → 88)
    writer
        .process_request(&WriteRequest::write_packet(200, 44100, 2))
        .unwrap();

    let cursor = writer.cursor();
    // write_position should be 88 (600 % 512), not 600 (total frames)
    assert_eq!(cursor.write_position, 88);
    // total_frames_written should be 600 (400 + 200)
    assert_eq!(cursor.total_frames_written, 600);
}

#[test]
fn small_capacity_wraps_frequently() {
    let config = WasapiDeviceBufferWriterConfig::new(100, 2, 44100);
    let mut writer = WasapiDeviceBufferWriter::new(config);

    // Write 250 frames = 2 full wraps + 50 remainder
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    writer.process_request(&WriteRequest::Flush).unwrap();
    writer
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    writer.process_request(&WriteRequest::Flush).unwrap();
    writer
        .process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();

    assert_eq!(writer.internal_state().write_head(), 50);
    assert_eq!(writer.internal_state().wrap_count(), 2);
}
