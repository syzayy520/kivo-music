//! Adapter error type.
//!
//! Represents errors that occur during render source adapter operations.
//! Pure data — no behavior, no IO, no WASAPI.

/// Error during a render source adapter operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdapterError {
    /// Source is exhausted and cannot provide more data.
    SourceExhausted,
    /// Audio format mismatch between source and sink.
    FormatMismatch {
        /// Expected format description.
        expected: String,
        /// Actual format description.
        actual: String,
    },
    /// Source was closed or disconnected.
    SourceClosed,
    /// Internal adapter error.
    Internal {
        /// Error description.
        description: String,
    },
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceExhausted => write!(f, "source exhausted"),
            Self::FormatMismatch { expected, actual } => {
                write!(f, "format mismatch: expected {}, got {}", expected, actual)
            }
            Self::SourceClosed => write!(f, "source closed"),
            Self::Internal { description } => {
                write!(f, "adapter internal error: {}", description)
            }
        }
    }
}
