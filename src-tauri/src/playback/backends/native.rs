use super::super::capabilities::PlaybackCapabilities;
use super::super::decoder_request::AudioDecoderOpenRequest;
use super::super::engine::PlaybackEngine;
use super::super::errors::{PlaybackError, PlaybackResult};
use super::super::native_pipeline::NativePipeline;
#[cfg(test)]
use super::super::native_pipeline::NativePipelineState;
use super::super::state::PlaybackState;
use super::super::types::PlaybackTrack;
use super::backend_types::{PlaybackBackendDescriptor, PlaybackBackendKind};
use super::native_playback::KivoNativePlayback;
use super::native_unsupported::unsupported_operation_message;

pub fn descriptor() -> PlaybackBackendDescriptor {
    PlaybackBackendDescriptor {
        kind: PlaybackBackendKind::Native,
        name: "kivo-core-audio".to_string(),
        capabilities: PlaybackCapabilities::default(),
        is_primary: true,
    }
}

#[derive(Debug)]
pub struct KivoNativeEngine {
    descriptor: PlaybackBackendDescriptor,
    playback: KivoNativePlayback,
    pipeline: NativePipeline,
    state: PlaybackState,
}

impl KivoNativeEngine {
    pub fn new() -> Self {
        Self {
            descriptor: descriptor(),
            playback: KivoNativePlayback::new(),
            pipeline: NativePipeline::new(),
            state: PlaybackState::default(),
        }
    }

    fn unsupported(&mut self, operation: &str) -> PlaybackResult<PlaybackState> {
        let message = unsupported_operation_message(operation);
        self.state.error = Some(message.clone());
        Err(PlaybackError::UnsupportedOperation(message))
    }

    fn load_pipeline_until_output_boundary(&mut self, track: &PlaybackTrack) -> PlaybackResult<()> {
        let request = AudioDecoderOpenRequest::from_track(track);
        self.pipeline.open_decoder(request, 0)?;
        self.pipeline.schedule_decode_step()?;

        match self.pipeline.schedule_output_submit_step() {
            Ok(()) => Ok(()),
            Err(PlaybackError::UnsupportedOperation(_)) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn unsupported_load(
        &mut self,
        pipeline_error: Option<PlaybackError>,
    ) -> PlaybackResult<PlaybackState> {
        let message = unsupported_operation_message("load");
        self.state.error =
            Some(pipeline_error.map_or_else(|| message.clone(), |error| error.to_string()));
        Err(PlaybackError::UnsupportedOperation(message))
    }

    fn record_error(&mut self, error: PlaybackError) -> PlaybackResult<PlaybackState> {
        self.state.error = Some(error.to_string());
        Err(error)
    }

    #[cfg(test)]
    pub fn pipeline_state(&self) -> NativePipelineState {
        self.pipeline.state()
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
        let pipeline_error = self.load_pipeline_until_output_boundary(&track).err();
        self.playback.load_track(track);
        self.state.current_track = self.playback.current_track();
        self.state.status = self.playback.current_status();
        self.unsupported_load(pipeline_error)
    }

    fn play(&mut self) -> PlaybackResult<PlaybackState> {
        match self.playback.play() {
            Ok(status) => {
                self.state.status = status;
                Ok(self.state.clone())
            }
            Err(error) => self.record_error(error),
        }
    }

    fn pause(&mut self) -> PlaybackResult<PlaybackState> {
        match self.playback.pause() {
            Ok(status) => {
                self.state.status = status;
                Ok(self.state.clone())
            }
            Err(error) => self.record_error(error),
        }
    }

    fn resume(&mut self) -> PlaybackResult<PlaybackState> {
        match self.playback.resume() {
            Ok(status) => {
                self.state.status = status;
                Ok(self.state.clone())
            }
            Err(error) => self.record_error(error),
        }
    }

    fn stop(&mut self) -> PlaybackResult<PlaybackState> {
        match self.playback.stop() {
            Ok(status) => {
                self.state.status = status;
                Ok(self.state.clone())
            }
            Err(error) => self.record_error(error),
        }
    }

    fn seek(&mut self, position_ms: u64) -> PlaybackResult<PlaybackState> {
        let _ = self.pipeline.seek_decoder(position_ms);
        match self.playback.seek(position_ms) {
            Ok(status) => {
                self.state.status = status;
                Ok(self.state.clone())
            }
            Err(error) => self.record_error(error),
        }
    }

    fn set_volume(&mut self, level: f32) -> PlaybackResult<PlaybackState> {
        self.state.volume.level = level.clamp(0.0, 1.0);
        self.unsupported("set volume")
    }

    fn set_muted(&mut self, muted: bool) -> PlaybackResult<PlaybackState> {
        self.state.volume.muted = muted;
        self.unsupported("set muted")
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
