use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowsAudioRenderWritePlan {
    pub requested_frame_count: u32,
    pub available_frame_count: u32,
    pub writable_frame_count: u32,
    pub deferred_frame_count: u32,
    pub can_write: bool,
}

impl WindowsAudioRenderWritePlan {
    pub fn plan(requested_frame_count: u32, available_frame_count: u32) -> Self {
        let writable_frame_count = requested_frame_count.min(available_frame_count);
        let deferred_frame_count = requested_frame_count.saturating_sub(writable_frame_count);

        Self {
            requested_frame_count,
            available_frame_count,
            writable_frame_count,
            deferred_frame_count,
            can_write: writable_frame_count > 0,
        }
    }
}
