//! Render client failure types.
//!
//! Pure metadata — no actual Windows API error codes.

/// Failure reasons for render client operations.
///
/// Pure metadata — no actual Windows API error codes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderClientFailure {
    /// Device was lost (e.g., unplugged).
    DeviceLost,
    /// Buffer is too small for the requested operation.
    BufferTooSmall {
        /// Requested size in frames.
        requested_frames: u32,
        /// Available size in frames.
        available_frames: u32,
    },
    /// Padding information is unavailable.
    PaddingUnavailable,
    /// Render client is unavailable (e.g., not initialized).
    RenderClientUnavailable,
    /// Invalid request parameters.
    InvalidRequest {
        /// Reason for rejection.
        reason: String,
    },
    /// Buffer acquisition failed.
    BufferAcquisitionFailed {
        /// Error description.
        description: String,
    },
    /// Buffer release failed.
    BufferReleaseFailed {
        /// Error description.
        description: String,
    },
    /// Internal error.
    Internal {
        /// Error description.
        description: String,
    },
}

impl std::fmt::Display for RenderClientFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeviceLost => write!(f, "render client device lost"),
            Self::BufferTooSmall {
                requested_frames,
                available_frames,
            } => write!(
                f,
                "render client buffer too small: requested {}, available {}",
                requested_frames, available_frames
            ),
            Self::PaddingUnavailable => write!(f, "render client padding unavailable"),
            Self::RenderClientUnavailable => write!(f, "render client unavailable"),
            Self::InvalidRequest { reason } => {
                write!(f, "render client invalid request: {}", reason)
            }
            Self::BufferAcquisitionFailed { description } => {
                write!(
                    f,
                    "render client buffer acquisition failed: {}",
                    description
                )
            }
            Self::BufferReleaseFailed { description } => {
                write!(f, "render client buffer release failed: {}", description)
            }
            Self::Internal { description } => {
                write!(f, "render client internal error: {}", description)
            }
        }
    }
}

impl RenderClientFailure {
    /// Returns true if this is a fatal error (device lost).
    pub fn is_fatal(&self) -> bool {
        matches!(self, Self::DeviceLost)
    }

    /// Returns true if this is a recoverable error.
    pub fn is_recoverable(&self) -> bool {
        !self.is_fatal()
    }

    /// Returns true if this is a buffer size error.
    pub fn is_buffer_size_error(&self) -> bool {
        matches!(self, Self::BufferTooSmall { .. })
    }

    /// Returns true if this is a padding error.
    pub fn is_padding_error(&self) -> bool {
        matches!(self, Self::PaddingUnavailable)
    }

    /// Returns true if this is a render client availability error.
    pub fn is_availability_error(&self) -> bool {
        matches!(self, Self::RenderClientUnavailable)
    }
}
