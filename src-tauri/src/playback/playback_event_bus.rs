use super::activity_log::PlaybackActivityLogState;
use super::activity_snapshot::PlaybackActivityLogSnapshot;
use super::events::PlaybackEvent;
use super::playback_event_dispatcher::PlaybackEventDispatcher;
use super::state::PlaybackState;

#[derive(Debug, Default)]
pub struct PlaybackEventBus {
    dispatcher: PlaybackEventDispatcher,
    activity: PlaybackActivityLogState,
}

impl PlaybackEventBus {
    pub fn new(dispatcher: PlaybackEventDispatcher, activity: PlaybackActivityLogState) -> Self {
        Self {
            dispatcher,
            activity,
        }
    }

    pub fn emit_state_changed(&mut self, state: PlaybackState) {
        self.dispatcher.emit_state_changed(state);
    }

    pub fn emit_track_changed(&mut self, state: PlaybackState) {
        self.dispatcher.emit_track_changed(state);
    }

    pub fn emit_error(&mut self, error: super::errors::PlaybackError) {
        self.dispatcher.emit_error(error);
    }

    pub fn emit_progress_at(&mut self, position_ms: u64, duration_ms: Option<u64>, now_ms: u64) {
        self.dispatcher
            .emit_progress_at(position_ms, duration_ms, now_ms);
    }

    pub fn flush_to_activity(&mut self) -> Vec<PlaybackEvent> {
        let events = self.dispatcher.drain();

        for event in events.iter().cloned() {
            self.activity.append(event);
        }

        events
    }

    pub fn activity_snapshot(&self) -> PlaybackActivityLogSnapshot {
        self.activity.snapshot()
    }
}
