use serde::{Deserialize, Serialize};

use super::decoder::AudioStreamInfo;
use super::errors::PlaybackResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OutputDevice {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutputSettings {
    pub selected_device_id: Option<String>,
    pub exclusive_mode: bool,
    pub bit_perfect_mode: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioOutputFrame {
    pub stream: AudioStreamInfo,
    pub position_ms: u64,
    pub samples: Vec<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutputLatency {
    pub requested_ms: Option<u32>,
    pub measured_ms: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OutputControlState {
    pub volume_level: f32,
    pub muted: bool,
}

impl Default for OutputControlState {
    fn default() -> Self {
        Self {
            volume_level: 1.0,
            muted: false,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutputRuntimeStatus {
    pub is_open: bool,
    pub is_active: bool,
    pub active_device_id: Option<String>,
    pub pending_frames: usize,
    pub latency: OutputLatency,
    pub controls: OutputControlState,
    pub gap_count: u64,
    pub last_error: Option<String>,
}

pub trait OutputSink {
    fn open(&mut self, settings: &OutputSettings) -> PlaybackResult<OutputRuntimeStatus>;
    fn submit_frame(&mut self, frame: AudioOutputFrame) -> PlaybackResult<OutputRuntimeStatus>;
    fn pause(&mut self) -> PlaybackResult<OutputRuntimeStatus>;
    fn resume(&mut self) -> PlaybackResult<OutputRuntimeStatus>;
    fn flush(&mut self) -> PlaybackResult<OutputRuntimeStatus>;
    fn stop(&mut self) -> PlaybackResult<OutputRuntimeStatus>;
    fn set_volume(&mut self, level: f32) -> PlaybackResult<OutputRuntimeStatus>;
    fn set_muted(&mut self, muted: bool) -> PlaybackResult<OutputRuntimeStatus>;
    fn status(&self) -> OutputRuntimeStatus;
    fn close(&mut self) -> PlaybackResult<()>;
}
