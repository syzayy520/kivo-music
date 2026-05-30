use std::sync::Mutex;

use super::backends::{backend_types::PlaybackBackendDescriptor, mpv, native::NativeEngine};
use super::engine::PlaybackEngine;
use super::errors::PlaybackResult;
use super::state::PlaybackState;
use super::types::PlaybackTrack;

#[derive(Debug)]
pub struct PlaybackManager {
    primary_engine: NativeEngine,
    compatibility_backends: Vec<PlaybackBackendDescriptor>,
}

impl PlaybackManager {
    pub fn new() -> Self {
        Self {
            primary_engine: NativeEngine::new(),
            compatibility_backends: vec![mpv::descriptor()],
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
