//! Render source error type.
//!
//! Represents errors that can occur during render source processing.
//! Pure data — no behavior, no IO.

/// Error during render source processing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderSourceError {
    /// Source is fully exhausted and cannot provide more data.
    SourceExhausted,
    /// Audio format mismatch between request and source.
    FormatMismatch {
        /// Expected format description.
        expected: String,
        /// Actual format description.
        actual: String,
    },
    /// Source has been closed or disconnected.
    SourceClosed,
    /// Internal render source error.
    Internal {
        /// Error description.
        description: String,
    },
}

impl std::fmt::Display for RenderSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceExhausted => write!(f, "source exhausted"),
            Self::FormatMismatch { expected, actual } => {
                write!(f, "format mismatch: expected {}, got {}", expected, actual)
            }
            Self::SourceClosed => write!(f, "source closed"),
            Self::Internal { description } => {
                write!(f, "internal source error: {}", description)
            }
        }
    }
}
