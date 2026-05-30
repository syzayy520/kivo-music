use super::backend_types::{PlaybackBackendDescriptor, PlaybackBackendKind};
use super::super::capabilities::PlaybackCapabilities;
use super::super::engine::PlaybackEngine;
use super::super::errors::{PlaybackError, PlaybackResult};
use super::super::state::PlaybackState;
use super::super::types::PlaybackTrack;

#[derive(Clone, Debug)]
enum KivoRuntimePhase {
    Idle,
    Prepared,
    Active,
    Paused,
    Stopped,
}

impl Default for KivoRuntimePhase {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Clone, Debug, Default)]
struct KivoRuntimeState {
    phase: KivoRuntimePhase,
    loaded_path: Option<String>,
}

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
    state: PlaybackState,
    runtime: KivoRuntimeState,
}

impl KivoNativeEngine {
    pub fn new() -> Self {
        Self {
            descriptor: descriptor(),
            state: PlaybackState::default(),
            runtime: KivoRuntimeState::default(),
        }
    }

    fn unsupported(&mut self, operation: &str) -> PlaybackResult<PlaybackState> {
        let message = format!("kivo core audio {operation} is not implemented yet");
        self.state.error = Some(message.clone());
        Err(PlaybackError::UnsupportedOperation(message))
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
        self.runtime.phase = KivoRuntimePhase::Prepared;
        self.runtime.loaded_path = Some(track.source_path.clone());
        self.state.current_track = Some(track);
        self.unsupported("load")
    }

    fn play(&mut self) -> PlaybackResult<PlaybackState> {
        self.runtime.phase = KivoRuntimePhase::Active;
        self.unsupported("play")
    }

    fn pause(&mut self) -> PlaybackResult<PlaybackState> {
        self.runtime.phase = KivoRuntimePhase::Paused;
        self.unsupported("pause")
    }

    fn resume(&mut self) -> PlaybackResult<PlaybackState> {
        self.runtime.phase = KivoRuntimePhase::Active;
        self.unsupported("resume")
    }

    fn stop(&mut self) -> PlaybackResult<PlaybackState> {
        self.runtime.phase = KivoRuntimePhase::Stopped;
        self.unsupported("stop")
    }

    fn seek(&mut self, _position_ms: u64) -> PlaybackResult<PlaybackState> {
        self.unsupported("seek")
    }

    fn set_volume(&mut self, _level: f32) -> PlaybackResult<PlaybackState> {
        self.unsupported("set volume")
    }

    fn set_muted(&mut self, _muted: bool) -> PlaybackResult<PlaybackState> {
        self.unsupported("set muted")
    }

    fn current_state(&self) -> PlaybackState {
        self.state.clone()
    }

    fn shutdown(&mut self) -> PlaybackResult<()> {
        self.runtime.phase = KivoRuntimePhase::Stopped;
        Ok(())
    }
}
