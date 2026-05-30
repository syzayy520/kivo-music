use std::process::Command;

use serde_json::Value;

use super::ffprobe_json::parse_probe_result;
use super::super::service::{MediaProbeError, MediaProbeResultValue, ProbeBackend};
use super::super::types::MediaProbeResult;

#[derive(Clone, Debug, Default)]
pub struct FfprobeBackend;

impl ProbeBackend for FfprobeBackend {
    fn name(&self) -> &'static str {
        "ffprobe"
    }

    fn probe(&self, path: &str) -> MediaProbeResultValue<MediaProbeResult> {
        let output = Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
            ])
            .arg(path)
            .output()
            .map_err(|error| MediaProbeError::BackendUnavailable(error.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(MediaProbeError::ProbeFailed(stderr));
        }

        let root: Value = serde_json::from_slice(&output.stdout)
            .map_err(|error| MediaProbeError::ProbeFailed(error.to_string()))?;

        Ok(parse_probe_result(path, &root))
    }
}
