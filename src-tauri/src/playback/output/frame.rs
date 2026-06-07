use serde::{Deserialize, Serialize};

use crate::playback::decoder::AudioStreamInfo;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioOutputFrame {
    pub stream: AudioStreamInfo,
    pub position_ms: u64,
    pub samples: Vec<f32>,
}
