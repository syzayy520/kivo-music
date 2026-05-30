use std::sync::Mutex;

use super::backends::{backend_types::PlaybackBackendDescriptor, mpv, native::NativeEngine};
use super::engine::PlaybackEngine;
use super::state::PlaybackState;

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
}
