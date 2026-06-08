//! Packet timing type.
//!
//! Timing information for a render packet.
//! Pure data — no behavior, no IO.

/// Timing information for a render packet.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct PacketTiming {
    /// Presentation timestamp in microseconds.
    pub timestamp_us: u64,
    /// Sequence number (monotonically increasing).
    pub sequence: u64,
    /// Duration of the packet in microseconds.
    pub duration_us: u64,
}
