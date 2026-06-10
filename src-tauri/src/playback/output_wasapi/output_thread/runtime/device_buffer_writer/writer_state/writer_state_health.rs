//! WriterState health and pressure assessment methods.

use super::buffer_pressure::BufferPressure;
use super::WriterState;

impl WriterState {
    /// Returns true if the writer is in a healthy state.
    ///
    /// A writer is healthy when:
    /// - No errors have occurred
    /// - Not currently stalled (no consecutive WouldBlock)
    /// - Not closed
    pub fn is_healthy(&self) -> bool {
        !self.has_errors() && !self.is_stalled() && !self.is_closed
    }

    /// Returns true if the buffer fill percentage meets or exceeds the given threshold.
    ///
    /// `threshold_pct` is clamped to 0-100. Returns false if capacity is 0.
    pub fn is_under_pressure(&self, threshold_pct: u8) -> bool {
        let threshold = threshold_pct.min(100);
        if self.buffer_capacity_frames == 0 {
            false
        } else {
            self.buffer_fill_percentage() >= threshold
        }
    }

    /// Returns the current buffer pressure level based on fill percentage.
    ///
    /// - Relaxed: below 50%
    /// - Moderate: 50%-74%
    /// - High: 75%-89%
    /// - Critical: 90% and above
    pub fn pressure_level(&self) -> BufferPressure {
        let pct = self.buffer_fill_percentage();
        if pct >= 90 {
            BufferPressure::Critical
        } else if pct >= 75 {
            BufferPressure::High
        } else if pct >= 50 {
            BufferPressure::Moderate
        } else {
            BufferPressure::Relaxed
        }
    }

    /// Returns a list of health warning strings.
    ///
    /// Each warning describes a condition that may indicate a problem.
    /// Returns an empty list if the writer is healthy.
    pub fn health_warnings(&self) -> Vec<&'static str> {
        let mut warnings = Vec::new();
        if self.is_closed {
            warnings.push("writer is closed");
        }
        if self.has_errors() {
            warnings.push("errors have occurred");
        }
        if self.is_stalled() {
            warnings.push("currently stalled (WouldBlock)");
        }
        if self.is_buffer_full() {
            warnings.push("buffer is full");
        }
        warnings
    }
}
