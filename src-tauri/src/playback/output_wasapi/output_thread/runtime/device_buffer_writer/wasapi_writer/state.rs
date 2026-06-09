//! WASAPI device buffer writer internal state.
//!
//! Runtime placeholder state tracking for WASAPI writer.
//! No real WASAPI runtime state, no COM lifecycle, no device state.

use super::super::{WriteResult, WriterCursor};

/// Internal state of a WASAPI device buffer writer.
///
/// Tracks runtime placeholder state for simulated buffer behavior.
/// No real WASAPI resources or runtime state are held.
#[derive(Debug, Clone)]
pub struct WasapiDeviceBufferWriterState {
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
    /// Last cursor snapshot.
    pub last_cursor: WriterCursor,
    /// Last result produced.
    pub last_result: WriteResult,
}

impl WasapiDeviceBufferWriterState {
    /// Creates a new initial state.
    pub fn new() -> Self {
        Self {
            is_closed: false,
            buffer_fill_frames: 0,
            frames_written: 0,
            bytes_written: 0,
            write_attempts: 0,
            would_block_count: 0,
            flush_count: 0,
            write_head: 0,
            wrap_count: 0,
            last_cursor: WriterCursor::default(),
            last_result: WriteResult::Noop,
        }
    }

    /// Returns true if the writer is closed.
    pub fn is_closed(&self) -> bool {
        self.is_closed
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

    /// Returns the last cursor snapshot.
    pub fn last_cursor(&self) -> &WriterCursor {
        &self.last_cursor
    }

    /// Returns the last result produced.
    pub fn last_result(&self) -> &WriteResult {
        &self.last_result
    }
}

impl Default for WasapiDeviceBufferWriterState {
    fn default() -> Self {
        Self::new()
    }
}
