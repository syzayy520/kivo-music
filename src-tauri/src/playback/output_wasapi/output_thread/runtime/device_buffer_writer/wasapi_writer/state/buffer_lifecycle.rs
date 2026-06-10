//! Buffer lifecycle state machine.

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
