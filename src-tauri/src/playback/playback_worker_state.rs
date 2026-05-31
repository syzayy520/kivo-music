use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PlaybackWorkerPhase {
    Idle,
    Loaded,
    Playing,
    Paused,
    Stopped,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaybackWorkerState {
    pub phase: PlaybackWorkerPhase,
    pub active_track_id: Option<String>,
    pub last_error: Option<String>,
}

impl PlaybackWorkerState {
    pub fn idle() -> Self {
        Self {
            phase: PlaybackWorkerPhase::Idle,
            active_track_id: None,
            last_error: None,
        }
    }

    pub fn mark_loaded(&mut self, track_id: impl Into<String>) {
        self.phase = PlaybackWorkerPhase::Loaded;
        self.active_track_id = Some(track_id.into());
        self.last_error = None;
    }

    pub fn mark_playing(&mut self) {
        self.phase = PlaybackWorkerPhase::Playing;
    }

    pub fn mark_paused(&mut self) {
        self.phase = PlaybackWorkerPhase::Paused;
    }

    pub fn mark_stopped(&mut self) {
        self.phase = PlaybackWorkerPhase::Stopped;
    }

    pub fn mark_failed(&mut self, message: impl Into<String>) {
        self.phase = PlaybackWorkerPhase::Failed;
        self.last_error = Some(message.into());
    }
}
