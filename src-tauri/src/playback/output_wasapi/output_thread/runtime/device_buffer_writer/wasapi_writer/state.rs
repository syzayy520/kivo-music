//! WASAPI device buffer writer internal state.
//!
//! Type scaffold state tracking for WASAPI writer.
//! No real WASAPI runtime state, no COM lifecycle, no device state.

use super::super::{WriteResult, WriterCursor};

/// Internal state of a WASAPI device buffer writer.
///
/// Tracks scaffold-level state only.
/// No real WASAPI resources or runtime state are held.
#[derive(Debug, Clone)]
pub struct WasapiDeviceBufferWriterState {
    /// Whether the writer has been closed.
    pub is_closed: bool,
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
            last_cursor: WriterCursor::default(),
            last_result: WriteResult::Noop,
        }
    }

    /// Returns true if the writer is closed.
    pub fn is_closed(&self) -> bool {
        self.is_closed
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
