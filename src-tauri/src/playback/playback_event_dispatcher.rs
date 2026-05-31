use super::events::{PlaybackEvent, DEFAULT_PROGRESS_EVENT_INTERVAL_MS};
use super::state::PlaybackState;

#[derive(Debug)]
pub struct PlaybackEventDispatcher {
    progress_interval_ms: u64,
    last_progress_emit_at_ms: Option<u64>,
    pending_events: Vec<PlaybackEvent>,
}

impl Default for PlaybackEventDispatcher {
    fn default() -> Self {
        Self::new(DEFAULT_PROGRESS_EVENT_INTERVAL_MS)
    }
}

impl PlaybackEventDispatcher {
    pub fn new(progress_interval_ms: u64) -> Self {
        Self {
            progress_interval_ms,
            last_progress_emit_at_ms: None,
            pending_events: Vec::new(),
        }
    }

    pub fn emit_state_changed(&mut self, state: PlaybackState) {
        self.pending_events.push(PlaybackEvent::StateChanged(state));
    }

    pub fn emit_track_changed(&mut self, state: PlaybackState) {
        self.pending_events.push(PlaybackEvent::TrackChanged(state));
    }

    pub fn emit_error(&mut self, error: super::errors::PlaybackError) {
        self.pending_events.push(PlaybackEvent::Error(error));
    }

    pub fn emit_progress_at(&mut self, position_ms: u64, duration_ms: Option<u64>, now_ms: u64) {
        if self.should_emit_progress(now_ms) {
            self.pending_events.push(PlaybackEvent::Progress {
                position_ms,
                duration_ms,
            });
            self.last_progress_emit_at_ms = Some(now_ms);
        }
    }

    pub fn drain(&mut self) -> Vec<PlaybackEvent> {
        std::mem::take(&mut self.pending_events)
    }

    fn should_emit_progress(&self, now_ms: u64) -> bool {
        match self.last_progress_emit_at_ms {
            None => true,
            Some(last_ms) => now_ms.saturating_sub(last_ms) >= self.progress_interval_ms,
        }
    }
}
