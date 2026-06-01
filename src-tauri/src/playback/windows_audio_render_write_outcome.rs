use serde::{Deserialize, Serialize};

use super::windows_audio_render_write_intent::WindowsAudioRenderWriteIntent;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowsAudioRenderWriteOutcome {
    pub requested_frame_count: u32,
    pub submitted_frame_count: u32,
    pub deferred_frame_count: u32,
    pub total_written_frames: u64,
    pub can_continue: bool,
}

impl WindowsAudioRenderWriteOutcome {
    pub fn from_intent(intent: &WindowsAudioRenderWriteIntent, previous_written_frames: u64) -> Self {
        let submitted_frame_count = intent.plan.writable_frame_count;
        let deferred_frame_count = intent.plan.deferred_frame_count;
        let total_written_frames =
            previous_written_frames.saturating_add(u64::from(submitted_frame_count));
        let can_continue = submitted_frame_count > 0;

        Self {
            requested_frame_count: intent.requested_frame_count,
            submitted_frame_count,
            deferred_frame_count,
            total_written_frames,
            can_continue,
        }
    }
}
