use std::process::Command;

use serde_json::Value;

use super::ffprobe_error::{command_error_message, spawn_error_message};
use super::super::service::{MediaProbeError, MediaProbeResultValue};

pub fn read_probe_json(path: &str) -> MediaProbeResultValue<Value> {
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
        .map_err(|error| MediaProbeError::BackendUnavailable(spawn_error_message(error)))?;

    if !output.status.success() {
        return Err(MediaProbeError::ProbeFailed(command_error_message(
            &output.stderr,
            "ffprobe command failed without stderr output",
        )));
    }

    serde_json::from_slice(&output.stdout)
        .map_err(|error| MediaProbeError::ProbeFailed(error.to_string()))
}
