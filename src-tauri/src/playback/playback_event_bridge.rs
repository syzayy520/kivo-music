use super::activity_log::PlaybackActivityLogState;
use super::errors::PlaybackResult;
use super::playback_event_bus::PlaybackEventBus;
use super::state::PlaybackState;

#[derive(Debug, Default)]
pub struct PlaybackEventBridge {
    bus: PlaybackEventBus,
}

impl PlaybackEventBridge {
    pub fn record_state_result(
        &mut self,
        activity: &PlaybackActivityLogState,
        result: &PlaybackResult<PlaybackState>,
    ) {
        self.bus.emit_state_result(result);
        self.flush_into(activity);
    }

    pub fn record_track_result(
        &mut self,
        activity: &PlaybackActivityLogState,
        result: &PlaybackResult<PlaybackState>,
    ) {
        self.bus.emit_track_result(result);
        self.flush_into(activity);
    }

    fn flush_into(&mut self, activity: &PlaybackActivityLogState) {
        for event in self.bus.drain_events() {
            activity.append(event);
        }
    }
}
