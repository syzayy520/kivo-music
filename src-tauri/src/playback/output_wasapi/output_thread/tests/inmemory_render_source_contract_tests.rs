//! In-memory render source contract tests.
//!
//! Tests for InMemoryRenderSource construction, snapshot, cursor, and basic properties.

use crate::playback::output_wasapi::output_thread::runtime::inmemory_render_source::{
    InMemoryRenderSource, SourceQueue,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::RenderSource;

#[test]
fn inmemory_source_new_empty() {
    let source = InMemoryRenderSource::empty();
    assert!(source.is_ready());
    assert!(!source.is_exhausted());
    assert_eq!(source.queue_len(), 0);
}

#[test]
fn inmemory_source_new_with_queue() {
    let mut queue = SourceQueue::new();
    queue.push_new(100);
    queue.push_new(200);
    
    let source = InMemoryRenderSource::new(queue);
    assert!(source.is_ready());
    assert_eq!(source.queue_len(), 2);
}

#[test]
fn inmemory_source_with_test_packets() {
    let source = InMemoryRenderSource::with_test_packets(3, 512);
    assert!(source.is_ready());
    assert_eq!(source.queue_len(), 3);
}

#[test]
fn inmemory_source_snapshot_default() {
    let source = InMemoryRenderSource::empty();
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
fn inmemory_source_cursor_default() {
    let source = InMemoryRenderSource::empty();
    let cursor = source.cursor();
    
    assert_eq!(cursor.position_frames, 0);
    assert_eq!(cursor.total_frames, 0);
    assert_eq!(cursor.sample_rate, 44100);
    assert_eq!(cursor.channel_count, 2);
}

#[test]
fn inmemory_source_has_eos() {
    let mut queue = SourceQueue::new();
    queue.push_new(100);
    queue.push_eos();
    
    let source = InMemoryRenderSource::new(queue);
    assert!(source.has_eos());
}

#[test]
fn inmemory_source_no_eos() {
    let mut queue = SourceQueue::new();
    queue.push_new(100);
    queue.push_new(200);
    
    let source = InMemoryRenderSource::new(queue);
    assert!(!source.has_eos());
}

#[test]
fn inmemory_source_custom_sample_rate() {
    let source = InMemoryRenderSource::empty().with_sample_rate(48000);
    let cursor = source.cursor();
    assert_eq!(cursor.sample_rate, 48000);
}

#[test]
fn inmemory_source_custom_channel_count() {
    let source = InMemoryRenderSource::empty().with_channel_count(1);
    let cursor = source.cursor();
    assert_eq!(cursor.channel_count, 1);
}

#[test]
fn inmemory_source_debug_format() {
    let source = InMemoryRenderSource::empty();
    let debug_str = format!("{:?}", source);
    assert!(debug_str.contains("InMemoryRenderSource"));
}