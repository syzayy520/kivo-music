use super::backends::mpv;
use super::backends::backend_types::PlaybackBackendDescriptor;
use super::errors::PlaybackResult;

#[derive(Clone, Debug)]
pub struct PlaybackLifecycle {
    pub backend: PlaybackBackendDescriptor,
}

impl PlaybackLifecycle {
    pub fn initialize() -> Self {
        Self {
            backend: mpv::descriptor(),
        }
    }

    pub fn shutdown(self) -> PlaybackResult<()> {
        Ok(())
    }
}
