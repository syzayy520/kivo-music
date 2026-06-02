use serde::{Deserialize, Serialize};

use super::errors::PlaybackResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AudioSampleFormat {
    Float32,
    Signed16,
    Signed24,
    Signed32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioStreamInfo {
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub sample_format: AudioSampleFormat,
}

#[derive(Clone, Debug)]
pub struct DecodedAudioFrame {
    pub stream: AudioStreamInfo,
    pub position_ms: u64,
    pub samples: Vec<f32>,
}

pub trait AudioDecoder: Send {
    fn open(&mut self, path: &str) -> PlaybackResult<AudioStreamInfo>;
    fn next_frame(&mut self) -> PlaybackResult<Option<DecodedAudioFrame>>;
    fn seek(&mut self, position_ms: u64) -> PlaybackResult<()>;
    fn close(&mut self) -> PlaybackResult<()>;
}
