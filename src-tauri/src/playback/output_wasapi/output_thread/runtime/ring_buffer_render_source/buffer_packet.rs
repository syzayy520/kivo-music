//! Ring buffer source packet metadata.
//!
//! Pure metadata type representing an audio packet in a ring buffer render source.
//! No actual PCM buffer — this is a contract shell for the ring buffer boundary.
//! No WASAPI, no IO, no NativePipeline.

use crate::playback::output_wasapi::output_thread::sink_boundary::render_packet::{
    PacketFormat, PacketTiming,
};

/// Metadata for a ring buffer audio packet.
///
/// Represents the metadata of an audio packet in a ring buffer source.
/// Contains no actual audio data — only the information needed to track
/// packet position and state within the ring buffer boundary.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BufferPacket {
    /// Unique identifier for this packet within the source.
    pub packet_id: u64,
    /// Number of frames in this packet.
    pub frame_count: u64,
    /// Audio format descriptor.
    pub format: PacketFormat,
    /// Timing information.
    pub timing: PacketTiming,
    /// Whether this packet represents end-of-stream.
    pub is_eos: bool,
    /// Write position in the ring buffer (frames from buffer start).
    pub buffer_write_position: u64,
    /// Sequence number for ordering in the ring buffer.
    pub sequence_number: u64,
}

impl BufferPacket {
    /// Creates a new buffer packet metadata with the given id and frame count.
    pub fn new(packet_id: u64, frame_count: u64) -> Self {
        Self {
            packet_id,
            frame_count,
            ..Default::default()
        }
    }

    /// Creates a new buffer packet with buffer position information.
    pub fn with_position(
        packet_id: u64,
        frame_count: u64,
        buffer_write_position: u64,
        sequence_number: u64,
    ) -> Self {
        Self {
            packet_id,
            frame_count,
            buffer_write_position,
            sequence_number,
            ..Default::default()
        }
    }

    /// Creates an end-of-stream buffer packet metadata.
    pub fn eos(packet_id: u64) -> Self {
        Self {
            packet_id,
            is_eos: true,
            ..Default::default()
        }
    }

    /// Returns true if this is an end-of-stream packet.
    pub fn is_end_of_stream(&self) -> bool {
        self.is_eos
    }

    /// Returns the number of bytes this packet would occupy (metadata only).
    pub fn byte_size(&self) -> u64 {
        // Each frame is 4 bytes per channel (f32 format)
        self.frame_count * self.format.channel_count as u64 * 4
    }

    /// Returns the end position in the buffer (exclusive).
    pub fn buffer_end_position(&self) -> u64 {
        self.buffer_write_position + self.frame_count
    }
}
