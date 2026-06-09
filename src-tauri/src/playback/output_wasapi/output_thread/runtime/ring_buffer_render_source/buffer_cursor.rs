//! Ring buffer source cursor type.
//!
//! Tracks the read position within a ring buffer render source stream.
//! Pure data — no behavior, no IO, no actual buffer.

/// Read position within a ring buffer render source stream.
///
/// Tracks the current read position and buffer state for a ring buffer source.
/// Pure metadata — no actual audio data or buffer manipulation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BufferCursor {
    /// Current read position in frames from buffer start.
    pub read_position: u64,
    /// Current write position in frames from buffer start.
    pub write_position: u64,
    /// Total buffer capacity in frames.
    pub buffer_capacity: u64,
    /// Total frames available for reading.
    pub available_frames: u64,
    /// Current sample rate in Hz.
    pub sample_rate: u32,
    /// Current channel count.
    pub channel_count: u16,
    /// Buffer wrap count (how many times the write pointer has wrapped).
    pub wrap_count: u64,
}

impl BufferCursor {
    /// Returns true if the buffer is empty (no frames available to read).
    pub fn is_empty(&self) -> bool {
        self.available_frames == 0
    }

    /// Returns true if the buffer is full (write position has caught up to read position).
    pub fn is_full(&self) -> bool {
        self.buffer_capacity > 0 && self.available_frames >= self.buffer_capacity
    }

    /// Returns the number of free frames in the buffer.
    pub fn free_frames(&self) -> u64 {
        self.buffer_capacity.saturating_sub(self.available_frames)
    }

    /// Returns the fill level as a percentage (0-100).
    pub fn fill_percentage(&self) -> u8 {
        if self.buffer_capacity == 0 {
            0
        } else {
            ((self.available_frames * 100) / self.buffer_capacity).min(100) as u8
        }
    }

    /// Returns true if the buffer has wrapped around at least once.
    pub fn has_wrapped(&self) -> bool {
        self.wrap_count > 0
    }

    /// Returns the number of frames that have been read in total.
    pub fn total_frames_read(&self) -> u64 {
        // This is a simplified calculation; actual implementation may differ
        self.read_position
    }
}
