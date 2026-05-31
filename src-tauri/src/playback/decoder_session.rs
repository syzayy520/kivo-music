use serde::{Deserialize, Serialize};

use super::decoder::AudioStreamInfo;
use super::decoder_request::AudioDecoderOpenRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DecoderSession {
    pub track_id: String,
    pub source_path: String,
    pub stream_info: AudioStreamInfo,
    pub opened_at_ms: u64,
    pub last_position_ms: u64,
    pub decoded_frame_count: u64,
}

impl DecoderSession {
    pub fn from_open_request(
        request: &AudioDecoderOpenRequest,
        stream_info: AudioStreamInfo,
        opened_at_ms: u64,
    ) -> Self {
        Self {
            track_id: request.track_id.clone(),
            source_path: request.source_path.clone(),
            stream_info,
            opened_at_ms,
            last_position_ms: 0,
            decoded_frame_count: 0,
        }
    }

    pub fn update_position(&mut self, position_ms: u64) {
        self.last_position_ms = position_ms;
    }

    pub fn count_frame(&mut self) {
        self.decoded_frame_count = self.decoded_frame_count.saturating_add(1);
    }
}
