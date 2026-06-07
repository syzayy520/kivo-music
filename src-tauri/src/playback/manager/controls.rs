use super::PlaybackManager;
use crate::playback::backends::backend_types::PlaybackBackendDescriptor;
use crate::playback::engine::PlaybackEngine;
use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::state::PlaybackState;
use crate::playback::types::PlaybackTrack;

impl PlaybackManager {
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

    pub fn seek(&mut self, _position_ms: u64) -> PlaybackResult<PlaybackState> {
        Err(PlaybackError::UnsupportedOperation(
            "native playback seek".to_string(),
        ))
    }

    pub fn set_volume(&mut self, level: f32) -> PlaybackResult<PlaybackState> {
        self.primary_engine.set_volume(level)
    }

    pub fn set_muted(&mut self, muted: bool) -> PlaybackResult<PlaybackState> {
        self.primary_engine.set_muted(muted)
    }
}
