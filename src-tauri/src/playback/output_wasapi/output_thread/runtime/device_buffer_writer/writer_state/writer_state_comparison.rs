//! WriterState comparison and logging methods.

use super::WriterState;

impl WriterState {
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
