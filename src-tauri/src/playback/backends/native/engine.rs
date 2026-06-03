use super::{control, load, KivoNativeEngine};
use crate::playback::backends::backend_types::PlaybackBackendDescriptor;
use crate::playback::engine::PlaybackEngine;
use crate::playback::errors::PlaybackResult;
use crate::playback::state::PlaybackState;
use crate::playback::types::PlaybackTrack;

impl PlaybackEngine for KivoNativeEngine {
    fn descriptor(&self) -> PlaybackBackendDescriptor {
        self.descriptor.clone()
    }

    fn load(&mut self, track: PlaybackTrack) -> PlaybackResult<PlaybackState> {
        load::load_track(self, track)
    }

    fn play(&mut self) -> PlaybackResult<PlaybackState> {
        let result = self.playback.play();
        control::apply_status_result(self, result)
    }

    fn pause(&mut self) -> PlaybackResult<PlaybackState> {
        let result = self.playback.pause();
        control::apply_status_result(self, result)
    }

    fn resume(&mut self) -> PlaybackResult<PlaybackState> {
        let result = self.playback.resume();
        control::apply_status_result(self, result)
    }

    fn stop(&mut self) -> PlaybackResult<PlaybackState> {
        let result = self.playback.stop();
        control::apply_status_result(self, result)
    }

    fn seek(&mut self, position_ms: u64) -> PlaybackResult<PlaybackState> {
        control::seek_decoder(self, position_ms);
        let result = self.playback.seek(position_ms);
        control::apply_status_result(self, result)
    }

    fn set_volume(&mut self, level: f32) -> PlaybackResult<PlaybackState> {
        control::set_output_volume(self, level);
        control::unsupported(self, "set volume")
    }

    fn set_muted(&mut self, muted: bool) -> PlaybackResult<PlaybackState> {
        control::set_output_muted(self, muted);
        control::unsupported(self, "set muted")
    }

    fn current_state(&self) -> PlaybackState {
        self.state.clone()
    }

    fn shutdown(&mut self) -> PlaybackResult<()> {
        self.pipeline.shutdown()?;
        let _ = self.playback.stop();
        Ok(())
    }
}
