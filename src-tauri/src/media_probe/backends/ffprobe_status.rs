use std::process::Command;

use super::super::status::MediaProbeBackendStatus;
use super::ffprobe_error::{command_error_message, spawn_error_message};

pub fn ffprobe_status() -> MediaProbeBackendStatus {
    match Command::new("ffprobe").arg("-version").output() {
        Ok(output) if output.status.success() => MediaProbeBackendStatus {
            backend_name: "ffprobe".to_string(),
            available: true,
            note: first_stdout_line(&output.stdout),
        },
        Ok(output) => MediaProbeBackendStatus {
            backend_name: "ffprobe".to_string(),
            available: false,
            note: Some(command_error_message(
                &output.stderr,
                "ffprobe version check failed without stderr output",
            )),
        },
        Err(error) => MediaProbeBackendStatus {
            backend_name: "ffprobe".to_string(),
            available: false,
            note: Some(spawn_error_message(error)),
        },
    }
}

fn first_stdout_line(stdout: &[u8]) -> Option<String> {
    String::from_utf8_lossy(stdout)
        .lines()
        .next()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
}
