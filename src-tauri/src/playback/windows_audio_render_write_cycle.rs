use serde::{Deserialize, Serialize};

use super::windows_audio_render_session::WindowsAudioRenderSessionState;
use super::windows_audio_render_write_intent::WindowsAudioRenderWriteIntent;
use super::windows_audio_render_write_outcome::WindowsAudioRenderWriteOutcome;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowsAudioRenderWriteCycle {
    pub intent: WindowsAudioRenderWriteIntent,
    pub outcome: WindowsAudioRenderWriteOutcome,
    pub should_request_buffer: bool,
    pub should_defer: bool,
}

impl WindowsAudioRenderWriteCycle {
    pub fn from_session(
        session: &WindowsAudioRenderSessionState,
        requested_frame_count: u32,
        source_frame_count: u32,
    ) -> Self {
        let intent = WindowsAudioRenderWriteIntent::from_session(
            session,
            requested_frame_count,
            source_frame_count,
        );
        let outcome = WindowsAudioRenderWriteOutcome::from_intent(&intent, session.written_frames);
        let should_request_buffer = intent.can_submit;
        let should_defer = outcome.deferred_frame_count > 0;

        Self {
            intent,
            outcome,
            should_request_buffer,
            should_defer,
        }
    }
}
