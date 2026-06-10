//! WASAPI device buffer writer internal state.
//!
//! Runtime placeholder state tracking for WASAPI writer.
//! No real WASAPI runtime state, no COM lifecycle, no device state.

use super::super::{WriteResult, WriterCursor};
use super::runtime_mode::{Readiness, RuntimeMode};

/// Buffer lifecycle state.
///
/// Tracks the current phase of the buffer lifecycle.
/// No real WASAPI resources — pure state machine.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum BufferLifecycle {
    /// Buffer is empty (no frames buffered).
    #[default]
    Empty,
    /// Buffer has some frames but is not full.
    Partial,
    /// Buffer is at capacity.
    Full,
    /// Buffer writer has been closed.
    Closed,
}

impl BufferLifecycle {
    /// Returns true if the buffer can accept more frames.
    pub fn can_accept_frames(&self) -> bool {
        matches!(self, Self::Empty | Self::Partial)
    }

    /// Returns true if the buffer is closed.
    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }

    /// Returns true if the buffer is at capacity.
    pub fn is_full(&self) -> bool {
        matches!(self, Self::Full)
    }
}

/// Internal state of a WASAPI device buffer writer.
///
/// Tracks runtime placeholder state for simulated buffer behavior.
/// No real WASAPI resources or runtime state are held.
#[derive(Debug, Clone)]
pub struct WasapiDeviceBufferWriterState {
    /// Current buffer lifecycle phase.
    pub lifecycle: BufferLifecycle,
    /// Whether the writer has been closed.
    pub is_closed: bool,
    /// Simulated buffer fill in frames.
    pub buffer_fill_frames: u64,
    /// Total frames written since last reset.
    pub frames_written: u64,
    /// Total bytes written since last reset.
    pub bytes_written: u64,
    /// Total write attempts since last reset.
    pub write_attempts: u64,
    /// Total would-block events since last reset.
    pub would_block_count: u64,
    /// Total flush operations since last reset.
    pub flush_count: u64,
    /// Current write head position in circular buffer (0..capacity-1).
    pub write_head: u64,
    /// Number of times the circular buffer has wrapped.
    pub wrap_count: u64,
    /// Current consecutive would-block streak.
    pub consecutive_would_blocks: u64,
    /// Maximum consecutive would-block streak observed.
    pub max_consecutive_would_blocks: u64,
    /// Current consecutive successful write streak.
    pub write_streak: u64,
    /// Maximum consecutive successful write streak observed.
    pub max_write_streak: u64,
    /// Last cursor snapshot.
    pub last_cursor: WriterCursor,
    /// Last result produced.
    pub last_result: WriteResult,
    /// Current runtime mode.
    pub runtime_mode: RuntimeMode,
    /// Current readiness state.
    pub readiness: Readiness,
}

impl WasapiDeviceBufferWriterState {
    /// Creates a new initial state.
    pub fn new() -> Self {
        Self {
            lifecycle: BufferLifecycle::Empty,
            is_closed: false,
            buffer_fill_frames: 0,
            frames_written: 0,
            bytes_written: 0,
            write_attempts: 0,
            would_block_count: 0,
            flush_count: 0,
            write_head: 0,
            wrap_count: 0,
            consecutive_would_blocks: 0,
            max_consecutive_would_blocks: 0,
            write_streak: 0,
            max_write_streak: 0,
            last_cursor: WriterCursor::default(),
            last_result: WriteResult::Noop,
            runtime_mode: RuntimeMode::default(),
            readiness: Readiness::default(),
        }
    }

    /// Returns the current buffer lifecycle phase.
    pub fn lifecycle(&self) -> BufferLifecycle {
        self.lifecycle
    }

    /// Returns true if the writer is closed.
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    /// Updates the lifecycle state based on current buffer fill and capacity.
    pub fn update_lifecycle(&mut self, capacity_frames: u64) {
        if self.is_closed {
            self.lifecycle = BufferLifecycle::Closed;
        } else if self.buffer_fill_frames == 0 {
            self.lifecycle = BufferLifecycle::Empty;
        } else if self.buffer_fill_frames >= capacity_frames {
            self.lifecycle = BufferLifecycle::Full;
        } else {
            self.lifecycle = BufferLifecycle::Partial;
        }
    }

    /// Returns the current simulated buffer fill in frames.
    pub fn buffer_fill_frames(&self) -> u64 {
        self.buffer_fill_frames
    }

    /// Returns the total frames written since last reset.
    pub fn frames_written(&self) -> u64 {
        self.frames_written
    }

    /// Returns the total bytes written since last reset.
    pub fn bytes_written(&self) -> u64 {
        self.bytes_written
    }

    /// Returns the total write attempts since last reset.
    pub fn write_attempts(&self) -> u64 {
        self.write_attempts
    }

    /// Returns the total would-block events since last reset.
    pub fn would_block_count(&self) -> u64 {
        self.would_block_count
    }

    /// Returns the total flush operations since last reset.
    pub fn flush_count(&self) -> u64 {
        self.flush_count
    }

    /// Returns the current write head position in the circular buffer.
    pub fn write_head(&self) -> u64 {
        self.write_head
    }

    /// Returns the number of times the circular buffer has wrapped.
    pub fn wrap_count(&self) -> u64 {
        self.wrap_count
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

    /// Returns the last cursor snapshot.
    pub fn last_cursor(&self) -> &WriterCursor {
        &self.last_cursor
    }

    /// Returns the last result produced.
    pub fn last_result(&self) -> &WriteResult {
        &self.last_result
    }

    /// Checks runtime invariants and returns any violations.
    ///
    /// Returns Ok(()) if all invariants hold, or Err(description) if violated.
    /// Used for debug assertions and runtime health checks.
    pub fn check_invariants(&self, capacity_frames: u64) -> Result<(), String> {
        // buffer_fill_frames must not exceed capacity
        if self.buffer_fill_frames > capacity_frames {
            return Err(format!(
                "buffer_fill_frames ({}) exceeds capacity ({})",
                self.buffer_fill_frames, capacity_frames
            ));
        }

        // write_head must be within capacity
        if capacity_frames > 0 && self.write_head >= capacity_frames {
            return Err(format!(
                "write_head ({}) exceeds capacity ({})",
                self.write_head, capacity_frames
            ));
        }

        // lifecycle must match actual state
        let expected_lifecycle = if self.is_closed {
            BufferLifecycle::Closed
        } else if self.buffer_fill_frames == 0 {
            BufferLifecycle::Empty
        } else if self.buffer_fill_frames >= capacity_frames {
            BufferLifecycle::Full
        } else {
            BufferLifecycle::Partial
        };
        if self.lifecycle != expected_lifecycle {
            return Err(format!(
                "lifecycle mismatch: expected {:?}, got {:?}",
                expected_lifecycle, self.lifecycle
            ));
        }

        // consecutive_would_blocks must not exceed max
        if self.consecutive_would_blocks > self.max_consecutive_would_blocks {
            return Err(format!(
                "consecutive_would_blocks ({}) exceeds max ({})",
                self.consecutive_would_blocks, self.max_consecutive_would_blocks
            ));
        }

        Ok(())
    }

    /// Returns true if all runtime invariants hold.
    pub fn invariants_hold(&self, capacity_frames: u64) -> bool {
        self.check_invariants(capacity_frames).is_ok()
    }

    /// Returns the current runtime mode.
    pub fn runtime_mode(&self) -> RuntimeMode {
        self.runtime_mode
    }

    /// Returns the current readiness state.
    pub fn readiness(&self) -> Readiness {
        self.readiness
    }

    /// Returns true if the writer is in placeholder mode.
    pub fn is_placeholder(&self) -> bool {
        self.runtime_mode.is_placeholder()
    }

    /// Returns true if the writer is in real runtime mode.
    pub fn is_real_runtime(&self) -> bool {
        self.runtime_mode.is_real()
    }

    /// Returns true if the writer is ready to accept requests.
    pub fn is_runtime_ready(&self) -> bool {
        self.readiness.is_ready()
    }

    /// Updates the runtime mode.
    pub fn set_runtime_mode(&mut self, mode: RuntimeMode) {
        self.runtime_mode = mode;
    }

    /// Updates the readiness state.
    pub fn set_readiness(&mut self, readiness: Readiness) {
        self.readiness = readiness;
    }
}

impl Default for WasapiDeviceBufferWriterState {
    fn default() -> Self {
        Self::new()
    }
}
