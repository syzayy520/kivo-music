/// Snapshot of the buffer state as seen by a future consumer loop.
///
/// Pure data — no references to the actual buffer type,
/// no Arc/Mutex, no external state queries.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct OutputThreadBufferSnapshot {
    /// Number of frames available for reading.
    pub available_frames: u32,
    /// Whether the buffer has been closed by the producer.
    pub is_closed: bool,
}

impl OutputThreadBufferSnapshot {
    /// Create a new snapshot with explicit values.
    #[allow(dead_code)]
    pub(crate) fn new(available_frames: u32, is_closed: bool) -> Self {
        Self {
            available_frames,
            is_closed,
        }
    }

    /// Whether there are frames available to read.
    #[allow(dead_code)]
    pub(crate) fn has_frames(self) -> bool {
        self.available_frames > 0
    }

    /// Whether the buffer is empty (no frames available).
    #[allow(dead_code)]
    pub(crate) fn is_empty(self) -> bool {
        self.available_frames == 0
    }

    /// Whether the buffer has been closed.
    #[allow(dead_code)]
    pub(crate) fn is_closed(self) -> bool {
        self.is_closed
    }

    /// Whether the consumer can still drain frames from this buffer.
    ///
    /// Returns `true` if there are frames available, regardless of closed state.
    /// A closed buffer with remaining frames is still drainable.
    #[allow(dead_code)]
    pub(crate) fn can_drain(self) -> bool {
        self.available_frames > 0
    }
}
