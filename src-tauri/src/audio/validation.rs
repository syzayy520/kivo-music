//! Audio input validation helpers.
//!
//! Centralized validation logic to ensure consistent error handling and bounds checking
//! across all audio command entry points.

use crate::audio::errors::AudioError;

/// Validates an audio file path.
///
/// Returns error if:
/// - path is empty or whitespace-only
/// - path does not exist
pub fn validate_file_path(path: &str) -> Result<(), AudioError> {
    let path = path.trim();
    if path.is_empty() {
        return Err(AudioError::InvalidInput("file path is empty".into()));
    }

    let pb = std::path::PathBuf::from(path);
    if !pb.exists() {
        return Err(AudioError::InvalidInput(format!(
            "file not found: {}",
            pb.display()
        )));
    }

    Ok(())
}

/// Validates a time offset in seconds.
///
/// Returns error if value is not finite or negative.
pub fn validate_seconds(
    value: Option<f64>,
    label: &str,
    allow_zero: bool,
) -> Result<(), AudioError> {
    if let Some(s) = value {
        if !s.is_finite() {
            return Err(AudioError::InvalidInput(format!(
                "{label} must be finite: {s}"
            )));
        }
        if s < 0.0 {
            return Err(AudioError::InvalidInput(format!(
                "{label} must be >= 0: {s}"
            )));
        }
        if !allow_zero && s == 0.0 {
            return Err(AudioError::InvalidInput(format!(
                "{label} must be > 0: {s}"
            )));
        }
    }
    Ok(())
}

/// Validates a volume level (0.0 to 1.0).
pub fn validate_volume(volume: f32) -> Result<(), AudioError> {
    if !volume.is_finite() {
        return Err(AudioError::InvalidInput(format!(
            "volume must be finite: {volume}"
        )));
    }
    if !(0.0..=1.0).contains(&volume) {
        return Err(AudioError::InvalidInput(format!(
            "volume must be between 0.0 and 1.0: {volume}"
        )));
    }
    Ok(())
}

/// Validates a session ID.
///
/// Returns error if session_id is 0 (reserved/invalid).
pub fn validate_session_id(session_id: u64) -> Result<(), AudioError> {
    if session_id == 0 {
        return Err(AudioError::InvalidInput("session_id must be > 0".into()));
    }
    Ok(())
}

/// Validates queue size.
///
/// Returns error if queue exceeds reasonable limits to prevent resource exhaustion.
pub fn validate_queue_size(count: usize) -> Result<(), AudioError> {
    const MAX_QUEUE_SIZE: usize = 10000;
    if count > MAX_QUEUE_SIZE {
        return Err(AudioError::InvalidInput(format!(
            "queue too large: {} (max {})",
            count, MAX_QUEUE_SIZE
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_file_path() {
        // Empty path
        assert!(validate_file_path("").is_err());
        assert!(validate_file_path("   ").is_err());

        // Non-existent file
        assert!(validate_file_path("/nonexistent/path/to/file.wav").is_err());
    }

    #[test]
    fn test_validate_seconds() {
        // Valid
        assert!(validate_seconds(Some(0.0), "test", true).is_ok());
        assert!(validate_seconds(Some(1.5), "test", true).is_ok());
        assert!(validate_seconds(None, "test", true).is_ok());

        // Invalid
        assert!(validate_seconds(Some(f64::NAN), "test", true).is_err());
        assert!(validate_seconds(Some(f64::INFINITY), "test", true).is_err());
        assert!(validate_seconds(Some(-1.0), "test", true).is_err());
        assert!(validate_seconds(Some(0.0), "test", false).is_err());
    }

    #[test]
    fn test_validate_volume() {
        assert!(validate_volume(0.0).is_ok());
        assert!(validate_volume(1.0).is_ok());

        assert!(validate_volume(-0.1).is_err());
        assert!(validate_volume(1.1).is_err());
        assert!(validate_volume(f32::NAN).is_err());
    }

    #[test]
    fn test_validate_session_id() {
        assert!(validate_session_id(1).is_ok());
        assert!(validate_session_id(u64::MAX).is_ok());
        assert!(validate_session_id(0).is_err());
    }

    #[test]
    fn test_validate_queue_size() {
        assert!(validate_queue_size(0).is_ok());
        assert!(validate_queue_size(5000).is_ok());
        assert!(validate_queue_size(10000).is_ok());
        assert!(validate_queue_size(10001).is_err());
    }
}
