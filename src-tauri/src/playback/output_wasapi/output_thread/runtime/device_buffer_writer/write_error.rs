//! Device buffer writer error type.
//!
//! Represents errors that can occur when writing to a device buffer.
//! Pure data — no behavior, no IO, no actual buffer.
//! Device-agnostic boundary for future real WASAPI device buffer writer.

/// Error when writing to a device buffer.
///
/// Represents the taxonomy of errors specific to device buffer write operations.
/// Pure metadata — no actual audio data or buffer manipulation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WriteError {
    /// Write would block — device buffer is full.
    WouldBlock,
    /// Device buffer writer has been closed.
    DeviceClosed,
    /// Audio format mismatch between request and device.
    FormatMismatch {
        /// Expected format description.
        expected: String,
        /// Actual format description.
        actual: String,
    },
    /// Buffer underflow — not enough data to write.
    Underflow {
        /// Number of frames available.
        available: u64,
        /// Number of frames requested.
        requested: u64,
    },
    /// Write operation failed.
    WriteFailed {
        /// Error description.
        description: String,
    },
    /// Internal device buffer writer error.
    Internal {
        /// Error description.
        description: String,
    },
    /// Invalid request parameters.
    InvalidRequest {
        /// Reason for rejection.
        reason: String,
    },
}

impl std::fmt::Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WouldBlock => write!(f, "device buffer write would block"),
            Self::DeviceClosed => write!(f, "device buffer writer closed"),
            Self::FormatMismatch { expected, actual } => {
                write!(
                    f,
                    "device buffer format mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            Self::Underflow {
                available,
                requested,
            } => {
                write!(
                    f,
                    "device buffer underflow: available {}, requested {}",
                    available, requested
                )
            }
            Self::WriteFailed { description } => {
                write!(f, "device buffer write failed: {}", description)
            }
            Self::Internal { description } => {
                write!(f, "device buffer internal error: {}", description)
            }
            Self::InvalidRequest { reason } => {
                write!(f, "device buffer invalid request: {}", reason)
            }
        }
    }
}

impl WriteError {
    /// Returns true if this is a fatal error (device closed).
    pub fn is_fatal(&self) -> bool {
        matches!(self, Self::DeviceClosed)
    }

    /// Returns true if this is a recoverable error.
    pub fn is_recoverable(&self) -> bool {
        !self.is_fatal()
    }

    /// Returns true if this is a would-block error.
    pub fn is_would_block(&self) -> bool {
        matches!(self, Self::WouldBlock)
    }

    /// Returns true if this is a format mismatch error.
    pub fn is_format_mismatch(&self) -> bool {
        matches!(self, Self::FormatMismatch { .. })
    }

    /// Returns true if this is an underflow error.
    pub fn is_underflow(&self) -> bool {
        matches!(self, Self::Underflow { .. })
    }

    /// Returns true if this is a write failed error.
    pub fn is_write_failed(&self) -> bool {
        matches!(self, Self::WriteFailed { .. })
    }

    /// Returns true if this is an invalid request error.
    pub fn is_invalid_request(&self) -> bool {
        matches!(self, Self::InvalidRequest { .. })
    }
}
