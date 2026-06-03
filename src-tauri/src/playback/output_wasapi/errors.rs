/// Error message constants for WASAPI stub operations.
///
/// These messages are used with `PlaybackError::UnsupportedOperation`
/// to indicate that the WASAPI output is not yet implemented.
pub const WASAPI_NOT_IMPLEMENTED: &str = "WASAPI output is not implemented yet";

/// Helper to create an unsupported operation error message for WASAPI stub.
pub fn wasapi_unsupported(operation: &str) -> String {
    format!("{WASAPI_NOT_IMPLEMENTED}: {operation}")
}
