use std::sync::Mutex;

use crate::playback::backends::{
    backend_types::PlaybackBackendDescriptor, mpv, native::KivoNativeEngine,
};
use crate::playback::queue::PlaybackQueue;

mod controls;
#[cfg_attr(not(test), allow(dead_code, unused_imports))]
mod diagnostic_query;
mod queue_controls;
mod service;

#[cfg(test)]
mod controls_tests;
#[cfg(test)]
mod diagnostic_query_tests;

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

pub mod queue_state;

#[cfg(test)]
mod tests;
