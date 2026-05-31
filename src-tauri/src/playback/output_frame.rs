use serde::{Deserialize, Serialize};

use super::decoder::{AudioStreamInfo, DecodedAudioFrame};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OutputAudioFrame {
    pub stream: AudioStreamInfo,
    pub position_ms: u64,
    pub samples: Vec<f32>,
}

impl OutputAudioFrame {
    pub fn from_decoded_frame(decoded: DecodedAudioFrame) -> Self {
        Self {
            stream: decoded.stream,
            position_ms: decoded.position_ms,
            samples: decoded.samples,
        }
    }
}
