use super::super::errors::PlaybackResult;
use super::super::types::{PlaybackStatus, PlaybackTrack};
use super::native_unsupported::unsupported_operation;

#[derive(Clone, Debug)]
pub struct KivoNativePlayback {
    current_track: Option<PlaybackTrack>,
    status: PlaybackStatus,
}

impl KivoNativePlayback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_track(&mut self, track: PlaybackTrack) -> PlaybackStatus {
        self.current_track = Some(track);
        self.status = PlaybackStatus::Idle;
        self.status.clone()
    }

    pub fn current_track(&self) -> Option<PlaybackTrack> {
        self.current_track.clone()
    }

    pub fn current_status(&self) -> PlaybackStatus {
        self.status.clone()
    }

    pub fn play(&self) -> PlaybackResult<PlaybackStatus> {
        unsupported_operation("native playback play")
    }

    pub fn pause(&self) -> PlaybackResult<PlaybackStatus> {
        unsupported_operation("native playback pause")
    }

    pub fn resume(&self) -> PlaybackResult<PlaybackStatus> {
        unsupported_operation("native playback resume")
    }

    pub fn stop(&self) -> PlaybackResult<PlaybackStatus> {
        unsupported_operation("native playback stop")
    }

    pub fn seek(&self, _position_ms: u64) -> PlaybackResult<PlaybackStatus> {
        unsupported_operation("native playback seek")
    }
}

impl Default for KivoNativePlayback {
    fn default() -> Self {
        Self {
            current_track: None,
            status: PlaybackStatus::Idle,
        }
    }
}
