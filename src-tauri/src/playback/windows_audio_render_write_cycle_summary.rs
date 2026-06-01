use serde::{Deserialize, Serialize};

use super::windows_audio_render_write_cycle::WindowsAudioRenderWriteCycle;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowsAudioRenderWriteCycleSummary {
    pub requested_frame_count: u32,
    pub source_frame_count: u32,
    pub submitted_frame_count: u32,
    pub deferred_frame_count: u32,
    pub total_written_frames: u64,
    pub should_request_buffer: bool,
    pub should_defer: bool,
    pub can_continue: bool,
}

impl WindowsAudioRenderWriteCycleSummary {
    pub fn from_cycle(cycle: &WindowsAudioRenderWriteCycle) -> Self {
        Self {
            requested_frame_count: cycle.intent.requested_frame_count,
            source_frame_count: cycle.intent.source_frame_count,
            submitted_frame_count: cycle.outcome.submitted_frame_count,
            deferred_frame_count: cycle.outcome.deferred_frame_count,
            total_written_frames: cycle.outcome.total_written_frames,
            should_request_buffer: cycle.should_request_buffer,
            should_defer: cycle.should_defer,
            can_continue: cycle.outcome.can_continue,
        }
    }
}
