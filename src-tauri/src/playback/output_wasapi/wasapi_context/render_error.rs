//! Error and report types for render buffer write operations.

/// Error from a render buffer write operation.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WasapiRenderWriteError {
    /// Context is not open.
    NotOpen,
    /// IAudioRenderClient is missing.
    MissingRenderClient,
    /// IAudioClient is missing.
    MissingAudioClient,
    /// Format is not IEEE Float32 (required for write).
    UnsupportedFormat,
    /// Requested frame count is zero.
    InvalidFrameCount,
    /// Byte length doesn't match expected size for frame count.
    ByteLengthMismatch { expected: usize, actual: usize },
    /// GetBuffer call failed.
    GetBufferFailed(String),
    /// ReleaseBuffer call failed.
    ReleaseBufferFailed(String),
}

impl std::fmt::Display for WasapiRenderWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotOpen => write!(f, "context is not open"),
            Self::MissingRenderClient => write!(f, "render client is missing"),
            Self::MissingAudioClient => write!(f, "audio client is missing"),
            Self::UnsupportedFormat => write!(f, "format is not IEEE Float32"),
            Self::InvalidFrameCount => write!(f, "frame count must be > 0"),
            Self::ByteLengthMismatch { expected, actual } => {
                write!(f, "byte length mismatch: expected {expected}, got {actual}")
            }
            Self::GetBufferFailed(e) => write!(f, "GetBuffer failed: {e}"),
            Self::ReleaseBufferFailed(e) => write!(f, "ReleaseBuffer failed: {e}"),
        }
    }
}

/// Report from a successful render buffer write.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WasapiRenderWriteReport {
    /// Number of frames written.
    pub frames_written: u32,
    /// Number of bytes written (0 for silence writes).
    pub bytes_written: u32,
    /// Whether the AUDCLNT_BUFFERFLAGS_SILENT flag was used.
    pub used_silent_flag: bool,
    /// Sample rate at time of write.
    pub sample_rate_hz: u32,
    /// Channel count at time of write.
    pub channels: u16,
}
