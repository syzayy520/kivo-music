use super::super::output_thread_core::buffer_snapshot::OutputThreadBufferSnapshot;

/// Pure in-memory mock buffer for testing the consumer harness.
///
/// No real buffer reads, no thread, no WASAPI.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct OutputThreadMockBuffer {
    available_frames: u32,
    closed: bool,
}

#[allow(dead_code)]
impl OutputThreadMockBuffer {
    /// Create a new mock buffer with explicit values.
    pub(crate) fn new(available_frames: u32, closed: bool) -> Self {
        Self {
            available_frames,
            closed,
        }
    }

    /// Empty open buffer (producer still active, no frames yet).
    pub(crate) fn empty_open() -> Self {
        Self {
            available_frames: 0,
            closed: false,
        }
    }

    /// Open buffer with frames available.
    pub(crate) fn with_frames(frames: u32) -> Self {
        Self {
            available_frames: frames,
            closed: false,
        }
    }

    /// Closed and empty (producer finished, all frames consumed).
    pub(crate) fn closed_empty() -> Self {
        Self {
            available_frames: 0,
            closed: true,
        }
    }

    /// Closed but still has frames remaining.
    pub(crate) fn closed_with_frames(frames: u32) -> Self {
        Self {
            available_frames: frames,
            closed: true,
        }
    }

    /// Take a snapshot of the current buffer state.
    pub(crate) fn snapshot(self) -> OutputThreadBufferSnapshot {
        OutputThreadBufferSnapshot::new(self.available_frames, self.closed)
    }

    /// Consume frames from the buffer (immutable, returns new state).
    ///
    /// Clamps to available frames. Preserves closed state.
    pub(crate) fn consume(self, frames: u32) -> Self {
        let consumed = frames.min(self.available_frames);
        Self {
            available_frames: self.available_frames - consumed,
            closed: self.closed,
        }
    }
}
