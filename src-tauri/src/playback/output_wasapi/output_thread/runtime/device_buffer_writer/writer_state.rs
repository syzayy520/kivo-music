//! Device buffer writer state type.
//!
//! Pure data snapshot of a device buffer writer's state.
//! No behavior, no IO, no actual buffer.

use super::wasapi_writer::BufferLifecycle;

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
}
