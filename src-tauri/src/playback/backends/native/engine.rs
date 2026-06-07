use super::{control, load, mute, pause, resume, stop, tap_diagnostic, volume, KivoNativeEngine};
use crate::playback::backends::backend_types::PlaybackBackendDescriptor;
use crate::playback::engine::PlaybackEngine;
use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::native_pipeline_loop::NativePipelineLoopStep;
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
        if self.state.current_track.is_none() {
            let result = self.playback.play();
            return control::apply_status_result(self, result);
        }

        match self.pipeline.pump_once() {
            Ok(NativePipelineLoopStep::Drained) => {
                self.playback.mark_playing();
                self.state.status = self.playback.current_status();
                self.state.error = None;
                Ok(self.state.clone())
            }
            Ok(NativePipelineLoopStep::EndOfStream) => control::record_error(
                self,
                PlaybackError::Backend(
                    "native pipeline reached end-of-stream before NullOutput submit".to_string(),
                ),
            ),
            Err(error) => control::record_error(self, error),
        }
    }

    fn pause(&mut self) -> PlaybackResult<PlaybackState> {
        pause::pause_track(self)
    }

    fn resume(&mut self) -> PlaybackResult<PlaybackState> {
        resume::resume_track(self)
    }

    fn stop(&mut self) -> PlaybackResult<PlaybackState> {
        stop::stop_track(self)
    }

    fn seek(&mut self, position_ms: u64) -> PlaybackResult<PlaybackState> {
        control::seek_decoder(self, position_ms);
        let result = self.playback.seek(position_ms);
        control::apply_status_result(self, result)
    }

    fn set_volume(&mut self, level: f32) -> PlaybackResult<PlaybackState> {
        volume::set_volume(self, level)
    }

    fn set_muted(&mut self, muted: bool) -> PlaybackResult<PlaybackState> {
        mute::set_muted(self, muted)
    }

    fn current_state(&self) -> PlaybackState {
        self.state.clone()
    }

    fn shutdown(&mut self) -> PlaybackResult<()> {
        tap_diagnostic::close_on_shutdown(&mut self.tap_diagnostic, &mut self.pipeline);
        self.pipeline.shutdown()?;
        let _ = self.playback.stop();
        Ok(())
    }
}
