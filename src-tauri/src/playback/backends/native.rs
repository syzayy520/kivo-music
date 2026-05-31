use super::backend_types::{PlaybackBackendDescriptor, PlaybackBackendKind};
use super::native_playback::KivoNativePlayback;
use super::native_unsupported::unsupported_operation_message;
use super::super::capabilities::PlaybackCapabilities;
use super::super::engine::PlaybackEngine;
use super::super::errors::{PlaybackError, PlaybackResult};
use super::super::state::PlaybackState;
use super::super::types::PlaybackTrack;

pub fn descriptor() -> PlaybackBackendDescriptor {
    PlaybackBackendDescriptor {
        kind: PlaybackBackendKind::Native,
        name: "kivo-core-audio".to_string(),
        capabilities: PlaybackCapabilities::default(),
        is_primary: true,
    }
}

#[derive(Clone, Debug)]
pub struct KivoNativeEngine {
    descriptor: PlaybackBackendDescriptor,
    playback: KivoNativePlayback,
    state: PlaybackState,
}

impl KivoNativeEngine {
    pub fn new() -> Self {
        Self {
            descriptor: descriptor(),
            playback: KivoNativePlayback::new(),
            state: PlaybackState::default(),
        }
    }

    fn unsupported(&mut self, operation: &str) -> PlaybackResult<PlaybackState> {
        let message = unsupported_operation_message(operation);
        self.state.error = Some(message.clone());
        Err(PlaybackError::UnsupportedOperation(message))
    }

    fn record_error(&mut self, error: PlaybackError) -> PlaybackResult<PlaybackState> {
        self.state.error = Some(error.to_string());
        Err(error)
    }
}

impl Default for KivoNativeEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PlaybackEngine for KivoNativeEngine {
    fn descriptor(&self) -> PlaybackBackendDescriptor {
        self.descriptor.clone()
    }

    fn load(&mut self, track: PlaybackTrack) -> PlaybackResult<PlaybackState> {
        self.playback.load_track(track);
        self.state.current_track = self.playback.current_track();
        self.state.status = self.playback.current_status();
        self.unsupported("load")
    }

    fn play(&mut self) -> PlaybackResult<PlaybackState> {
        match self.playback.play() {
            Ok(status) => {
                self.state.status = status;
                Ok(self.state.clone())
            }
            Err(error) => self.record_error(error),
        }
    }

    fn pause(&mut self) -> PlaybackResult<PlaybackState> {
        match self.playback.pause() {
            Ok(status) => {
                self.state.status = status;
                Ok(self.state.clone())
            }
            Err(error) => self.record_error(error),
        }
    }

    fn resume(&mut self) -> PlaybackResult<PlaybackState> {
        match self.playback.resume() {
            Ok(status) => {
                self.state.status = status;
                Ok(self.state.clone())
            }
            Err(error) => self.record_error(error),
        }
    }

    fn stop(&mut self) -> PlaybackResult<PlaybackState> {
        match self.playback.stop() {
            Ok(status) => {
                self.state.status = status;
                Ok(self.state.clone())
            }
            Err(error) => self.record_error(error),
        }
    }

    fn seek(&mut self, position_ms: u64) -> PlaybackResult<PlaybackState> {
        match self.playback.seek(position_ms) {
            Ok(status) => {
                self.state.status = status;
                Ok(self.state.clone())
            }
            Err(error) => self.record_error(error),
        }
    }

    fn set_volume(&mut self, level: f32) -> PlaybackResult<PlaybackState> {
        self.state.volume.level = level.clamp(0.0, 1.0);
        self.unsupported("set volume")
    }

    fn set_muted(&mut self, muted: bool) -> PlaybackResult<PlaybackState> {
        self.state.volume.muted = muted;
        self.unsupported("set muted")
    }

    fn current_state(&self) -> PlaybackState {
        self.state.clone()
    }

    fn shutdown(&mut self) -> PlaybackResult<()> {
        let _ = self.playback.stop();
        Ok(())
    }
}
