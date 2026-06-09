//! Device buffer writer request type.
//!
//! Represents a request to write audio data to a device buffer.
//! Pure data — no behavior, no IO, no actual buffer.
//! Device-agnostic boundary for future real WASAPI device buffer writer.

/// Request to write audio data to a device buffer.
///
/// Captures the intent of a write operation on a device buffer writer.
/// Pure metadata — no actual audio data or buffer manipulation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum WriteRequest {
    /// Write an audio packet to the device buffer.
    WritePacket {
        /// Number of frames to write.
        frame_count: u64,
        /// Sample rate in Hz.
        sample_rate: u32,
        /// Number of audio channels.
        channel_count: u16,
    },
    /// Flush the device buffer (discard pending data).
    Flush,
    /// Close the device buffer writer.
    Close,
    /// No operation — skip this request.
    #[default]
    Noop,
}

impl WriteRequest {
    /// Creates a new write packet request.
    pub fn write_packet(frame_count: u64, sample_rate: u32, channel_count: u16) -> Self {
        Self::WritePacket {
            frame_count,
            sample_rate,
            channel_count,
        }
    }

    /// Creates a flush request.
    pub fn flush() -> Self {
        Self::Flush
    }

    /// Creates a close request.
    pub fn close() -> Self {
        Self::Close
    }

    /// Creates a no-op request.
    pub fn noop() -> Self {
        Self::Noop
    }

    /// Returns true if this is a write packet request.
    pub fn is_write_packet(&self) -> bool {
        matches!(self, Self::WritePacket { .. })
    }

    /// Returns true if this is a flush request.
    pub fn is_flush(&self) -> bool {
        matches!(self, Self::Flush)
    }

    /// Returns true if this is a close request.
    pub fn is_close(&self) -> bool {
        matches!(self, Self::Close)
    }

    /// Returns true if this is a no-op request.
    pub fn is_noop(&self) -> bool {
        matches!(self, Self::Noop)
    }

    /// Returns the frame count, or 0 if not a write packet request.
    pub fn frame_count(&self) -> u64 {
        match self {
            Self::WritePacket { frame_count, .. } => *frame_count,
            _ => 0,
        }
    }

    /// Returns the sample rate, or 0 if not a write packet request.
    pub fn sample_rate(&self) -> u32 {
        match self {
            Self::WritePacket { sample_rate, .. } => *sample_rate,
            _ => 0,
        }
    }

    /// Returns the channel count, or 0 if not a write packet request.
    pub fn channel_count(&self) -> u16 {
        match self {
            Self::WritePacket { channel_count, .. } => *channel_count,
            _ => 0,
        }
    }
}
