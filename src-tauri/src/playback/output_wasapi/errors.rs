/// Error message constants for WASAPI stub operations.
///
/// These messages are used with `PlaybackError::UnsupportedOperation`
/// to indicate that the WASAPI output is not yet implemented.
pub const WASAPI_NOT_IMPLEMENTED: &str = "WASAPI output is not implemented yet";

/// Helper to create an unsupported operation error message for WASAPI stub.
pub fn wasapi_unsupported(operation: &str) -> String {
    format!("{WASAPI_NOT_IMPLEMENTED}: {operation}")
}

/// Typed errors for WASAPI device open operations.
///
/// These errors describe specific failure points in the WASAPI
/// initialization chain, allowing callers to distinguish between
/// different failure modes.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WasapiOpenError {
    /// Platform does not support WASAPI (non-Windows).
    UnsupportedPlatform,
    /// COM initialization failed.
    ComInitFailed(String),
    /// Audio device not found or unavailable.
    DeviceNotFound(String),
    /// IAudioClient activation failed.
    ClientActivateFailed(String),
    /// Mix format retrieval or negotiation failed.
    FormatFailed(String),
    /// IAudioClient::Initialize failed.
    InitializeFailed(String),
    /// IAudioRenderClient acquisition failed.
    RenderClientFailed(String),
}

impl std::fmt::Display for WasapiOpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedPlatform => write!(f, "WASAPI not supported on this platform"),
            Self::ComInitFailed(msg) => write!(f, "COM init failed: {msg}"),
            Self::DeviceNotFound(msg) => write!(f, "device not found: {msg}"),
            Self::ClientActivateFailed(msg) => write!(f, "client activate failed: {msg}"),
            Self::FormatFailed(msg) => write!(f, "format failed: {msg}"),
            Self::InitializeFailed(msg) => write!(f, "initialize failed: {msg}"),
            Self::RenderClientFailed(msg) => write!(f, "render client failed: {msg}"),
        }
    }
}

impl WasapiOpenError {
    /// Convert to a user-facing error message string.
    pub(crate) fn to_message(&self) -> String {
        self.to_string()
    }
}
