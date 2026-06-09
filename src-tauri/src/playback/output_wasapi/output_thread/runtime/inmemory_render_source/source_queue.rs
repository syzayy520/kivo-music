//! In-memory source queue management.
//!
//! Simple queue for managing SourcePacketMetadata instances.
//! Uses std::collections::VecDeque. No threads, no channels.

use std::collections::VecDeque;

use super::source_packet::SourcePacketMetadata;

/// Queue for managing in-memory source packets.
///
/// Provides FIFO access to source packet metadata.
/// No actual audio data — only metadata for testing.
#[derive(Debug, Clone, Default)]
pub struct SourceQueue {
    /// Internal queue storage.
    queue: VecDeque<SourcePacketMetadata>,
    /// Total packets added (for generating IDs).
    next_packet_id: u64,
}

impl SourceQueue {
    /// Creates a new empty source queue.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            next_packet_id: 0,
        }
    }

    /// Creates a new queue with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            next_packet_id: 0,
        }
    }

    /// Adds a packet metadata to the back of the queue.
    pub fn push(&mut self, packet: SourcePacketMetadata) {
        self.queue.push_back(packet);
    }

    /// Adds a new packet with the given frame count and returns its ID.
    pub fn push_new(&mut self, frame_count: u64) -> u64 {
        let packet_id = self.next_packet_id;
        self.next_packet_id += 1;
        let packet = SourcePacketMetadata::new(packet_id, frame_count);
        self.queue.push_back(packet);
        packet_id
    }

    /// Adds an end-of-stream packet.
    pub fn push_eos(&mut self) -> u64 {
        let packet_id = self.next_packet_id;
        self.next_packet_id += 1;
        let packet = SourcePacketMetadata::eos(packet_id);
        self.queue.push_back(packet);
        packet_id
    }

    /// Removes and returns the front packet.
    pub fn pop(&mut self) -> Option<SourcePacketMetadata> {
        self.queue.pop_front()
    }

    /// Returns a reference to the front packet without removing it.
    pub fn peek(&self) -> Option<&SourcePacketMetadata> {
        self.queue.front()
    }

    /// Returns true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Returns the number of packets in the queue.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Removes all packets from the queue.
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// Returns the total number of frames in all packets.
    pub fn total_frames(&self) -> u64 {
        self.queue.iter().map(|p| p.frame_count).sum()
    }

    /// Returns true if any packet in the queue is end-of-stream.
    pub fn has_eos(&self) -> bool {
        self.queue.iter().any(|p| p.is_eos)
    }

    /// Returns the next packet ID that would be assigned.
    pub fn next_packet_id(&self) -> u64 {
        self.next_packet_id
    }
}
