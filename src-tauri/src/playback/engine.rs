use super::backends::backend_types::PlaybackBackendDescriptor;
use super::errors::PlaybackResult;
use super::state::PlaybackState;

pub trait PlaybackEngine {
    fn descriptor(&self) -> PlaybackBackendDescriptor;
    fn current_state(&self) -> PlaybackState;
    fn shutdown(&mut self) -> PlaybackResult<()>;
}
