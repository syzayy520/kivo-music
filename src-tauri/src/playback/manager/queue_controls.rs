use super::PlaybackManager;
use crate::playback::errors::PlaybackResult;
use crate::playback::manager_queue;
use crate::playback::queue::PlaybackQueue;
use crate::playback::state::PlaybackState;
use crate::playback::types::{PlaybackTrack, RepeatMode};

impl PlaybackManager {
    pub fn queue(&self) -> PlaybackQueue {
        self.queue.clone()
    }

    pub fn queue_append(&mut self, track: PlaybackTrack) -> PlaybackQueue {
        manager_queue::append(&mut self.queue, track)
    }

    pub fn queue_remove(&mut self, index: usize) -> PlaybackResult<PlaybackQueue> {
        manager_queue::remove(&mut self.queue, index)
    }

    pub fn queue_set_repeat_mode(&mut self, repeat_mode: RepeatMode) -> PlaybackQueue {
        manager_queue::set_repeat_mode(&mut self.queue, repeat_mode)
    }

    pub fn queue_set_shuffle(&mut self, shuffle: bool) -> PlaybackQueue {
        manager_queue::set_shuffle(&mut self.queue, shuffle)
    }

    pub fn queue_set_current(&mut self, index: usize) -> PlaybackResult<PlaybackState> {
        let track = manager_queue::select_track(&mut self.queue, index)?;
        self.load(track)
    }

    pub fn queue_next(&mut self) -> PlaybackResult<PlaybackState> {
        let track = manager_queue::select_next_track(&mut self.queue)?;
        self.load(track)
    }

    pub fn queue_previous(&mut self) -> PlaybackResult<PlaybackState> {
        let track = manager_queue::select_previous_track(&mut self.queue)?;
        self.load(track)
    }
}
