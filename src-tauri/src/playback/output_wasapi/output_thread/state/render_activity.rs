//! Render activity state types.
//!
//! Defines render activity tracking for the output thread.

use serde::{Deserialize, Serialize};

/// Render activity state of the output thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum RenderActivity {
    /// No render activity.
    #[default]
    Idle,
    /// Actively rendering frames to the audio device.
    Rendering,
    /// Rendering silence (underrun fill).
    Silenced,
    /// Render paused.
    Paused,
    /// Render encountered an error.
    Error,
}

/// Statistics for render activity.
///
/// Tracks frame processing metrics for monitoring and debugging.
/// All counters are monotonic (never decremented).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderActivityStats {
    /// Total frames submitted to the output thread.
    pub frames_submitted: u64,
    /// Total frames rendered to the audio device.
    pub frames_rendered: u64,
    /// Total bytes written to the audio device.
    pub bytes_rendered: u64,
}

impl RenderActivityStats {
    /// Create a new stats instance with all counters at zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a frame submission.
    pub fn record_submit(&mut self, count: u64) {
        self.frames_submitted = self.frames_submitted.saturating_add(count);
    }

    /// Record frames rendered to the audio device.
    pub fn record_render(&mut self, frames: u64, bytes: u64) {
        self.frames_rendered = self.frames_rendered.saturating_add(frames);
        self.bytes_rendered = self.bytes_rendered.saturating_add(bytes);
    }

    /// Get the number of frames pending (submitted - rendered).
    pub fn pending_frames(&self) -> u64 {
        self.frames_submitted.saturating_sub(self.frames_rendered)
    }

    /// Reset all counters to zero.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
