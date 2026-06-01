use serde::{Deserialize, Serialize};

use super::windows_audio_render_session::WindowsAudioRenderSessionState;
use super::windows_audio_render_write_plan::WindowsAudioRenderWritePlan;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowsAudioRenderWriteIntent {
    pub requested_frame_count: u32,
    pub source_frame_count: u32,
    pub plan: WindowsAudioRenderWritePlan,
    pub can_submit: bool,
}

impl WindowsAudioRenderWriteIntent {
    pub fn from_session(
        session: &WindowsAudioRenderSessionState,
        requested_frame_count: u32,
        source_frame_count: u32,
    ) -> Self {
        let effective_request = requested_frame_count.min(source_frame_count);
        let plan =
            WindowsAudioRenderWritePlan::plan(effective_request, session.available_frame_count);
        let can_submit = plan.can_write;

        Self {
            requested_frame_count,
            source_frame_count,
            plan,
            can_submit,
        }
    }
}
