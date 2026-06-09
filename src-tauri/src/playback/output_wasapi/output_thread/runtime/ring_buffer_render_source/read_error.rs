//! Ring buffer read error type.
//!
//! Represents errors that can occur when reading from a ring buffer render source.
//! Pure data — no behavior, no IO, no actual buffer.

/// Error when reading from a ring buffer render source.
///
/// Represents the taxonomy of errors specific to ring buffer read operations.
/// Pure metadata — no actual audio data or buffer manipulation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RingBufferReadError {
    /// Source is fully exhausted and cannot provide more data.
    SourceExhausted,
    /// Buffer is empty — underrun condition.
    BufferUnderrun,
    /// Buffer is full — overrun condition (write failed).
    BufferOverrun,
    /// Audio format mismatch between request and source.
    FormatMismatch {
        /// Expected format description.
        expected: String,
        /// Actual format description.
        actual: String,
    },
    /// Source has been closed or disconnected.
    SourceClosed,
    /// Buffer capacity exceeded.
    CapacityExceeded {
        /// Requested capacity.
        requested: u64,
        /// Maximum allowed capacity.
        maximum: u64,
    },
    /// Invalid buffer state detected.
    InvalidState {
        /// Description of the invalid state.
        description: String,
    },
    /// Internal ring buffer error.
    Internal {
        /// Error description.
        description: String,
    },
}

impl std::fmt::Display for RingBufferReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceExhausted => write!(f, "ring buffer source exhausted"),
            Self::BufferUnderrun => write!(f, "ring buffer underrun: buffer empty"),
            Self::BufferOverrun => write!(f, "ring buffer overrun: buffer full"),
            Self::FormatMismatch { expected, actual } => {
                write!(
                    f,
                    "ring buffer format mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            Self::SourceClosed => write!(f, "ring buffer source closed"),
            Self::CapacityExceeded { requested, maximum } => {
                write!(
                    f,
                    "ring buffer capacity exceeded: requested {}, maximum {}",
                    requested, maximum
                )
            }
            Self::InvalidState { description } => {
                write!(f, "ring buffer invalid state: {}", description)
            }
            Self::Internal { description } => {
                write!(f, "ring buffer internal error: {}", description)
            }
        }
    }
}

impl RingBufferReadError {
    /// Returns true if this is a fatal error (source exhausted or closed).
    pub fn is_fatal(&self) -> bool {
        matches!(self, Self::SourceExhausted | Self::SourceClosed)
    }

    /// Returns true if this is a recoverable error.
    pub fn is_recoverable(&self) -> bool {
        !self.is_fatal()
    }

    /// Returns true if this is an underrun error.
    pub fn is_underrun(&self) -> bool {
        matches!(self, Self::BufferUnderrun)
    }

    /// Returns true if this is an overrun error.
    pub fn is_overrun(&self) -> bool {
        matches!(self, Self::BufferOverrun)
    }

    /// Returns true if this is a format mismatch error.
    pub fn is_format_mismatch(&self) -> bool {
        matches!(self, Self::FormatMismatch { .. })
    }
}
