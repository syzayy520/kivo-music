//! Sink error type.
//!
//! Represents errors that can occur during sink consumer processing.
//! Pure data — no behavior, no IO.

/// Error during sink consumer processing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SinkError {
    /// Buffer underrun occurred.
    BufferUnderrun {
        /// Number of frames missing.
        frames_missing: u64,
    },
    /// Device was lost or disconnected.
    DeviceLost,
    /// Request was invalid or malformed.
    InvalidRequest {
        /// Reason for rejection.
        reason: String,
    },
    /// Internal sink error.
    Internal {
        /// Error description.
        description: String,
    },
}

impl std::fmt::Display for SinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BufferUnderrun { frames_missing } => {
                write!(f, "buffer underrun: {} frames missing", frames_missing)
            }
            Self::DeviceLost => write!(f, "device lost"),
            Self::InvalidRequest { reason } => write!(f, "invalid request: {}", reason),
            Self::Internal { description } => write!(f, "internal error: {}", description),
        }
    }
}
