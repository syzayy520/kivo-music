//! Dispatch error type.
//!
//! Represents errors that occur during sink dispatch operations.
//! Pure data — no behavior, no IO, no WASAPI.

/// Error during a sink dispatch operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DispatchError {
    /// Buffer underrun occurred during dispatch.
    BufferUnderrun {
        /// Number of frames missing.
        frames_missing: u64,
    },
    /// Sink device was lost or disconnected.
    DeviceLost,
    /// Request was invalid or malformed.
    InvalidRequest {
        /// Reason for rejection.
        reason: String,
    },
    /// Internal dispatch error.
    Internal {
        /// Error description.
        description: String,
    },
}

impl std::fmt::Display for DispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BufferUnderrun { frames_missing } => {
                write!(
                    f,
                    "dispatch buffer underrun: {} frames missing",
                    frames_missing
                )
            }
            Self::DeviceLost => write!(f, "dispatch device lost"),
            Self::InvalidRequest { reason } => {
                write!(f, "dispatch invalid request: {}", reason)
            }
            Self::Internal { description } => {
                write!(f, "dispatch internal error: {}", description)
            }
        }
    }
}
