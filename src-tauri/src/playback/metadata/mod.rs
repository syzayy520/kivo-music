use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioMetadata {
    pub codec: Option<String>,
    pub container: Option<String>,
    pub sample_rate_hz: Option<u32>,
    pub bit_depth: Option<u16>,
    pub channels: Option<u16>,
    pub bitrate_kbps: Option<u32>,
    pub is_lossless: Option<bool>,
}
