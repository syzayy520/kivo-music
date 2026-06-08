//! Audio packet type.
//!
//! Represents a render packet for the sink consumer.
//! Pure data — no actual PCM buffer, no device resources.

use super::{PacketFormat, PacketTiming};

/// Audio render packet for the sink consumer.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AudioPacket {
    /// Number of frames in this packet.
    pub frame_count: u64,
    /// Audio format descriptor.
    pub format: PacketFormat,
    /// Timing information.
    pub timing: PacketTiming,
}
