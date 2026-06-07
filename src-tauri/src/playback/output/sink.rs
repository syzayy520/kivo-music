use crate::playback::errors::PlaybackResult;

use super::frame::AudioOutputFrame;
use super::settings::OutputSettings;
use super::status::OutputRuntimeStatus;

pub trait OutputSink {
    fn open(&mut self, settings: &OutputSettings) -> PlaybackResult<OutputRuntimeStatus>;
    fn submit_frame(&mut self, frame: AudioOutputFrame) -> PlaybackResult<OutputRuntimeStatus>;
    fn pause(&mut self) -> PlaybackResult<OutputRuntimeStatus>;
    fn resume(&mut self) -> PlaybackResult<OutputRuntimeStatus>;
    fn flush(&mut self) -> PlaybackResult<OutputRuntimeStatus>;
    fn stop(&mut self) -> PlaybackResult<OutputRuntimeStatus>;
    fn set_volume(&mut self, level: f32) -> PlaybackResult<OutputRuntimeStatus>;
    fn set_muted(&mut self, muted: bool) -> PlaybackResult<OutputRuntimeStatus>;
    fn status(&self) -> OutputRuntimeStatus;
    fn close(&mut self) -> PlaybackResult<()>;
}
