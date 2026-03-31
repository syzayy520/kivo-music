use crate::audio::{errors::AudioResult, AudioError};

pub(super) fn validate_input_path(p: &str) -> AudioResult<std::path::PathBuf> {
    if p.trim().is_empty() {
        return Err(AudioError::InvalidInput("input_path is empty".into()));
    }
    let pb = std::path::PathBuf::from(p);
    if !pb.exists() {
        return Err(AudioError::InvalidInput(format!(
            "input_path not found: {}",
            pb.display()
        )));
    }
    Ok(pb)
}

pub(super) fn validate_start_seconds(v: Option<f64>) -> AudioResult<f64> {
    let s = v.unwrap_or(0.0);
    if !s.is_finite() || s < 0.0 {
        return Err(AudioError::InvalidInput(format!(
            "start_seconds invalid: {s}"
        )));
    }
    Ok(s)
}

pub(super) fn validate_max_seconds_or_default(v: Option<f64>) -> AudioResult<f64> {
    let m = v.unwrap_or(30.0);
    if !m.is_finite() || m <= 0.0 {
        return Err(AudioError::InvalidInput(format!(
            "max_seconds invalid: {m}"
        )));
    }
    Ok(m.min(6.0 * 60.0 * 60.0))
}

pub(super) fn create_diag_log_path() -> Option<std::path::PathBuf> {
    let base = std::env::var_os("KIVO_AUDIO_LOG_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("kivo_shell_logs"));
    let _ = std::fs::create_dir_all(&base);
    Some(base.join(format!(
        "play_file_wasapi_shared_{}_{}.log",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    )))
}
