//! Cross-boundary contract tests for DeviceBufferWriter ↔ RenderClientBoundary.
//!
//! Verifies that WasapiDeviceBufferWriter correctly delegates buffer operations
//! to an attached RenderClientBoundary (FakeRenderClientBoundary) and that
//! error propagation, state tracking, and lifecycle semantics are preserved
//! across the boundary.

use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::wasapi_writer::{
    WasapiDeviceBufferWriter, WasapiDeviceBufferWriterConfig,
};
use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::{
    DeviceBufferWriter, WriteError, WriteRequest, WriteResult,
};
use crate::playback::output_wasapi::render_client_boundary::{
    FakeRenderClientBoundary, RenderClientFailure,
};

/// Helper: create a WasapiDeviceBufferWriter with a FakeRenderClientBoundary.
///
/// Writer capacity: 100 frames, render client capacity: 100 frames.
fn writer_with_fake_client(capacity: u32) -> WasapiDeviceBufferWriter {
    let client = FakeRenderClientBoundary::new(capacity);
    WasapiDeviceBufferWriter::with_render_client(
        WasapiDeviceBufferWriterConfig::new(capacity as u64, 2, 44100),
        Box::new(client),
    )
}

/// Helper: create a WasapiDeviceBufferWriter without render client (simulated mode).
fn writer_without_client(capacity: u64) -> WasapiDeviceBufferWriter {
    WasapiDeviceBufferWriter::new(WasapiDeviceBufferWriterConfig::new(capacity, 2, 44100))
}

// ── Delegation: WritePacket delegates to render client ────────────────────

#[test]
fn write_packet_delegates_to_render_client_returns_written() {
    let mut w = writer_with_fake_client(100);
    let result = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap();
    assert!(result.is_written());
    assert_eq!(result.frames_written(), 10);
}

#[test]
fn write_packet_delegation_updates_local_state() {
    let mut w = writer_with_fake_client(100);
    w.process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap();

    let snap = w.snapshot();
    assert_eq!(snap.frames_written, 10);
    assert!(snap.bytes_written > 0);
    assert_eq!(snap.requests_accepted, 1);
    assert_eq!(snap.would_block_count, 0);
}

#[test]
fn write_packet_delegation_advances_cursor() {
    let mut w = writer_with_fake_client(100);
    w.process_request(&WriteRequest::write_packet(25, 44100, 2))
        .unwrap();

    let cursor = w.cursor();
    assert_eq!(cursor.write_position, 25);
    assert_eq!(cursor.total_frames_written, 25);
}

// ── Delegation: WouldBlock when render client buffer too small ────────────

#[test]
fn write_packet_would_block_when_render_client_buffer_full() {
    let mut w = writer_with_fake_client(100);
    // Fill the render client buffer
    w.process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    // Next write should WouldBlock (no available frames in render client)
    let result = w
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    assert_eq!(result, WriteResult::WouldBlock);
}

#[test]
fn write_packet_would_block_updates_would_block_count() {
    let mut w = writer_with_fake_client(100);
    w.process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    // Trigger WouldBlock
    w.process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    let snap = w.snapshot();
    assert_eq!(snap.would_block_count, 1);
    assert_eq!(snap.consecutive_would_blocks, 1);
}

#[test]
fn write_packet_would_block_resets_write_streak() {
    let mut w = writer_with_fake_client(200);
    // Build a streak of 3
    w.process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    w.process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    w.process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    assert_eq!(w.snapshot().write_streak, 3);

    // Fill remaining (200 - 150 = 50 available)
    w.process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    // Now buffer is full — next write WouldBlock
    w.process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    assert_eq!(w.snapshot().write_streak, 0);
}

// ── Delegation: Flush with render client ──────────────────────────────────

#[test]
fn flush_with_render_client_resets_local_state() {
    let mut w = writer_with_fake_client(100);
    w.process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    assert!(w.snapshot().buffer_fill_frames > 0);

    w.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(w.snapshot().buffer_fill_frames, 0);
    assert_eq!(w.snapshot().lifecycle, BufferLifecycle::Empty);
}

#[test]
fn flush_with_render_client_resets_consecutive_would_blocks() {
    let mut w = writer_with_fake_client(100);
    // Fill buffer and trigger WouldBlock
    w.process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    w.process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    assert_eq!(w.snapshot().consecutive_would_blocks, 1);

    // Flush should reset stall state
    w.process_request(&WriteRequest::Flush).unwrap();
    assert_eq!(w.snapshot().consecutive_would_blocks, 0);
}

// ── Delegation: Close with render client ──────────────────────────────────

#[test]
fn close_with_render_client_returns_noop() {
    let mut w = writer_with_fake_client(100);
    let result = w.process_request(&WriteRequest::Close).unwrap();
    assert_eq!(result, WriteResult::Noop);
}

#[test]
fn close_with_render_client_then_write_returns_device_closed() {
    let mut w = writer_with_fake_client(100);
    w.process_request(&WriteRequest::Close).unwrap();
    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    assert_eq!(err, WriteError::DeviceClosed);
}

// ── Delegation: Reset preserves render client ─────────────────────────────

#[test]
fn reset_preserves_render_client() {
    let mut w = writer_with_fake_client(100);
    assert!(w.has_render_client());

    w.process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    w.reset();

    // State is reset but render client is preserved
    assert!(w.has_render_client());
    assert_eq!(w.snapshot().buffer_fill_frames, 0);
    assert_eq!(w.snapshot().frames_written, 0);
}

// ── Delegation: is_ready checks render client ─────────────────────────────

#[test]
fn is_ready_with_ready_render_client() {
    let w = writer_with_fake_client(100);
    assert!(w.is_ready());
}

#[test]
fn is_ready_with_unready_render_client() {
    let mut client = FakeRenderClientBoundary::new(100);
    client.set_ready(false);
    let w = WasapiDeviceBufferWriter::with_render_client(
        WasapiDeviceBufferWriterConfig::new(100, 2, 44100),
        Box::new(client),
    );
    assert!(!w.is_ready());
}

#[test]
fn is_ready_after_close_with_render_client() {
    let mut w = writer_with_fake_client(100);
    w.process_request(&WriteRequest::Close).unwrap();
    assert!(!w.is_ready());
}

// ── Delegation: Error propagation ─────────────────────────────────────────

#[test]
fn render_client_device_lost_propagates_as_write_failed() {
    let mut client = FakeRenderClientBoundary::new(100);
    client.set_failure_mode(Some(RenderClientFailure::DeviceLost));
    let mut w = WasapiDeviceBufferWriter::with_render_client(
        WasapiDeviceBufferWriterConfig::new(100, 2, 44100),
        Box::new(client),
    );

    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    assert!(err.is_write_failed());
}

#[test]
fn render_client_unavailable_propagates_as_write_failed() {
    let mut client = FakeRenderClientBoundary::new(100);
    client.set_failure_mode(Some(RenderClientFailure::RenderClientUnavailable));
    let mut w = WasapiDeviceBufferWriter::with_render_client(
        WasapiDeviceBufferWriterConfig::new(100, 2, 44100),
        Box::new(client),
    );

    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    assert!(err.is_write_failed());
}

#[test]
fn render_client_invalid_request_propagates_as_invalid_request() {
    let mut client = FakeRenderClientBoundary::new(100);
    client.set_failure_mode(Some(RenderClientFailure::InvalidRequest {
        reason: "test".to_string(),
    }));
    let mut w = WasapiDeviceBufferWriter::with_render_client(
        WasapiDeviceBufferWriterConfig::new(100, 2, 44100),
        Box::new(client),
    );

    let err = w
        .process_request(&WriteRequest::write_packet(10, 44100, 2))
        .unwrap_err();
    assert!(err.is_invalid_request());
}

// ── Behavioral parity: delegated vs simulated ─────────────────────────────

#[test]
fn delegated_write_matches_simulated_for_basic_write() {
    let mut delegated = writer_with_fake_client(100);
    let mut simulated = writer_without_client(100);

    let req = WriteRequest::write_packet(10, 44100, 2);
    let d_result = delegated.process_request(&req).unwrap();
    let s_result = simulated.process_request(&req).unwrap();

    assert_eq!(d_result, s_result);
    assert_eq!(
        delegated.snapshot().frames_written,
        simulated.snapshot().frames_written
    );
    assert_eq!(
        delegated.snapshot().bytes_written,
        simulated.snapshot().bytes_written
    );
}

#[test]
fn delegated_would_block_matches_simulated() {
    let mut delegated = writer_with_fake_client(100);
    let mut simulated = writer_without_client(100);

    // Fill both to capacity
    delegated
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    simulated
        .process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();

    // Both should WouldBlock
    let d_result = delegated
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();
    let s_result = simulated
        .process_request(&WriteRequest::write_packet(1, 44100, 2))
        .unwrap();

    assert_eq!(d_result, WriteResult::WouldBlock);
    assert_eq!(d_result, s_result);
}

#[test]
fn delegated_flush_matches_simulated() {
    let mut delegated = writer_with_fake_client(100);
    let mut simulated = writer_without_client(100);

    // Write, then flush
    delegated
        .process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    simulated
        .process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();

    delegated.process_request(&WriteRequest::Flush).unwrap();
    simulated.process_request(&WriteRequest::Flush).unwrap();

    assert_eq!(
        delegated.snapshot().buffer_fill_frames,
        simulated.snapshot().buffer_fill_frames
    );
    assert_eq!(
        delegated.snapshot().lifecycle,
        simulated.snapshot().lifecycle
    );
}

// ── Write streak tracking with delegation ─────────────────────────────────

#[test]
fn delegated_write_streak_tracks_consecutive_writes() {
    let mut w = writer_with_fake_client(500);
    for _ in 0..5 {
        w.process_request(&WriteRequest::write_packet(50, 44100, 2))
            .unwrap();
    }

    let snap = w.snapshot();
    assert_eq!(snap.write_streak, 5);
    assert_eq!(snap.max_write_streak, 5);
    assert_eq!(snap.frames_written, 250);
}

// ── Snapshot lifecycle with delegation ────────────────────────────────────

#[test]
fn delegated_lifecycle_transitions_empty_to_partial() {
    let mut w = writer_with_fake_client(100);
    assert_eq!(w.snapshot().lifecycle, BufferLifecycle::Empty);

    w.process_request(&WriteRequest::write_packet(50, 44100, 2))
        .unwrap();
    assert_eq!(w.snapshot().lifecycle, BufferLifecycle::Partial);
}

#[test]
fn delegated_lifecycle_transitions_partial_to_full() {
    let mut w = writer_with_fake_client(100);
    w.process_request(&WriteRequest::write_packet(100, 44100, 2))
        .unwrap();
    assert_eq!(w.snapshot().lifecycle, BufferLifecycle::Full);
}

#[test]
fn delegated_lifecycle_transitions_to_closed() {
    let mut w = writer_with_fake_client(100);
    w.process_request(&WriteRequest::Close).unwrap();
    assert_eq!(w.snapshot().lifecycle, BufferLifecycle::Closed);
}

// ── Import for BufferLifecycle ────────────────────────────────────────────
use crate::playback::output_wasapi::output_thread::runtime::device_buffer_writer::BufferLifecycle;
