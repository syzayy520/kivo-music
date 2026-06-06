use crate::playback::decoder::{AudioSampleFormat, AudioStreamInfo};

#[derive(Clone, Debug)]
pub struct PcmSourceFormat {
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub sample_format: AudioSampleFormat,
}

impl From<&AudioStreamInfo> for PcmSourceFormat {
    fn from(stream: &AudioStreamInfo) -> Self {
        Self {
            sample_rate_hz: stream.sample_rate_hz,
            channels: stream.channels,
            sample_format: stream.sample_format.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PcmSourceChunk<'a> {
    pub stream: AudioStreamInfo,
    pub position_ms: u64,
    pub samples: &'a [f32],
    pub source_closed: bool,
}
