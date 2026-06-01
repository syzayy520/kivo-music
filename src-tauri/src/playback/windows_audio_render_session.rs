use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowsAudioRenderSessionState {
    pub initialized: bool,
    pub render_client_available: bool,
    pub started: bool,
    pub buffer_frame_count: Option<u32>,
    pub queued_padding_frames: u32,
    pub available_frame_count: u32,
    pub written_frames: u64,
    pub note: Option<String>,
}

impl WindowsAudioRenderSessionState {
    pub fn unavailable(note: impl Into<String>) -> Self {
        Self {
            initialized: false,
            render_client_available: false,
            started: false,
            buffer_frame_count: None,
            queued_padding_frames: 0,
            available_frame_count: 0,
            written_frames: 0,
            note: Some(note.into()),
        }
    }

    pub fn ready(buffer_frame_count: u32) -> Self {
        Self {
            initialized: true,
            render_client_available: true,
            started: false,
            buffer_frame_count: Some(buffer_frame_count),
            queued_padding_frames: 0,
            available_frame_count: buffer_frame_count,
            written_frames: 0,
            note: None,
        }
    }

    pub fn with_padding(mut self, queued_padding_frames: u32) -> Self {
        self.queued_padding_frames = queued_padding_frames;
        self.available_frame_count = self
            .buffer_frame_count
            .map(|buffer_frame_count| buffer_frame_count.saturating_sub(queued_padding_frames))
            .unwrap_or_default();
        self
    }

    pub fn with_written_frames(mut self, written_frames: u64) -> Self {
        self.written_frames = written_frames;
        self
    }
}
