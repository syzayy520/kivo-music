use super::backends::backend_types::PlaybackBackendDescriptor;
use super::errors::PlaybackResult;
use super::state::PlaybackState;
use super::types::PlaybackTrack;

pub trait PlaybackEngine {
    fn descriptor(&self) -> PlaybackBackendDescriptor;
    fn load(&mut self, track: PlaybackTrack) -> PlaybackResult<PlaybackState>;
    fn play(&mut self) -> PlaybackResult<PlaybackState>;
    fn pause(&mut self) -> PlaybackResult<PlaybackState>;
    fn resume(&mut self) -> PlaybackResult<PlaybackState>;
    fn stop(&mut self) -> PlaybackResult<PlaybackState>;
    fn seek(&mut self, position_ms: u64) -> PlaybackResult<PlaybackState>;
    fn set_volume(&mut self, level: f32) -> PlaybackResult<PlaybackState>;
    fn set_muted(&mut self, muted: bool) -> PlaybackResult<PlaybackState>;
    fn current_state(&self) -> PlaybackState;
    fn shutdown(&mut self) -> PlaybackResult<()>;
}
