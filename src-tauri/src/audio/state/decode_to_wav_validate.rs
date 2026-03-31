use std::path::{Path, PathBuf};

use crate::audio::errors::{AudioError, AudioResult};

pub(super) fn resolve_output_path(input: &Path, output_arg: Option<&str>) -> AudioResult<PathBuf> {
    if let Some(p) = output_arg {
        let p = p.trim();
        if p.is_empty() {
            return Err(AudioError::InvalidInput("output_path is empty".to_string()));
        }
        return Ok(PathBuf::from(p));
    }
    let mut out = input.to_path_buf();
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("decoded");
    out.set_file_name(format!("{stem}.decoded.wav"));
    Ok(out)
}

pub(super) fn validate_input_path(input: &Path, input_raw: &str) -> AudioResult<()> {
    let md = std::fs::metadata(input).map_err(|e| {
        AudioError::InvalidInput(format!(
            "input_path not accessible: '{}' ({})",
            input_raw, e
        ))
    })?;
    if !md.is_file() {
        return Err(AudioError::InvalidInput(format!(
            "input_path is not a file: '{}'",
            input_raw
        )));
    }
    Ok(())
}

pub(super) fn validate_output_path(out: &Path, output_arg: Option<&str>) -> AudioResult<()> {
    if out.as_os_str().is_empty() {
        return Err(AudioError::InvalidInput("output_path is empty".to_string()));
    }
    if out.exists() {
        let md = std::fs::metadata(out).map_err(|e| {
            AudioError::InvalidInput(format!(
                "output_path not accessible: '{}' ({})",
                out.display(),
                e
            ))
        })?;
        if md.is_dir() {
            return Err(AudioError::InvalidInput(format!(
                "output_path is a directory: '{}'",
                out.display()
            )));
        }
    }

    // Parent dir must exist if explicitly specified.
    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(AudioError::InvalidInput(format!(
                "output_path parent directory does not exist: '{}' (from arg '{}')",
                parent.display(),
                output_arg.unwrap_or("(auto)")
            )));
        }
    }
    Ok(())
}

pub(super) fn validate_start_seconds(v: Option<f64>) -> AudioResult<f64> {
    match v {
        None => Ok(0.0),
        Some(x) => {
            if !x.is_finite() {
                return Err(AudioError::InvalidInput(format!(
                    "start_seconds must be a finite number, got {x}"
                )));
            }
            if x < 0.0 {
                return Err(AudioError::InvalidInput(format!(
                    "start_seconds must be >= 0, got {x}"
                )));
            }
            Ok(x)
        }
    }
}

pub(super) fn validate_max_seconds(v: Option<f64>) -> AudioResult<Option<f64>> {
    match v {
        None => Ok(None),
        Some(x) => {
            if !x.is_finite() {
                return Err(AudioError::InvalidInput(format!(
                    "max_seconds must be a finite number, got {x}"
                )));
            }
            if x < 0.0 {
                return Err(AudioError::InvalidInput(format!(
                    "max_seconds must be >= 0 (0 means unlimited), got {x}"
                )));
            }
            if x == 0.0 {
                Ok(None)
            } else {
                Ok(Some(x))
            }
        }
    }
}

pub(super) fn seconds_to_frames(limit_seconds: f64, sample_rate: u32) -> Option<u64> {
    if limit_seconds <= 0.0 {
        return None;
    }
    let frames_f = (limit_seconds * (sample_rate as f64)).floor();
    if !frames_f.is_finite() {
        return Some(u64::MAX);
    }
    if frames_f >= (u64::MAX as f64) {
        return Some(u64::MAX);
    }
    Some(frames_f as u64)
}
