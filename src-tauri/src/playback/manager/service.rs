use super::PlaybackManagerState;
use crate::playback::backends::backend_types::PlaybackBackendDescriptor;
use crate::playback::errors::PlaybackResult;
use crate::playback::queue::PlaybackQueue;
use crate::playback::state::PlaybackState;
use crate::playback::types::{PlaybackTrack, RepeatMode};

impl PlaybackManagerState {
    fn with_manager<T>(&self, read: impl FnOnce(&super::PlaybackManager) -> T) -> T {
        let manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        read(&manager)
    }

    fn with_manager_mut<T>(&self, write: impl FnOnce(&mut super::PlaybackManager) -> T) -> T {
        let mut manager = self
            .manager
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        write(&mut manager)
    }

    pub fn current_state(&self) -> PlaybackState {
        self.with_manager(super::PlaybackManager::current_state)
    }

    pub fn primary_backend(&self) -> PlaybackBackendDescriptor {
        self.with_manager(super::PlaybackManager::primary_backend)
    }

    pub fn compatibility_backends(&self) -> Vec<PlaybackBackendDescriptor> {
        self.with_manager(super::PlaybackManager::compatibility_backends)
    }

    pub fn queue(&self) -> PlaybackQueue {
        self.with_manager(super::PlaybackManager::queue)
    }

    pub fn queue_append(&self, track: PlaybackTrack) -> PlaybackQueue {
        self.with_manager_mut(move |manager| manager.queue_append(track))
    }

    pub fn queue_remove(&self, index: usize) -> PlaybackResult<PlaybackQueue> {
        self.with_manager_mut(|manager| manager.queue_remove(index))
    }

    pub fn queue_set_repeat_mode(&self, repeat_mode: RepeatMode) -> PlaybackQueue {
        self.with_manager_mut(|manager| manager.queue_set_repeat_mode(repeat_mode))
    }

    pub fn queue_set_shuffle(&self, shuffle: bool) -> PlaybackQueue {
        self.with_manager_mut(|manager| manager.queue_set_shuffle(shuffle))
    }

    pub fn queue_set_current(&self, index: usize) -> PlaybackResult<PlaybackState> {
        self.with_manager_mut(|manager| manager.queue_set_current(index))
    }

    pub fn queue_next(&self) -> PlaybackResult<PlaybackState> {
        self.with_manager_mut(super::PlaybackManager::queue_next)
    }

    pub fn queue_previous(&self) -> PlaybackResult<PlaybackState> {
        self.with_manager_mut(super::PlaybackManager::queue_previous)
    }

    pub fn load(&self, track: PlaybackTrack) -> PlaybackResult<PlaybackState> {
        self.with_manager_mut(move |manager| manager.load(track))
    }

    pub fn play(&self) -> PlaybackResult<PlaybackState> {
        self.with_manager_mut(super::PlaybackManager::play)
    }

    pub fn pause(&self) -> PlaybackResult<PlaybackState> {
        self.with_manager_mut(super::PlaybackManager::pause)
    }

    pub fn resume(&self) -> PlaybackResult<PlaybackState> {
        self.with_manager_mut(super::PlaybackManager::resume)
    }

    pub fn stop(&self) -> PlaybackResult<PlaybackState> {
        self.with_manager_mut(super::PlaybackManager::stop)
    }

    pub fn seek(&self, position_ms: u64) -> PlaybackResult<PlaybackState> {
        self.with_manager_mut(|manager| manager.seek(position_ms))
    }

    pub fn set_volume(&self, level: f32) -> PlaybackResult<PlaybackState> {
        self.with_manager_mut(|manager| manager.set_volume(level))
    }

    pub fn set_muted(&self, muted: bool) -> PlaybackResult<PlaybackState> {
        self.with_manager_mut(|manager| manager.set_muted(muted))
    }
}
