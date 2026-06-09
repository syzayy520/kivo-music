//! Device buffer writer frame byte sizing.
//!
//! Defines the f32 PCM byte contract used by device buffer writers.

pub(crate) const F32_SAMPLE_BYTES: u64 = 4;

pub(crate) fn f32_frame_byte_count(channel_count: u16) -> u64 {
    channel_count as u64 * F32_SAMPLE_BYTES
}

pub(crate) fn f32_packet_byte_count(frame_count: u64, channel_count: u16) -> Option<u64> {
    frame_count.checked_mul(f32_frame_byte_count(channel_count))
}
