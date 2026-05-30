use super::service::{MediaProbeError, MediaProbeResultValue};

pub fn validate_probe_path(path: &str) -> MediaProbeResultValue<&str> {
    let path = path.trim();

    if path.is_empty() {
        Err(MediaProbeError::InvalidPath(
            "media probe path is empty".to_string(),
        ))
    } else {
        Ok(path)
    }
}
