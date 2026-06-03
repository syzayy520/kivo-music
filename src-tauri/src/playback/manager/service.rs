use super::PlaybackManagerState;
use crate::playback::backends::backend_types::PlaybackBackendDescriptor;
use crate::playback::errors::PlaybackResult;
use crate::playback::queue::PlaybackQueue;
use crate::playback::state::PlaybackState;
use crate::playback::types::{PlaybackTrack, RepeatMode};

impl PlaybackManagerState {
    pub fn current_state(&self) -> PlaybackState {
        let manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.current_state()
    }

    pub fn primary_backend(&self) -> PlaybackBackendDescriptor {
        let manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.primary_backend()
    }

    pub fn compatibility_backends(&self) -> Vec<PlaybackBackendDescriptor> {
        let manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.compatibility_backends()
    }

    pub fn queue(&self) -> PlaybackQueue {
        let manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.queue()
    }

    pub fn queue_append(&self, track: PlaybackTrack) -> PlaybackQueue {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.queue_append(track)
    }

    pub fn queue_remove(&self, index: usize) -> PlaybackResult<PlaybackQueue> {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.queue_remove(index)
    }

    pub fn queue_set_repeat_mode(&self, repeat_mode: RepeatMode) -> PlaybackQueue {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.queue_set_repeat_mode(repeat_mode)
    }

    pub fn queue_set_shuffle(&self, shuffle: bool) -> PlaybackQueue {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.queue_set_shuffle(shuffle)
    }

    pub fn queue_set_current(&self, index: usize) -> PlaybackResult<PlaybackState> {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.queue_set_current(index)
    }

    pub fn queue_next(&self) -> PlaybackResult<PlaybackState> {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.queue_next()
    }

    pub fn queue_previous(&self) -> PlaybackResult<PlaybackState> {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.queue_previous()
    }

    pub fn load(&self, track: PlaybackTrack) -> PlaybackResult<PlaybackState> {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.load(track)
    }

    pub fn play(&self) -> PlaybackResult<PlaybackState> {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.play()
    }

    pub fn pause(&self) -> PlaybackResult<PlaybackState> {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.pause()
    }

    pub fn resume(&self) -> PlaybackResult<PlaybackState> {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.resume()
    }

    pub fn stop(&self) -> PlaybackResult<PlaybackState> {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.stop()
    }

    pub fn seek(&self, position_ms: u64) -> PlaybackResult<PlaybackState> {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.seek(position_ms)
    }

    pub fn set_volume(&self, level: f32) -> PlaybackResult<PlaybackState> {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.set_volume(level)
    }

    pub fn set_muted(&self, muted: bool) -> PlaybackResult<PlaybackState> {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        manager.set_muted(muted)
    }
}
