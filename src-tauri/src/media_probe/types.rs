use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AudioProbeResult {
    pub duration_ms: Option<u64>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub sample_rate_hz: Option<u32>,
    pub bit_depth: Option<u16>,
    pub channels: Option<u16>,
    pub bitrate_kbps: Option<u32>,
    pub is_lossless: Option<bool>,
    pub has_embedded_cover: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VideoProbeResult {
    pub duration_ms: Option<u64>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frame_rate: Option<String>,
    pub hdr_format: Option<String>,
    pub dolby_vision_profile: Option<String>,
    pub audio_layout: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MediaProbeResult {
    pub path: String,
    pub audio: Option<AudioProbeResult>,
    pub video: Option<VideoProbeResult>,
}
