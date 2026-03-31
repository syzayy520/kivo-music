use serde::{Deserialize, Serialize};

use crate::audio::errors::AudioError;

/// Options for starting a background playback session.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayStartOptions {
    pub start_seconds: Option<f64>,
    pub max_seconds: Option<f64>,
    pub ffmpeg_dir: Option<String>,
    /// Optional WASAPI render device id (Windows only). If not found, fallback to default device.
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioSessionState {
    Starting,
    Playing,
    Paused,
    Stopping,
    Stopped,
    Error,
}

/// Snapshot returned to the frontend / CLI for diagnostics.
#[derive(Debug, Clone, Serialize)]
pub struct AudioSessionStatus {
    pub session_id: u64,
    pub state: AudioSessionState,
    /// Current media position in seconds.
    #[serde(rename = "pos")]
    pub pos_seconds: f64,
    /// Total duration in seconds (best-effort, may be unknown).
    #[serde(rename = "duration")]
    pub duration_seconds: Option<f64>,
    pub sample_rate: u32,
    pub channels: u16,
    pub device_name: Option<String>,
    pub input_path: String,
    pub last_error: Option<String>,

    /// True iff the session ended naturally due to EOF.
    /// Stop / seek-past-EOF / error MUST keep this as false.
    pub ended: bool,

    /// Linear volume gain in [0.0, 1.0].
    pub volume: f32,

    /// Number of times the audio output had to be rebuilt for this session.
    ///
    /// Field name is stable and must remain unchanged.
    pub output_rebuilds: u32,
    /// Last output rebuild reason. When set, it MUST include a stable prefix like:
    /// "[OUTPUT_REBUILD] ...".
    ///
    /// Field name is stable and must remain unchanged.
    pub last_output_rebuild_reason: Option<String>,

    // --- Minimal observability metrics (stable, long-term) ---
    /// Current ring buffer fill level, in *frames*.
    pub buffer_fill_frames: u64,
    /// Ring buffer capacity, in *frames*.
    pub buffer_capacity_frames: u64,
    /// Underrun events since session start / last seek.
    pub underrun_events: u64,
    /// Estimated underrun frames since session start / last seek.
    pub underrun_frames: u64,
}

impl AudioSessionStatus {
    pub fn new(session_id: u64, input_path: String) -> Self {
        Self {
            session_id,
            state: AudioSessionState::Starting,
            pos_seconds: 0.0,
            duration_seconds: None,
            sample_rate: 0,
            channels: 0,
            device_name: None,
            input_path,
            last_error: None,

            ended: false,
            volume: 1.0,

            output_rebuilds: 0,
            last_output_rebuild_reason: None,

            buffer_fill_frames: 0,
            buffer_capacity_frames: 0,
            underrun_events: 0,
            underrun_frames: 0,
        }
    }

    pub fn set_last_error_from_audio_error(&mut self, err: &AudioError) {
        self.last_error = Some(err.to_public_string());
    }

    pub fn set_last_error_message(&mut self, msg: impl Into<String>) {
        self.last_error = Some(msg.into());
    }

    pub fn set_last_output_rebuild_reason(&mut self, reason: impl Into<String>) {
        let reason = reason.into();
        let normalized = if reason.starts_with("[OUTPUT_REBUILD] ") {
            reason
        } else {
            format!("[OUTPUT_REBUILD] {reason}")
        };
        self.last_output_rebuild_reason = Some(normalized);
    }
}

/// Internal control commands for a session.
#[derive(Debug, Clone)]
pub(super) enum ServiceCmd {
    Stop,
    Pause(bool),
    Seek { seconds: f64 },
    SetVolume(f32),
}

/// Serializable queue snapshot for the frontend.
///
/// Field names are stable and must remain unchanged.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueSnapshot {
    pub items: Vec<String>,
    pub current_index: Option<u32>,
    pub current_path: Option<String>,
    pub autoplay: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_last_error_from_audio_error_uses_public_string() {
        let mut status = AudioSessionStatus::new(1, "a.mp3".into());
        status.set_last_error_from_audio_error(&AudioError::InvalidInput("bad path".into()));
        assert_eq!(
            status.last_error.as_deref(),
            Some("[AUDIO_INVALID_INPUT] invalid input: bad path")
        );
    }

    #[test]
    fn set_last_output_rebuild_reason_adds_prefix_when_missing() {
        let mut status = AudioSessionStatus::new(1, "a.mp3".into());
        status.set_last_output_rebuild_reason("start output failed");
        assert_eq!(
            status.last_output_rebuild_reason.as_deref(),
            Some("[OUTPUT_REBUILD] start output failed")
        );
    }

    #[test]
    fn set_last_output_rebuild_reason_preserves_existing_prefix() {
        let mut status = AudioSessionStatus::new(1, "a.mp3".into());
        status.set_last_output_rebuild_reason("[OUTPUT_REBUILD] already tagged");
        assert_eq!(
            status.last_output_rebuild_reason.as_deref(),
            Some("[OUTPUT_REBUILD] already tagged")
        );
    }

    #[test]
    fn set_last_error_message_stores_verbatim_message() {
        let mut status = AudioSessionStatus::new(1, "a.mp3".into());
        status.set_last_error_message("[SEEK_FAILED] seek_to=12.300: boom");
        assert_eq!(
            status.last_error.as_deref(),
            Some("[SEEK_FAILED] seek_to=12.300: boom")
        );
    }
}
