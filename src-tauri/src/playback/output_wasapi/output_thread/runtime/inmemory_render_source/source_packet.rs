//! In-memory source packet metadata.
//!
//! Pure metadata type representing an audio packet in the in-memory render source queue.
//! No actual PCM buffer — this is a contract shell for testing the runtime adapter/pump/sink chain.

use crate::playback::output_wasapi::output_thread::sink_boundary::render_packet::{
    PacketFormat, PacketTiming,
};

/// Metadata for an in-memory audio packet.
///
/// Represents the metadata of an audio packet that would be provided by a render source.
/// Contains no actual audio data — only the information needed to simulate packet delivery.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SourcePacketMetadata {
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
}

impl SourcePacketMetadata {
    /// Creates a new packet metadata with the given id and frame count.
    pub fn new(packet_id: u64, frame_count: u64) -> Self {
        Self {
            packet_id,
            frame_count,
            ..Default::default()
        }
    }

    /// Creates an end-of-stream packet metadata.
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
}
