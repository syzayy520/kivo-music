use serde::{Deserialize, Serialize};

use super::windows_audio_render_session::WindowsAudioRenderSessionState;
use super::windows_audio_render_write_cycle::WindowsAudioRenderWriteCycle;
use super::windows_audio_render_write_cycle_summary::WindowsAudioRenderWriteCycleSummary;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowsAudioRenderWriteDiagnosticSnapshot {
    pub initialized: bool,
    pub render_client_available: bool,
    pub started: bool,
    pub buffer_frame_count: Option<u32>,
    pub queued_padding_frames: u32,
    pub available_frame_count: u32,
    pub written_frames: u64,
    pub cycle_summary: WindowsAudioRenderWriteCycleSummary,
    pub can_request_buffer: bool,
    pub backpressure_active: bool,
    pub note: Option<String>,
}

impl WindowsAudioRenderWriteDiagnosticSnapshot {
    pub fn from_session_and_cycle(
        session: &WindowsAudioRenderSessionState,
        cycle: &WindowsAudioRenderWriteCycle,
    ) -> Self {
        Self {
            initialized: session.initialized,
            render_client_available: session.render_client_available,
            started: session.started,
            buffer_frame_count: session.buffer_frame_count,
            queued_padding_frames: session.queued_padding_frames,
            available_frame_count: session.available_frame_count,
            written_frames: session.written_frames,
            cycle_summary: WindowsAudioRenderWriteCycleSummary::from_cycle(cycle),
            can_request_buffer: cycle.should_request_buffer,
            backpressure_active: cycle.should_defer,
            note: session.note.clone(),
        }
    }
}
