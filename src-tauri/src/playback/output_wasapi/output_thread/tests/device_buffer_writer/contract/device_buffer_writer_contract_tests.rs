//! Cross-implementation behavioral contract tests.
//!
//! Verifies that both FakeDeviceBufferWriter and WasapiDeviceBufferWriter
//! satisfy the same behavioral contract for shared operations.
//! Ensures the DeviceBufferWriter trait has meaningful, consistent semantics.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, FakeDeviceBufferWriter, WasapiDeviceBufferWriter,
    WasapiDeviceBufferWriterConfig, WriteError, WriteRequest, WriteResult,
};

/// Helper: create a FakeDeviceBufferWriter with capacity 100.
fn fake_writer() -> FakeDeviceBufferWriter {
    FakeDeviceBufferWriter::new(100)
}

/// Helper: create a WasapiDeviceBufferWriter with capacity 100.
fn wasapi_writer() -> WasapiDeviceBufferWriter {
    WasapiDeviceBufferWriter::new(WasapiDeviceBufferWriterConfig::new(100, 2, 44100))
}

// ── Close → DeviceClosed on subsequent requests ─────────────────────────

#[test]
fn fake_close_then_write_returns_device_closed() {
    let mut w = fake_writer();
    w.process_request(&WriteRequest::Close).unwrap();
    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    assert_eq!(err, WriteError::DeviceClosed);
}

#[test]
fn wasapi_close_then_write_returns_device_closed() {
    let mut w = wasapi_writer();
    w.process_request(&WriteRequest::Close).unwrap();
    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    assert_eq!(err, WriteError::DeviceClosed);
}

// ── Flush resets buffer fill ────────────────────────────────────────────

#[test]
fn fake_flush_resets_buffer_fill() {
    let mut w = fake_writer();
    w.process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    assert!(w.snapshot().buffer_fill_frames > 0);
    w.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(w.snapshot().buffer_fill_frames, 0);
}

#[test]
fn wasapi_flush_resets_buffer_fill() {
    let mut w = wasapi_writer();
    w.process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    assert!(w.snapshot().buffer_fill_frames > 0);
    w.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(w.snapshot().buffer_fill_frames, 0);
}

// ── Noop returns Noop ──────────────────────────────────────────────────

#[test]
fn fake_noop_returns_noop() {
    let mut w = fake_writer();
    let result = w.process_request(&WriteRequest::Noop).unwrap();
    assert_eq!(result, WriteResult::Noop);
}

#[test]
fn wasapi_noop_returns_noop() {
    let mut w = wasapi_writer();
    let result = w.process_request(&WriteRequest::Noop).unwrap();
    assert_eq!(result, WriteResult::Noop);
}

// ── WritePacket returns Written when capacity available ─────────────────

#[test]
fn fake_write_packet_returns_written() {
    let mut w = fake_writer();
    let result = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap();
    assert!(result.is_written());
    assert_eq!(result.frames_written(), 10);
}

#[test]
fn wasapi_write_packet_returns_written() {
    let mut w = wasapi_writer();
    let result = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap();
    assert!(result.is_written());
    assert_eq!(result.frames_written(), 10);
}

// ── WritePacket returns WouldBlock when buffer full ─────────────────────

#[test]
fn fake_write_when_full_returns_would_block() {
    let mut w = fake_writer();
    // Fill buffer to capacity
    w.process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    // Next write should WouldBlock
    let result = w
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    assert_eq!(result, WriteResult::WouldBlock);
}

#[test]
fn wasapi_write_when_full_returns_would_block() {
    let mut w = wasapi_writer();
    // Fill buffer to capacity
    w.process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    // Next write should WouldBlock
    let result = w.process_request(&WriteRequest::write_packet(1, 44100, 2));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), WriteResult::WouldBlock);
}

// ── Reset restores initial state ────────────────────────────────────────

#[test]
fn fake_reset_restores_initial_state() {
    let mut w = fake_writer();
    w.process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    w.process_request(&WriteRequest::Close).unwrap();
    assert!(w.is_closed());

    w.reset();
    assert!(!w.is_closed());
    assert!(w.is_ready());
    assert_eq!(w.snapshot().buffer_fill_frames, 0);
    assert_eq!(w.snapshot().frames_written, 0);
}

#[test]
fn wasapi_reset_restores_initial_state() {
    let mut w = wasapi_writer();
    w.process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    w.process_request(&WriteRequest::Close).unwrap();
    assert!(w.is_closed());

    w.reset();
    assert!(!w.is_closed());
    assert!(w.is_ready());
    assert_eq!(w.snapshot().buffer_fill_frames, 0);
    assert_eq!(w.snapshot().frames_written, 0);
}

// ── Snapshot tracks request count ───────────────────────────────────────

#[test]
fn fake_snapshot_tracks_request_count() {
    let mut w = fake_writer();
    w.process_request(&WriteRequest::Noop).unwrap();
    w.process_request(&WriteRequest::Flush).unwrap();
    w.process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap();
    assert_eq!(w.snapshot().requests_accepted, 3);
}

#[test]
fn wasapi_snapshot_tracks_write_attempts() {
    let mut w = wasapi_writer();
    w.process_request(&WriteRequest::Noop).unwrap();
    w.process_request(&WriteRequest::Flush).unwrap();
    w.process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap();
    // WasapiDeviceBufferWriter counts write_attempts (WritePacket only)
    assert_eq!(w.snapshot().requests_accepted, 1);
}

// ── Cursor tracks write position ────────────────────────────────────────

#[test]
fn fake_cursor_advances_on_write() {
    let mut w = fake_writer();
    w.process_request(&WriteRequest::write_packet(25, 44100, 2))
        .unwrap();
    let cursor = w.cursor();
    assert_eq!(cursor.write_position, 25);
    assert_eq!(cursor.total_frames_written, 25);
}

#[test]
fn wasapi_cursor_advances_on_write() {
    let mut w = wasapi_writer();
    w.process_request(&WriteRequest::write_packet(25, 44100, 2))
        .unwrap();
    let cursor = w.cursor();
    assert_eq!(cursor.write_position, 25);
    assert_eq!(cursor.total_frames_written, 25);
}

// ── Snapshot lifecycle transitions ──────────────────────────────────────

#[test]
fn fake_lifecycle_empty_after_flush() {
    let mut w = fake_writer();
    w.process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap();
    w.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(w.snapshot().lifecycle, BufferLifecycle::Empty);
}

#[test]
fn wasapi_lifecycle_empty_after_flush() {
    let mut w = wasapi_writer();
    w.process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap();
    w.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(w.snapshot().lifecycle, BufferLifecycle::Empty);
}

// ── Import for BufferLifecycle ──────────────────────────────────────────
use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::BufferLifecycle;
