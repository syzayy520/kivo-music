use super::super::errors::{PlaybackError, PlaybackResult};
use super::super::output::OutputRuntimeStatus;
use super::super::types::{PlaybackStatus, PlaybackTrack};

#[derive(Clone, Debug)]
pub struct KivoNativeBackend {
    current_track: Option<PlaybackTrack>,
    status: PlaybackStatus,
    output_status: OutputRuntimeStatus,
}

impl KivoNativeBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attach_track(&mut self, track: PlaybackTrack) -> PlaybackStatus {
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

    pub fn output_status(&self) -> OutputRuntimeStatus {
        self.output_status.clone()
    }

    pub fn play(&self) -> PlaybackResult<PlaybackStatus> {
        Self::unsupported("native backend play")
    }

    pub fn pause(&self) -> PlaybackResult<PlaybackStatus> {
        Self::unsupported("native backend pause")
    }

    pub fn resume(&self) -> PlaybackResult<PlaybackStatus> {
        Self::unsupported("native backend resume")
    }

    pub fn stop(&self) -> PlaybackResult<PlaybackStatus> {
        Self::unsupported("native backend stop")
    }

    pub fn seek(&self, _position_ms: u64) -> PlaybackResult<PlaybackStatus> {
        Self::unsupported("native backend seek")
    }

    fn unsupported(operation: &str) -> PlaybackResult<PlaybackStatus> {
        Err(PlaybackError::UnsupportedOperation(format!(
            "{operation} is not implemented yet"
        )))
    }
}

impl Default for KivoNativeBackend {
    fn default() -> Self {
        Self {
            current_track: None,
            status: PlaybackStatus::Idle,
            output_status: OutputRuntimeStatus::default(),
        }
    }
}
