//! Render source cursor type.
//!
//! Tracks the read position within a render source stream.
//! Pure data — no behavior, no IO.

/// Read position within a render source stream.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SourceCursor {
    /// Current frame position in the stream.
    pub position_frames: u64,
    /// Total frames available (0 = unknown or unbounded).
    pub total_frames: u64,
    /// Current sample rate in Hz.
    pub sample_rate: u32,
    /// Current channel count.
    pub channel_count: u16,
}

impl SourceCursor {
    /// Returns true if total_frames is known and position has reached the end.
    pub fn is_at_end(&self) -> bool {
        self.total_frames > 0 && self.position_frames >= self.total_frames
    }

    /// Returns the number of frames remaining, or None if total is unknown.
    pub fn frames_remaining(&self) -> Option<u64> {
        if self.total_frames == 0 {
            None
        } else {
            Some(self.total_frames.saturating_sub(self.position_frames))
        }
    }
}
