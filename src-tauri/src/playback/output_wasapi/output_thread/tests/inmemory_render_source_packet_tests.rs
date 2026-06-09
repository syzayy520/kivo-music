//! In-memory source packet tests.
//!
//! Tests for SourcePacketMetadata and SourceQueue types.

use crate::playback::output_wasapi::output_thread::runtime::inmemory_render_source::{
    SourcePacketMetadata, SourceQueue,
};

#[test]
fn source_packet_metadata_new() {
    let packet = SourcePacketMetadata::new(1, 1024);
    assert_eq!(packet.packet_id, 1);
    assert_eq!(packet.frame_count, 1024);
    assert!(!packet.is_eos);
}

#[test]
fn source_packet_metadata_eos() {
    let packet = SourcePacketMetadata::eos(5);
    assert_eq!(packet.packet_id, 5);
    assert_eq!(packet.frame_count, 0);
    assert!(packet.is_eos);
}

#[test]
fn source_packet_metadata_is_end_of_stream() {
    let packet1 = SourcePacketMetadata::new(1, 100);
    let packet2 = SourcePacketMetadata::eos(2);
    assert!(!packet1.is_end_of_stream());
    assert!(packet2.is_end_of_stream());
}

#[test]
fn source_packet_metadata_byte_size() {
    let packet = SourcePacketMetadata::new(1, 1024);
    // Default format: 2 channels, 4 bytes per sample (f32)
    let expected = 1024 * 2 * 4;
    assert_eq!(packet.byte_size(), expected);
}

#[test]
fn source_packet_metadata_default() {
    let packet = SourcePacketMetadata::default();
    assert_eq!(packet.packet_id, 0);
    assert_eq!(packet.frame_count, 0);
    assert!(!packet.is_eos);
}

#[test]
fn source_packet_metadata_clone_eq() {
    let packet1 = SourcePacketMetadata::new(1, 512);
    let packet2 = packet1.clone();
    assert_eq!(packet1, packet2);
}

#[test]
fn source_queue_new() {
    let queue = SourceQueue::new();
    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);
    assert_eq!(queue.total_frames(), 0);
    assert!(!queue.has_eos());
}

#[test]
fn source_queue_with_capacity() {
    let queue = SourceQueue::with_capacity(10);
    assert!(queue.is_empty());
}

#[test]
fn source_queue_push_pop() {
    let mut queue = SourceQueue::new();
    let packet = SourcePacketMetadata::new(1, 100);
    queue.push(packet.clone());

    assert_eq!(queue.len(), 1);
    assert!(!queue.is_empty());

    let popped = queue.pop().unwrap();
    assert_eq!(popped, packet);
    assert!(queue.is_empty());
}

#[test]
fn source_queue_push_new() {
    let mut queue = SourceQueue::new();
    let id1 = queue.push_new(100);
    let id2 = queue.push_new(200);

    assert_eq!(id1, 0);
    assert_eq!(id2, 1);
    assert_eq!(queue.len(), 2);
    assert_eq!(queue.total_frames(), 300);
}

#[test]
fn source_queue_push_eos() {
    let mut queue = SourceQueue::new();
    queue.push_new(100);
    let eos_id = queue.push_eos();

    assert_eq!(eos_id, 1);
    assert!(queue.has_eos());

    // First pop should return the normal packet
    let normal_packet = queue.pop().unwrap();
    assert!(!normal_packet.is_eos);
    assert_eq!(normal_packet.packet_id, 0);

    // Second pop should return the EOS packet
    let eos_packet = queue.pop().unwrap();
    assert!(eos_packet.is_eos);
    assert_eq!(eos_packet.packet_id, 1);
}

#[test]
fn source_queue_peek() {
    let mut queue = SourceQueue::new();
    let packet = SourcePacketMetadata::new(1, 100);
    queue.push(packet.clone());

    let peeked = queue.peek().unwrap();
    assert_eq!(peeked, &packet);
    assert_eq!(queue.len(), 1); // peek doesn't remove
}

#[test]
fn source_queue_peek_empty() {
    let queue = SourceQueue::new();
    assert!(queue.peek().is_none());
}

#[test]
fn source_queue_pop_empty() {
    let mut queue = SourceQueue::new();
    assert!(queue.pop().is_none());
}

#[test]
fn source_queue_clear() {
    let mut queue = SourceQueue::new();
    queue.push_new(100);
    queue.push_new(200);

    assert_eq!(queue.len(), 2);
    queue.clear();
    assert!(queue.is_empty());
}

#[test]
fn source_queue_total_frames() {
    let mut queue = SourceQueue::new();
    queue.push_new(100);
    queue.push_new(200);
    queue.push_new(300);

    assert_eq!(queue.total_frames(), 600);
}

#[test]
fn source_queue_has_eos_false() {
    let mut queue = SourceQueue::new();
    queue.push_new(100);
    queue.push_new(200);

    assert!(!queue.has_eos());
}

#[test]
fn source_queue_has_eos_true() {
    let mut queue = SourceQueue::new();
    queue.push_new(100);
    queue.push_eos();

    assert!(queue.has_eos());
}

#[test]
fn source_queue_next_packet_id() {
    let mut queue = SourceQueue::new();
    assert_eq!(queue.next_packet_id(), 0);

    queue.push_new(100);
    assert_eq!(queue.next_packet_id(), 1);

    queue.push_new(200);
    assert_eq!(queue.next_packet_id(), 2);
}

#[test]
fn source_queue_fifo_order() {
    let mut queue = SourceQueue::new();
    let packet1 = SourcePacketMetadata::new(1, 100);
    let packet2 = SourcePacketMetadata::new(2, 200);
    let packet3 = SourcePacketMetadata::new(3, 300);

    queue.push(packet1.clone());
    queue.push(packet2.clone());
    queue.push(packet3.clone());

    assert_eq!(queue.pop().unwrap(), packet1);
    assert_eq!(queue.pop().unwrap(), packet2);
    assert_eq!(queue.pop().unwrap(), packet3);
}

#[test]
fn source_queue_clone_eq() {
    let mut queue1 = SourceQueue::new();
    queue1.push_new(100);
    queue1.push_new(200);

    let queue2 = queue1.clone();
    assert_eq!(queue1.len(), queue2.len());
    assert_eq!(queue1.total_frames(), queue2.total_frames());
}
