use std::sync::Mutex;

use super::backends::{backend_types::PlaybackBackendDescriptor, mpv, native::KivoNativeEngine};
use super::engine::PlaybackEngine;
use super::errors::PlaybackResult;
use super::manager_queue;
use super::queue::PlaybackQueue;
use super::state::PlaybackState;
use super::types::{PlaybackTrack, RepeatMode};

#[derive(Debug)]
pub struct PlaybackManager {
    primary_engine: KivoNativeEngine,
    compatibility_backends: Vec<PlaybackBackendDescriptor>,
    queue: PlaybackQueue,
}

impl PlaybackManager {
    pub fn new() -> Self {
        Self {
            primary_engine: KivoNativeEngine::new(),
            compatibility_backends: vec![mpv::descriptor()],
            queue: PlaybackQueue::default(),
        }
    }

    pub fn current_state(&self) -> PlaybackState {
        self.primary_engine.current_state()
    }

    pub fn primary_backend(&self) -> PlaybackBackendDescriptor {
        self.primary_engine.descriptor()
    }

    pub fn compatibility_backends(&self) -> Vec<PlaybackBackendDescriptor> {
        self.compatibility_backends.clone()
    }

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

    pub fn load(&mut self, track: PlaybackTrack) -> PlaybackResult<PlaybackState> {
        self.primary_engine.load(track)
    }

    pub fn play(&mut self) -> PlaybackResult<PlaybackState> {
        self.primary_engine.play()
    }

    pub fn pause(&mut self) -> PlaybackResult<PlaybackState> {
        self.primary_engine.pause()
    }

    pub fn resume(&mut self) -> PlaybackResult<PlaybackState> {
        self.primary_engine.resume()
    }

    pub fn stop(&mut self) -> PlaybackResult<PlaybackState> {
        self.primary_engine.stop()
    }

    pub fn seek(&mut self, position_ms: u64) -> PlaybackResult<PlaybackState> {
        self.primary_engine.seek(position_ms)
    }

    pub fn set_volume(&mut self, level: f32) -> PlaybackResult<PlaybackState> {
        self.primary_engine.set_volume(level)
    }

    pub fn set_muted(&mut self, muted: bool) -> PlaybackResult<PlaybackState> {
        self.primary_engine.set_muted(muted)
    }
}

impl Default for PlaybackManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct PlaybackManagerState {
    manager: Mutex<PlaybackManager>,
}

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
