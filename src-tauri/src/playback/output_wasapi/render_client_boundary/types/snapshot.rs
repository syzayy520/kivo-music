//! Buffer snapshot types for render client boundary.
//!
//! Pure metadata — no actual buffer data.

/// Snapshot of the current padding in the buffer.
///
/// Pure metadata — no actual buffer data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PaddingSnapshot {
    /// Number of frames in the buffer that have been played.
    pub padding_frames: u32,
    /// Total buffer capacity in frames.
    pub buffer_capacity_frames: u32,
    /// Number of frames available for writing.
    pub available_frames: u32,
}

impl PaddingSnapshot {
    /// Creates a new padding snapshot.
    pub fn new(padding_frames: u32, buffer_capacity_frames: u32, available_frames: u32) -> Self {
        Self {
            padding_frames,
            buffer_capacity_frames,
            available_frames,
        }
    }

    /// Returns the buffer fill percentage (0-100).
    pub fn fill_percentage(&self) -> u8 {
        if self.buffer_capacity_frames == 0 {
            0
        } else {
            ((self.padding_frames * 100) / self.buffer_capacity_frames).min(100) as u8
        }
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.padding_frames == 0
    }

    /// Returns true if the buffer is full.
    pub fn is_full(&self) -> bool {
        self.buffer_capacity_frames > 0 && self.padding_frames >= self.buffer_capacity_frames
    }
}

/// Snapshot of the available frames in the buffer.
///
/// Pure metadata — no actual buffer data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AvailableFramesSnapshot {
    /// Number of frames available for writing.
    pub available_frames: u32,
    /// Total buffer capacity in frames.
    pub buffer_capacity_frames: u32,
    /// Number of frames currently in the buffer (padding).
    pub padding_frames: u32,
}

impl AvailableFramesSnapshot {
    /// Creates a new available frames snapshot.
    pub fn new(available_frames: u32, buffer_capacity_frames: u32, padding_frames: u32) -> Self {
        Self {
            available_frames,
            buffer_capacity_frames,
            padding_frames,
        }
    }

    /// Returns the available frames as a percentage of capacity (0-100).
    pub fn available_percentage(&self) -> u8 {
        if self.buffer_capacity_frames == 0 {
            0
        } else {
            ((self.available_frames * 100) / self.buffer_capacity_frames).min(100) as u8
        }
    }

    /// Returns true if there are frames available for writing.
    pub fn has_available_frames(&self) -> bool {
        self.available_frames > 0
    }

    /// Returns true if the buffer is full (no available frames).
    pub fn is_full(&self) -> bool {
        self.available_frames == 0 && self.buffer_capacity_frames > 0
    }
}
