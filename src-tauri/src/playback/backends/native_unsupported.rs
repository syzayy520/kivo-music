use super::super::errors::{PlaybackError, PlaybackResult};

const NATIVE_CORE_LABEL: &str = "kivo core audio";

pub fn unsupported_operation<T>(operation: &str) -> PlaybackResult<T> {
    Err(PlaybackError::UnsupportedOperation(format!(
        "{NATIVE_CORE_LABEL} {operation} is not implemented yet"
    )))
}

pub fn unsupported_operation_message(operation: &str) -> String {
    format!("{NATIVE_CORE_LABEL} {operation} is not implemented yet")
}
