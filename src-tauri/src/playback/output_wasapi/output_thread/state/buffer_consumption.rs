//! Buffer consumption state types.
//!
//! Defines buffer consumption tracking for the output thread.

use serde::{Deserialize, Serialize};

/// Buffer consumption state of the output thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BufferConsumptionState {
    /// Buffer is empty (no frames available).
    #[default]
    Empty,
    /// Buffer has frames available for consumption.
    Available,
    /// Buffer is full (cannot accept more frames).
    Full,
    /// Buffer is in underrun state (silence fill active).
    Underrun,
    /// Buffer is in overrun state (frames rejected).
    Overrun,
}

/// Statistics for buffer consumption.
///
/// Tracks buffer-related metrics for monitoring and debugging.
/// All counters are monotonic (never decremented).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BufferConsumptionStats {
    /// Total silence frames written (underrun fill).
    pub silence_frames_written: u64,
    /// Number of underrun events (no frames available when needed).
    pub underrun_count: u64,
    /// Number of overrun events (frame rejected due to full buffer).
    pub overrun_count: u64,
}

impl BufferConsumptionStats {
    /// Create a new stats instance with all counters at zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record silence frames written (underrun fill).
    pub fn record_silence(&mut self, count: u64) {
        self.silence_frames_written = self.silence_frames_written.saturating_add(count);
    }

    /// Record an underrun event.
    pub fn record_underrun(&mut self) {
        self.underrun_count = self.underrun_count.saturating_add(1);
    }

    /// Record an overrun event.
    pub fn record_overrun(&mut self) {
        self.overrun_count = self.overrun_count.saturating_add(1);
    }

    /// Get the number of frames lost (silence frames written due to underrun).
    pub fn lost_frames(&self) -> u64 {
        self.silence_frames_written
    }

    /// Reset all counters to zero.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
