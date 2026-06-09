//! Device buffer writer result type.
//!
//! Represents the outcome of writing to a device buffer.
//! Pure data — no behavior, no IO, no actual buffer.
//! Device-agnostic boundary for future real WASAPI device buffer writer.

/// Outcome of writing to a device buffer.
///
/// Captures the result of a write operation on a device buffer writer.
/// Pure metadata — no actual audio data or buffer manipulation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum WriteResult {
    /// Audio data written successfully to the device buffer.
    Written {
        /// Number of frames actually written.
        frames_written: u64,
        /// Number of bytes written.
        bytes_written: u64,
    },
    /// Write would block (device buffer is full).
    WouldBlock,
    /// Write was skipped (e.g., writer not ready or closed).
    Skipped,
    /// No operation performed.
    #[default]
    Noop,
}

impl WriteResult {
    /// Creates a successful write result.
    pub fn written(frames_written: u64, bytes_written: u64) -> Self {
        Self::Written {
            frames_written,
            bytes_written,
        }
    }

    /// Creates a would-block result.
    pub fn would_block() -> Self {
        Self::WouldBlock
    }

    /// Returns true if this is a successful write result.
    pub fn is_written(&self) -> bool {
        matches!(self, Self::Written { .. })
    }

    /// Returns true if this is a would-block result.
    pub fn is_would_block(&self) -> bool {
        matches!(self, Self::WouldBlock)
    }

    /// Returns true if this is a skipped result.
    pub fn is_skipped(&self) -> bool {
        matches!(self, Self::Skipped)
    }

    /// Returns true if this is a no-op result.
    pub fn is_noop(&self) -> bool {
        matches!(self, Self::Noop)
    }

    /// Returns the number of frames written, or 0 if not a written result.
    pub fn frames_written(&self) -> u64 {
        match self {
            Self::Written { frames_written, .. } => *frames_written,
            _ => 0,
        }
    }

    /// Returns the number of bytes written, or 0 if not a written result.
    pub fn bytes_written(&self) -> u64 {
        match self {
            Self::Written { bytes_written, .. } => *bytes_written,
            _ => 0,
        }
    }
}
