use std::path::PathBuf;

/// Audio-wide result type.
pub type AudioResult<T> = Result<T, AudioError>;

/// Unified audio error.
///
/// Design goals:
/// - actionable messages (especially for missing FFmpeg DLLs)
/// - stable display (`to_string()`) for passing across Tauri boundary
#[derive(Debug, thiserror::Error)]
pub enum AudioError {
    // ---- legacy/compat variants (used by older modules importing `audio::error`) ----
    #[error("unsupported platform: {0}")]
    UnsupportedPlatform(&'static str),

    #[error("invalid argument: {0}")]
    InvalidArg(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("io error while {context}: {source}")]
    Io {
        context: &'static str,
        #[source]
        source: std::io::Error,
    },

    #[error(
        "ffmpeg library not found: {prefix}. search={search} attempted_dirs={attempted_dirs:?} tried={tried:?} last_error={last_error:?}. hint: set env KIVO_FFMPEG_DIR or pass ffmpeg_dir arg"
    )]
    FfmpegLibraryNotFound {
        prefix: &'static str,
        search: String,
        attempted_dirs: Vec<String>,
        tried: Vec<String>,
        last_error: Option<String>,
    },

    #[error("ffmpeg symbol missing: symbol={symbol} library={library} search={search} os_error={os_error:?}")]
    FfmpegSymbolMissing {
        symbol: String,
        library: String,
        search: String,
        os_error: Option<String>,
    },

    #[error("ffmpeg api error while {context}: {message} (code={code})")]
    FfmpegApiError {
        context: &'static str,
        code: i32,
        message: String,
    },

    #[error("thread panicked: {0}")]
    ThreadPanic(&'static str),

    #[error("ring buffer closed")]
    RingClosed,

    #[error("unsupported format: {0}")]
    Unsupported(String),

    #[error("ffmpeg dll dir invalid: {0}")]
    FfmpegDirInvalid(PathBuf),

    #[cfg(windows)]
    #[error("wasapi error: {0}")]
    Wasapi(#[from] wasapi::WasapiError),
}

impl AudioError {
    /// Stable, machine-readable error code.
    ///
    /// This code is intended for external surfaces (Tauri boundary / logs /
    /// acceptance evidence). It should remain stable over time.
    pub fn code(&self) -> &'static str {
        match self {
            AudioError::UnsupportedPlatform(_) => "AUDIO_UNSUPPORTED_PLATFORM",
            AudioError::InvalidArg(_) => "AUDIO_INVALID_ARG",
            AudioError::InvalidInput(_) => "AUDIO_INVALID_INPUT",
            AudioError::Io { .. } => "AUDIO_IO",
            AudioError::FfmpegLibraryNotFound { .. } => "AUDIO_FFMPEG_LIB_NOT_FOUND",
            AudioError::FfmpegSymbolMissing { .. } => "AUDIO_FFMPEG_SYMBOL_MISSING",
            AudioError::FfmpegApiError { .. } => "AUDIO_FFMPEG_API",
            AudioError::ThreadPanic(_) => "AUDIO_THREAD_PANIC",
            AudioError::RingClosed => "AUDIO_RING_CLOSED",
            AudioError::Unsupported(_) => "AUDIO_UNSUPPORTED",
            AudioError::FfmpegDirInvalid(_) => "AUDIO_FFMPEG_DIR_INVALID",
            #[cfg(windows)]
            AudioError::Wasapi(_) => "AUDIO_WASAPI",
        }
    }

    /// Stable, public-facing error string.
    ///
    /// Format is fixed: "[CODE] message".
    ///
    /// - The CODE is stable and machine-readable.
    /// - The message is actionable and may contain dynamic details.
    pub fn to_public_string(&self) -> String {
        format!("[{}] {}", self.code(), self.public_message())
    }

    fn public_message(&self) -> String {
        match self {
            AudioError::UnsupportedPlatform(p) => format!("unsupported platform: {p}"),
            AudioError::InvalidArg(msg) => format!("invalid argument: {msg}"),
            AudioError::InvalidInput(msg) => format!("invalid input: {msg}"),
            AudioError::Io { context, source } => {
                format!("io error while {context}: {source}")
            }
            AudioError::FfmpegLibraryNotFound {
                prefix,
                search,
                attempted_dirs,
                tried,
                last_error,
            } => {
                format!(
                    "ffmpeg library not found: {prefix}. search={search} attempted_dirs={attempted_dirs:?} tried={tried:?} last_error={last_error:?}. hint: set env KIVO_FFMPEG_DIR or pass ffmpeg_dir arg"
                )
            }
            AudioError::FfmpegSymbolMissing {
                symbol,
                library,
                search,
                os_error,
            } => {
                format!(
                    "ffmpeg symbol missing: symbol={symbol} library={library} search={search} os_error={os_error:?}"
                )
            }
            AudioError::FfmpegApiError {
                context,
                code,
                message,
            } => {
                format!("ffmpeg api error while {context}: {message} (code={code})")
            }
            AudioError::ThreadPanic(where_) => format!("thread panicked: {where_}"),
            AudioError::RingClosed => "ring buffer closed".to_string(),
            AudioError::Unsupported(what) => format!("unsupported: {what}"),
            AudioError::FfmpegDirInvalid(p) => {
                format!("ffmpeg dll dir invalid: {}", p.display())
            }
            #[cfg(windows)]
            AudioError::Wasapi(e) => format!("wasapi error: {e}"),
        }
    }

    pub fn io(context: &'static str, source: std::io::Error) -> Self {
        Self::Io { context, source }
    }

    pub fn invalid_arg(msg: impl Into<String>) -> Self {
        Self::InvalidArg(msg.into())
    }

    pub fn unsupported(msg: impl Into<String>) -> Self {
        Self::Unsupported(msg.into())
    }
}

impl From<std::io::Error> for AudioError {
    fn from(source: std::io::Error) -> Self {
        // Best-effort context for legacy `?` usage.
        Self::Io {
            context: "io",
            source,
        }
    }
}
