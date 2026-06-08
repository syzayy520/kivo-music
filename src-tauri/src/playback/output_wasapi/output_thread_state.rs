//! Output thread state types for the WASAPI output thread.
//!
//! Pure data types with no behavior. Defines the lifecycle states
//! of the output thread and statistics tracking.

use serde::{Deserialize, Serialize};

/// Lifecycle states of the WASAPI output thread.
///
/// Transitions:
/// - NotStarted → Starting (thread spawn requested)
/// - Starting → Running (thread successfully started)
/// - Running → Draining (flush/stop requested, draining remaining frames)
/// - Running → Stopping (immediate stop requested)
/// - Draining → Stopping (drain complete or timeout)
/// - Stopping → Stopped (thread gracefully stopped)
/// - Starting → Failed (thread spawn failed)
/// - Running → Failed (runtime error)
/// - Draining → Failed (error during drain)
/// - Stopping → Failed (error during stop)
/// - Failed → NotStarted (reset after failure)
/// - Stopped → NotStarted (reset after stop)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OutputThreadState {
    /// Thread has not been spawned.
    #[default]
    NotStarted,
    /// Thread spawn requested, waiting for confirmation.
    Starting,
    /// Thread is running and processing frames.
    Running,
    /// Thread is draining remaining frames before stopping.
    Draining,
    /// Thread is stopping (immediate or after drain).
    Stopping,
    /// Thread has stopped gracefully.
    Stopped,
    /// Thread encountered an error.
    Failed,
}

impl OutputThreadState {
    /// Returns true if the thread is in an active state (Starting, Running, Draining).
    pub fn is_active(self) -> bool {
        matches!(self, Self::Starting | Self::Running | Self::Draining)
    }

    /// Returns true if the thread is in a terminal state (Stopped, Failed).
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Stopped | Self::Failed)
    }

    /// Returns true if the thread is in a transitional state (Starting, Stopping).
    pub fn is_transitional(self) -> bool {
        matches!(self, Self::Starting | Self::Stopping)
    }

    /// Returns true if the thread can accept new frames.
    pub fn can_accept_frames(self) -> bool {
        matches!(self, Self::Running)
    }

    /// Returns true if the thread is draining (accepting no new frames).
    pub fn is_draining(self) -> bool {
        self == Self::Draining
    }
}

/// Statistics for the output thread.
///
/// Tracks frame processing metrics for monitoring and debugging.
/// All counters are monotonic (never decremented).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputThreadStats {
    /// Total frames submitted to the output thread.
    pub frames_submitted: u64,
    /// Total frames rendered to the audio device.
    pub frames_rendered: u64,
    /// Total silence frames written (underrun fill).
    pub silence_frames_written: u64,
    /// Number of underrun events (no frames available when needed).
    pub underrun_count: u64,
    /// Number of overrun events (frame rejected due to full buffer).
    pub overrun_count: u64,
    /// Number of errors encountered.
    pub error_count: u64,
    /// Total bytes written to the audio device.
    pub bytes_rendered: u64,
}

impl OutputThreadStats {
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

    /// Record an error.
    pub fn record_error(&mut self) {
        self.error_count = self.error_count.saturating_add(1);
    }

    /// Get the number of frames pending (submitted - rendered).
    pub fn pending_frames(&self) -> u64 {
        self.frames_submitted.saturating_sub(self.frames_rendered)
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
