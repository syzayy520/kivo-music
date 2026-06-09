//! Ring buffer read result type.
//!
//! Represents the outcome of reading from a ring buffer render source.
//! Pure data — no behavior, no IO, no actual buffer.

/// Outcome of reading from a ring buffer render source.
///
/// Captures the result of a read operation on a ring buffer source.
/// Pure metadata — no actual audio data or buffer manipulation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum RingBufferReadResult {
    /// Audio packet provided successfully from the buffer.
    Packet {
        /// Number of frames actually read.
        frames_read: u64,
        /// Number of bytes read.
        bytes_read: u64,
        /// Sequence number of the packet in the buffer.
        sequence_number: u64,
    },
    /// Buffer is exhausted (end of stream reached).
    Exhausted,
    /// Buffer is empty (underrun — no data available).
    Empty,
    /// Read was skipped (e.g., source not ready).
    #[default]
    Skipped,
}

impl RingBufferReadResult {
    /// Returns true if this is a successful packet result.
    pub fn is_packet(&self) -> bool {
        matches!(self, Self::Packet { .. })
    }

    /// Returns true if the source is exhausted.
    pub fn is_exhausted(&self) -> bool {
        matches!(self, Self::Exhausted)
    }

    /// Returns true if the buffer was empty (underrun).
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns true if the read was skipped.
    pub fn is_skipped(&self) -> bool {
        matches!(self, Self::Skipped)
    }

    /// Returns the number of frames read, or 0 if not a packet result.
    pub fn frames_read(&self) -> u64 {
        match self {
            Self::Packet { frames_read, .. } => *frames_read,
            _ => 0,
        }
    }

    /// Returns the number of bytes read, or 0 if not a packet result.
    pub fn bytes_read(&self) -> u64 {
        match self {
            Self::Packet { bytes_read, .. } => *bytes_read,
            _ => 0,
        }
    }

    /// Returns the sequence number, or 0 if not a packet result.
    pub fn sequence_number(&self) -> u64 {
        match self {
            Self::Packet {
                sequence_number, ..
            } => *sequence_number,
            _ => 0,
        }
    }
}
