use std::{path::PathBuf, thread};

use serde::{Deserialize, Serialize};

use crate::audio::{
    diag::DiagWriter,
    error::{AudioError, AudioResult},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SmokeMode {
    Silence,
    Sine,
}

impl SmokeMode {
    pub fn parse(s: &str) -> AudioResult<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "silence" | "silent" => Ok(Self::Silence),
            "sine" | "tone" => Ok(Self::Sine),
            other => Err(AudioError::InvalidArg(format!(
                "mode must be 'silence' or 'sine', got: {other}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmokeTestConfig {
    pub mode: SmokeMode,
    pub duration_secs: u32,

    /// Only used for sine mode.
    pub frequency_hz: f32,

    /// Only used for sine mode.
    pub amplitude: f32,

    /// Ring buffer size in seconds.
    pub ring_buffer_secs: f32,
}

impl Default for SmokeTestConfig {
    fn default() -> Self {
        Self {
            mode: SmokeMode::Sine,
            duration_secs: 60,
            frequency_hz: 440.0,
            amplitude: 0.2,
            ring_buffer_secs: 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmokeTestSummary {
    pub mode: String,
    pub duration_secs: f64,
    pub log_path: Option<String>,

    // device
    pub device_name: String,
    pub device_id: String,

    // format
    pub sample_rate: u32,
    pub channels: u16,
    pub wasapi_buffer_frames: u32,
    pub bits_per_sample: u16,
    pub sample_type: String,

    // ring + totals
    pub ring_capacity_samples: u32,
    pub ring_watermark_max_samples: usize,

    pub total_frames_written: u64,
    pub total_samples_written: u64,
    pub total_samples_popped: u64,
    pub total_samples_padded: u64,

    pub ring_write_samples: u64,
    pub ring_write_events: u64,
    pub ring_write_blocked_events: u64,
    pub ring_discard_samples: u64,

    pub underrun_events: u64,
    pub underrun_samples: u64,
    pub overrun_events: u64,
    pub overrun_samples: u64,

    pub device_rebuilds: u64,
    pub last_error: Option<String>,
    pub last_underrun_ring_fill_samples: u64,
    pub last_overrun_ring_fill_samples: u64,
    pub last_producer_blocked_ring_fill_samples: u64,
}

/// Runs the smoke test in a background thread and returns the log path.
///
/// The thread writes the final summary to the log once finished.
pub fn spawn_smoke_test(config: SmokeTestConfig) -> AudioResult<PathBuf> {
    let log_dir = std::env::temp_dir().join("kivo-shell").join("audio");
    let mut diag = DiagWriter::create(&log_dir, "wasapi_shared_smoke")?;
    let log_path = diag.path().to_path_buf();

    diag.line(format!(
        "config: mode={:?} duration_secs={} freq_hz={} amp={} ring_buffer_secs={}",
        config.mode,
        config.duration_secs,
        config.frequency_hz,
        config.amplitude,
        config.ring_buffer_secs
    ))?;

    thread::spawn(move || {
        let mut diag = diag;
        match run_smoke_test_blocking(config, &mut diag) {
            Ok(summary) => {
                let _ = diag.line(format!(
                    "done: device='{}' sr={} ch={} buffer_frames={} underrun_events={} underrun_samples={} log={}",
                    summary.device_name,
                    summary.sample_rate,
                    summary.channels,
                    summary.wasapi_buffer_frames,
                    summary.underrun_events,
                    summary.underrun_samples,
                    summary.log_path.as_deref().unwrap_or("<none>")
                ));
            }
            Err(e) => {
                let _ = diag.line(format!("error: {e}"));
            }
        }
    });

    Ok(log_path)
}

/// Blocking smoke test (useful for `cargo run --example ...`).
pub fn run_smoke_test_blocking(
    config: SmokeTestConfig,
    diag: &mut DiagWriter,
) -> AudioResult<SmokeTestSummary> {
    if config.duration_secs == 0 {
        return Err(AudioError::InvalidArg("duration_secs must be > 0".into()));
    }

    #[cfg(windows)]
    {
        crate::audio::output_wasapi::wasapi_shared::run_wasapi_shared_smoke(&config, diag)
    }

    #[cfg(not(windows))]
    {
        let _ = diag;
        Err(AudioError::UnsupportedPlatform(
            "WASAPI is only available on Windows",
        ))
    }
}
