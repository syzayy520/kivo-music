//! Device buffer writer state type.
//!
//! Pure data snapshot of a device buffer writer's state.
//! No behavior, no IO, no actual buffer.

use super::wasapi_writer::runtime_mode::{Readiness, RuntimeMode};
use super::wasapi_writer::BufferLifecycle;

/// Buffer pressure level based on fill percentage.
///
/// Categorizes the buffer fill state into severity tiers.
/// Pure data — no IO, no real buffer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum BufferPressure {
    /// Buffer fill is below 50%.
    #[default]
    Relaxed,
    /// Buffer fill is between 50% and 74%.
    Moderate,
    /// Buffer fill is between 75% and 89%.
    High,
    /// Buffer fill is 90% or above.
    Critical,
}

impl BufferPressure {
    /// Returns true if the pressure level indicates the buffer is under significant pressure.
    pub fn is_concerning(&self) -> bool {
        matches!(self, Self::High | Self::Critical)
    }

    /// Returns true if the pressure level is critical.
    pub fn is_critical(&self) -> bool {
        matches!(self, Self::Critical)
    }
}

/// Snapshot of a device buffer writer's current state.
///
/// Captures the state of a device buffer writer at a point in time.
/// Pure metadata — no actual audio data or buffer manipulation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WriterState {
    /// Current buffer lifecycle phase.
    pub lifecycle: BufferLifecycle,
    /// Total requests accepted by the writer.
    pub requests_accepted: u64,
    /// Total writes completed successfully.
    pub writes_completed: u64,
    /// Total frames written.
    pub frames_written: u64,
    /// Total bytes written.
    pub bytes_written: u64,
    /// Total errors encountered.
    pub errors: u64,
    /// Whether the writer is closed.
    pub is_closed: bool,
    /// Whether the writer is ready to accept requests.
    pub is_ready: bool,
    /// Current buffer fill level in frames.
    pub buffer_fill_frames: u64,
    /// Total buffer capacity in frames.
    pub buffer_capacity_frames: u64,
    /// Number of buffer wraps that have occurred.
    pub buffer_wrap_count: u64,
    /// Total would-block events (write when buffer full).
    pub would_block_count: u64,
    /// Total flush operations performed.
    pub flush_count: u64,
    /// Current consecutive would-block streak.
    pub consecutive_would_blocks: u64,
    /// Maximum consecutive would-block streak observed.
    pub max_consecutive_would_blocks: u64,
    /// Current consecutive successful write streak.
    pub write_streak: u64,
    /// Maximum consecutive successful write streak observed.
    pub max_write_streak: u64,
    /// Current runtime mode.
    pub runtime_mode: RuntimeMode,
    /// Current readiness state.
    pub readiness: Readiness,
}

impl WriterState {
    /// Returns true if the buffer is empty.
    pub fn is_buffer_empty(&self) -> bool {
        self.buffer_fill_frames == 0
    }

    /// Returns true if the buffer is full.
    pub fn is_buffer_full(&self) -> bool {
        self.buffer_capacity_frames > 0 && self.buffer_fill_frames >= self.buffer_capacity_frames
    }

    /// Returns the buffer fill percentage (0-100).
    pub fn buffer_fill_percentage(&self) -> u8 {
        if self.buffer_capacity_frames == 0 {
            0
        } else {
            ((self.buffer_fill_frames * 100) / self.buffer_capacity_frames).min(100) as u8
        }
    }

    /// Returns the average write size in frames.
    pub fn average_write_size(&self) -> u64 {
        if self.writes_completed == 0 {
            0
        } else {
            self.frames_written / self.writes_completed
        }
    }

    /// Returns true if any would-block events have occurred.
    pub fn has_would_blocks(&self) -> bool {
        self.would_block_count > 0
    }

    /// Returns true if any errors have occurred.
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }

    /// Returns true if the writer is currently in a would-block streak.
    pub fn is_stalled(&self) -> bool {
        self.consecutive_would_blocks > 0
    }

    /// Returns the current consecutive would-block count.
    pub fn consecutive_would_blocks(&self) -> u64 {
        self.consecutive_would_blocks
    }

    /// Returns the maximum consecutive would-block count observed.
    pub fn max_consecutive_would_blocks(&self) -> u64 {
        self.max_consecutive_would_blocks
    }

    /// Returns the current consecutive successful write streak.
    pub fn write_streak(&self) -> u64 {
        self.write_streak
    }

    /// Returns the maximum consecutive successful write streak observed.
    pub fn max_write_streak(&self) -> u64 {
        self.max_write_streak
    }

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

    /// Returns the names of fields that differ between two snapshots.
    ///
    /// Compares each field and returns a list of field names where values differ.
    /// Useful for logging state transitions and debugging.
    pub fn changed_field_names(&self, other: &WriterState) -> Vec<&'static str> {
        let mut changes = Vec::new();
        if self.lifecycle != other.lifecycle {
            changes.push("lifecycle");
        }
        if self.requests_accepted != other.requests_accepted {
            changes.push("requests_accepted");
        }
        if self.writes_completed != other.writes_completed {
            changes.push("writes_completed");
        }
        if self.frames_written != other.frames_written {
            changes.push("frames_written");
        }
        if self.bytes_written != other.bytes_written {
            changes.push("bytes_written");
        }
        if self.errors != other.errors {
            changes.push("errors");
        }
        if self.is_closed != other.is_closed {
            changes.push("is_closed");
        }
        if self.is_ready != other.is_ready {
            changes.push("is_ready");
        }
        if self.buffer_fill_frames != other.buffer_fill_frames {
            changes.push("buffer_fill_frames");
        }
        if self.buffer_capacity_frames != other.buffer_capacity_frames {
            changes.push("buffer_capacity_frames");
        }
        if self.buffer_wrap_count != other.buffer_wrap_count {
            changes.push("buffer_wrap_count");
        }
        if self.would_block_count != other.would_block_count {
            changes.push("would_block_count");
        }
        if self.flush_count != other.flush_count {
            changes.push("flush_count");
        }
        if self.consecutive_would_blocks != other.consecutive_would_blocks {
            changes.push("consecutive_would_blocks");
        }
        if self.max_consecutive_would_blocks != other.max_consecutive_would_blocks {
            changes.push("max_consecutive_would_blocks");
        }
        if self.write_streak != other.write_streak {
            changes.push("write_streak");
        }
        if self.max_write_streak != other.max_write_streak {
            changes.push("max_write_streak");
        }
        if self.runtime_mode != other.runtime_mode {
            changes.push("runtime_mode");
        }
        if self.readiness != other.readiness {
            changes.push("readiness");
        }
        changes
    }

    /// Returns true if all fields match between two snapshots.
    ///
    /// Equivalent to `self == other` but explicit for readability.
    pub fn is_same_state(&self, other: &WriterState) -> bool {
        self.changed_field_names(other).is_empty()
    }

    /// Returns a one-line summary string for logging.
    ///
    /// Format: `[lifecycle] fill=X% streak=Y/Z wb=A/B healthy=H`
    pub fn summary_line(&self) -> String {
        format!(
            "[{:?}] fill={}% streak={}/{} wb={}/{} flush={} healthy={} pressure={:?} mode={:?} ready={:?}",
            self.lifecycle,
            self.buffer_fill_percentage(),
            self.write_streak,
            self.max_write_streak,
            self.consecutive_would_blocks,
            self.max_consecutive_would_blocks,
            self.flush_count,
            self.is_healthy(),
            self.pressure_level(),
            self.runtime_mode,
            self.readiness,
        )
    }
}
