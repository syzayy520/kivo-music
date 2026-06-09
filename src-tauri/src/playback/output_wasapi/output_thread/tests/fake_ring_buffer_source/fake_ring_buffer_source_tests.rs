//! Fake ring buffer source tests.
//!
//! Tests for FakeRingBufferSource construction, snapshot, cursor, and basic properties.

use crate::playback::output_wasapi::output_thread::runtime::ring_buffer_render_source::FakeRingBufferSource;
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::RenderSource;

#[test]
fn fake_source_new() {
    let source = FakeRingBufferSource::new(10);
    assert!(source.is_ready());
    assert!(!source.is_exhausted());
    assert_eq!(source.buffer_len(), 0);
    assert_eq!(source.capacity(), 10);
}

#[test]
fn fake_source_empty() {
    let source = FakeRingBufferSource::empty();
    assert!(source.is_ready());
    assert!(!source.is_exhausted());
    assert_eq!(source.buffer_len(), 0);
}

#[test]
fn fake_source_with_test_packets() {
    let source = FakeRingBufferSource::with_test_packets(3, 512);
    assert!(source.is_ready());
    assert_eq!(source.buffer_len(), 3);
}

#[test]
fn fake_source_push_packet() {
    let mut source = FakeRingBufferSource::new(10);
    source.push_packet(100);
    assert_eq!(source.buffer_len(), 1);
    source.push_packet(200);
    assert_eq!(source.buffer_len(), 2);
}

#[test]
fn fake_source_push_eos() {
    let mut source = FakeRingBufferSource::new(10);
    source.push_packet(100);
    source.push_eos();
    assert_eq!(source.buffer_len(), 2);
    assert!(source.has_eos());
}

#[test]
fn fake_source_no_eos() {
    let mut source = FakeRingBufferSource::new(10);
    source.push_packet(100);
    source.push_packet(200);
    assert!(!source.has_eos());
}

#[test]
fn fake_source_capacity_limit() {
    let mut source = FakeRingBufferSource::new(2);
    source.push_packet(100);
    source.push_packet(200);
    source.push_packet(300); // Ignored (capacity reached)
    assert_eq!(source.buffer_len(), 2);
}

#[test]
fn fake_source_snapshot_default() {
    let source = FakeRingBufferSource::empty();
    let snapshot = source.snapshot();
    assert_eq!(snapshot.requests_accepted, 0);
    assert_eq!(snapshot.packets_provided, 0);
    assert_eq!(snapshot.frames_read, 0);
    assert_eq!(snapshot.bytes_read, 0);
    assert_eq!(snapshot.errors, 0);
    assert!(!snapshot.is_exhausted);
    assert!(snapshot.is_ready);
}

#[test]
fn fake_source_cursor_default() {
    let source = FakeRingBufferSource::empty();
    let cursor = source.cursor();
    assert_eq!(cursor.position_frames, 0);
    assert_eq!(cursor.total_frames, 0);
    assert_eq!(cursor.sample_rate, 44100);
    assert_eq!(cursor.channel_count, 2);
}

#[test]
fn fake_source_debug_format() {
    let source = FakeRingBufferSource::empty();
    let debug_str = format!("{:?}", source);
    assert!(debug_str.contains("FakeRingBufferSource"));
}
