//! Render packet types.
//!
//! Device-agnostic types representing audio packets for the sink consumer.
//! No actual PCM buffer, no device resources, no WASAPI.

pub mod audio_packet;
pub mod packet_format;
pub mod packet_timing;

pub use audio_packet::AudioPacket;
pub use packet_format::{PacketFormat, SampleFormat};
pub use packet_timing::PacketTiming;
