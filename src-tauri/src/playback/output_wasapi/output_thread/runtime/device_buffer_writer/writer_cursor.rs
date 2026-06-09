//! Device buffer writer cursor type.
//!
//! Tracks the write position within a device buffer writer stream.
//! Pure data — no behavior, no IO, no actual buffer.

/// Write position within a device buffer writer stream.
///
/// Tracks the current write position and buffer state for a device buffer writer.
/// Pure metadata — no actual audio data or buffer manipulation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WriterCursor {
    /// Current write position in frames from buffer start.
    pub write_position: u64,
    /// Total buffer capacity in frames.
    pub buffer_capacity: u64,
    /// Number of frames currently in the buffer.
    pub buffered_frames: u64,
    /// Current sample rate in Hz.
    pub sample_rate: u32,
    /// Current channel count.
    pub channel_count: u16,
    /// Total frames written since last reset.
    pub total_frames_written: u64,
    /// Number of buffer wraps that have occurred.
    pub wrap_count: u64,
}

impl WriterCursor {
    /// Returns true if the buffer is empty (no frames buffered).
    pub fn is_empty(&self) -> bool {
        self.buffered_frames == 0
    }

    /// Returns true if the buffer is full.
    pub fn is_full(&self) -> bool {
        self.buffer_capacity > 0 && self.buffered_frames >= self.buffer_capacity
    }

    /// Returns the number of free frames in the buffer.
    pub fn free_frames(&self) -> u64 {
        self.buffer_capacity.saturating_sub(self.buffered_frames)
    }

    /// Returns the buffer fill level as a percentage (0-100).
    pub fn fill_percentage(&self) -> u8 {
        if self.buffer_capacity == 0 {
            0
        } else {
            ((self.buffered_frames * 100) / self.buffer_capacity).min(100) as u8
        }
    }

    /// Returns true if the buffer has wrapped around at least once.
    pub fn has_wrapped(&self) -> bool {
        self.wrap_count > 0
    }

    /// Returns the number of frames written in the current session.
    pub fn session_frames_written(&self) -> u64 {
        self.total_frames_written
    }
}
