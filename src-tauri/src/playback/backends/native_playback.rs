use super::super::errors::{PlaybackError, PlaybackResult};
use super::super::types::{PlaybackStatus, PlaybackTrack};
use super::native_backend::KivoNativeBackend;

#[derive(Clone, Debug)]
pub struct KivoNativePlayback {
    backend: KivoNativeBackend,
    status: PlaybackStatus,
}

impl KivoNativePlayback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_track(&mut self, track: PlaybackTrack) -> PlaybackStatus {
        self.status = self.backend.attach_track(track);
        self.status.clone()
    }

    pub fn current_track(&self) -> Option<PlaybackTrack> {
        self.backend.current_track()
    }

    pub fn current_status(&self) -> PlaybackStatus {
        self.status.clone()
    }

    pub fn play(&self) -> PlaybackResult<PlaybackStatus> {
        Self::unsupported("native playback play")
    }

    pub fn pause(&self) -> PlaybackResult<PlaybackStatus> {
        Self::unsupported("native playback pause")
    }

    pub fn resume(&self) -> PlaybackResult<PlaybackStatus> {
        Self::unsupported("native playback resume")
    }

    pub fn stop(&self) -> PlaybackResult<PlaybackStatus> {
        Self::unsupported("native playback stop")
    }

    pub fn seek(&self, _position_ms: u64) -> PlaybackResult<PlaybackStatus> {
        Self::unsupported("native playback seek")
    }

    fn unsupported(operation: &str) -> PlaybackResult<PlaybackStatus> {
        Err(PlaybackError::UnsupportedOperation(format!(
            "{operation} is not implemented yet"
        )))
    }
}

impl Default for KivoNativePlayback {
    fn default() -> Self {
        Self {
            backend: KivoNativeBackend::new(),
            status: PlaybackStatus::Idle,
        }
    }
}
