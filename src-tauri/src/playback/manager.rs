use std::sync::Mutex;

use super::backends::{backend_types::PlaybackBackendDescriptor, mpv, native::KivoNativeEngine};
use super::queue::PlaybackQueue;

mod controls;
mod queue_controls;
mod service;

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
